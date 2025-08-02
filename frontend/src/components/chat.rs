use yew::prelude::*;
use web_sys::HtmlTextAreaElement;
use wasm_bindgen::JsCast;
use chrono::Utc;
use shared::ChatMessage;

use crate::app::AppState;
use crate::i18n::I18nContext;
use crate::components::{
    message::MessageBubble,
    welcome::WelcomeMessage,
};

#[function_component(ChatInterface)]
pub fn chat_interface() -> Html {
    let state = use_context::<UseStateHandle<AppState>>().expect("AppState not found");
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");
    
    let input_ref = use_node_ref();
    let messages_container_ref = use_node_ref();
    
    let input_value = use_state(|| String::new());
    
    // Auto-scroll to bottom when new messages arrive
    {
        let messages_container_ref = messages_container_ref.clone();
        let messages_len = state.messages.len();
        
        use_effect_with(messages_len, move |_| {
            if let Some(container) = messages_container_ref.cast::<web_sys::Element>() {
                container.set_scroll_top(container.scroll_height());
            }
        });
    }
    
    let send_message = {
        let state = state.clone();
        let input_value = input_value.clone();
        let input_ref = input_ref.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let message_text = (*input_value).clone().trim().to_string();
            
            if !message_text.is_empty() {
                // Add user message
                let user_message = ChatMessage {
                    role: "user".to_string(),
                    content: message_text.clone(),
                    timestamp: Utc::now(),
                };
                
                let mut new_state = (*state).clone();
                new_state.messages.push(user_message);
                
                // Add a bot response (placeholder for now)
                let bot_message = ChatMessage {
                    role: "assistant".to_string(),
                    content: format!("ขอบคุณสำหรับข้อความ: \"{}\" - ระบบ AI กำลังพัฒนาและจะตอบกลับเร็วๆ นี้", message_text),
                    timestamp: Utc::now(),
                };
                new_state.messages.push(bot_message);
                
                state.set(new_state);
                
                // Clear input
                input_value.set(String::new());
                if let Some(input) = input_ref.cast::<HtmlTextAreaElement>() {
                    input.set_value("");
                }
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
                // Trigger the send message directly
                let event = web_sys::Event::new("submit").unwrap();
                send_message.emit(SubmitEvent::from(event.unchecked_into::<web_sys::SubmitEvent>()));
            }
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
                                {state.messages.iter().enumerate().map(|(i, message)| {
                                    let is_bot = message.role == "assistant" || message.role == "system";
                                    html! {
                                        <MessageBubble
                                            key={format!("{}-{}", i, message.role)}
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
                
                <div class="chat-input-section">
                    {if let Some(error) = &state.error_message {
                        html! {
                            <div class="error-banner">
                                <span class="error-icon">{"⚠️"}</span>
                                <span class="error-text">{error}</span>
                                <button 
                                    class="error-close"
                                    onclick={
                                        let state = state.clone();
                                        Callback::from(move |_: MouseEvent| {
                                            let mut new_state = (*state).clone();
                                            new_state.error_message = None;
                                            state.set(new_state);
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
                                    class="btn btn-icon"
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