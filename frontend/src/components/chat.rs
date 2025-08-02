use yew::prelude::*;
use yewdux::prelude::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use chrono::Utc;
use shared::{ChatMessage, Language};

use crate::store::{AppState, AppAction};
use crate::i18n::I18nContext;
use crate::components::{
    message::MessageBubble,
    welcome::WelcomeMessage,
    image::ImageUpload,
};
use crate::services::api::ApiService;

#[function_component(ChatInterface)]
pub fn chat_interface() -> Html {
    let (state, dispatch) = use_store::<AppState>();
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");
    
    let input_ref = use_node_ref();
    let messages_container_ref = use_node_ref();
    
    let input_value = use_state(|| String::new());
    let show_image_upload = use_state(|| false);
    
    // Auto-scroll to bottom when new messages arrive
    {
        let messages_container_ref = messages_container_ref.clone();
        let messages_len = state.messages.len();
        
        use_effect_with_deps(move |_| {
            if let Some(container) = messages_container_ref.cast::<web_sys::Element>() {
                container.set_scroll_top(container.scroll_height());
            }
        }, messages_len);
    }
    
    let send_message = {
        let dispatch = dispatch.clone();
        let input_value = input_value.clone();
        let input_ref = input_ref.clone();
        
        Callback::from(move |_: SubmitEvent| {
            let message_text = (*input_value).clone().trim().to_string();
            
            if !message_text.is_empty() {
                // Add user message
                let user_message = ChatMessage {
                    role: "user".to_string(),
                    content: message_text.clone(),
                    timestamp: Utc::now(),
                };
                dispatch.apply(AppAction::AddMessage(user_message));
                
                // Clear input
                input_value.set(String::new());
                if let Some(input) = input_ref.cast::<HtmlTextAreaElement>() {
                    input.set_value("");
                }
                
                // Send to API and get response
                let dispatch_clone = dispatch.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    dispatch_clone.apply(AppAction::SetLoading(true));
                    
                    match ApiService::send_chat_message(&message_text, &state.conversation_id).await {
                        Ok(response) => {
                            let bot_message = ChatMessage {
                                role: "assistant".to_string(),
                                content: response,
                                timestamp: Utc::now(),
                            };
                            dispatch_clone.apply(AppAction::AddMessage(bot_message));
                        }
                        Err(err) => {
                            let error_message = ChatMessage {
                                role: "assistant".to_string(),
                                content: format!("เกิดข้อผิดพลาด: {}", err),
                                timestamp: Utc::now(),
                            };
                            dispatch_clone.apply(AppAction::AddMessage(error_message));
                        }
                    }
                    
                    dispatch_clone.apply(AppAction::SetLoading(false));
                });
            }
        })
    };
    
    let on_input_change = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };
    
    let on_key_down = {
        let send_message = send_message.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" && !e.shift_key() {
                e.prevent_default();
                send_message.emit(SubmitEvent::new("submit").unwrap());
            }
        })
    };
    
    let toggle_image_upload = {
        let show_image_upload = show_image_upload.clone();
        Callback::from(move |_: MouseEvent| {
            show_image_upload.set(!*show_image_upload);
        })
    };
    
    let on_image_uploaded = {
        let show_image_upload = show_image_upload.clone();
        Callback::from(move |_| {
            show_image_upload.set(false);
        })
    };
    
    html! {
        <div class="chat-interface">
            <div class="chat-container">
                <div class="messages-container" ref={messages_container_ref}>
                    {if state.messages.is_empty() {
                        html! { <WelcomeMessage /> }
                    } else {
                        html! {
                            <div class="messages-list">
                                {state.messages.iter().map(|message| {
                                    let is_bot = message.role == "assistant" || message.role == "system";
                                    html! {
                                        <MessageBubble
                                            key={format!("{}-{}", message.timestamp, message.role)}
                                            message={message.clone()}
                                            is_bot={is_bot}
                                        />
                                    }
                                }).collect::<Html>()}
                                
                                {if state.is_loading {
                                    html! {
                                        <div class="message message-bot loading-message">
                                            <div class="message-avatar">
                                                <span class="avatar-bot">{"🤖"}</span>
                                            </div>
                                            <div class="message-content">
                                                <div class="message-bubble">
                                                    <div class="typing-indicator">
                                                        <span></span>
                                                        <span></span>
                                                        <span></span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                        }
                    }}
                </div>
                
                {if *show_image_upload {
                    html! {
                        <div class="image-upload-section">
                            <ImageUpload on_uploaded={on_image_uploaded} />
                        </div>
                    }
                } else {
                    html! {}
                }}
                
                <div class="chat-input-section">
                    {if let Some(error) = &state.error_message {
                        html! {
                            <div class="error-banner">
                                <span class="error-icon">{"⚠️"}</span>
                                <span class="error-text">{error}</span>
                                <button 
                                    class="error-close"
                                    onclick={
                                        let dispatch = dispatch.clone();
                                        Callback::from(move |_: MouseEvent| {
                                            dispatch.apply(AppAction::SetError(None));
                                        })
                                    }
                                >
                                    {"×"}
                                </button>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                    
                    <form class="chat-input-form" onsubmit={send_message}>
                        <div class="input-container">
                            <div class="input-actions-left">
                                <button
                                    type="button"
                                    class={classes!("btn", "btn-icon", if *show_image_upload { "active" } else { "" })}
                                    onclick={toggle_image_upload}
                                    title={i18n.t("chat.upload_image")}
                                >
                                    {"📷"}
                                </button>
                            </div>
                            
                            <textarea
                                ref={input_ref}
                                class="chat-input"
                                placeholder={i18n.t("chat.placeholder")}
                                value={(*input_value).clone()}
                                oninput={on_input_change}
                                onkeydown={on_key_down}
                                rows="1"
                                disabled={state.is_loading}
                            />
                            
                            <div class="input-actions-right">
                                <button
                                    type="submit"
                                    class="btn btn-primary btn-send"
                                    disabled={input_value.trim().is_empty() || state.is_loading}
                                    title={i18n.t("chat.send")}
                                >
                                    {if state.is_loading {
                                        html! { <div class="spinner-sm"></div> }
                                    } else {
                                        html! { <span class="send-icon">{"➤"}</span> }
                                    }}
                                </button>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}