//! Chat Window Component
//! 
//! This module provides the main chat interface for the AI4Thai Crop Guardian
//! application, supporting multimodal conversations with AI agricultural advisors.

use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::types::{ChatMessage, ChatRole, Language};
use crate::i18n::I18nContext;
use crate::components::ui::{GradientButton, StatusCard};
use crate::services::api::ApiService;
use uuid::Uuid;
use chrono::Utc;

/// Properties for the chat window component
#[derive(Properties, PartialEq)]
pub struct ChatWindowProps {
    /// Current conversation ID
    pub conversation_id: Uuid,
    
    /// Language for the chat interface
    #[prop_or(Language::Thai)]
    pub language: Language,
    
    /// Whether the chat is in loading state
    #[prop_or(false)]
    pub loading: bool,
    
    /// Callback when a new message is sent
    #[prop_or_default]
    pub on_message_sent: Callback<ChatMessage>,
}

/// Main chat window component
#[function_component(ChatWindow)]
pub fn chat_window(props: &ChatWindowProps) -> Html {
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");
    
    // State management
    let messages = use_state(Vec::<ChatMessage>::new);
    let input_value = use_state(String::new);
    let is_sending = use_state(|| false);
    let input_ref = use_node_ref();
    
    // Send message handler
    let on_send_message = {
        let messages = messages.clone();
        let input_value = input_value.clone();
        let is_sending = is_sending.clone();
        let conversation_id = props.conversation_id;
        let language = props.language.clone();
        let on_message_sent = props.on_message_sent.clone();
        
        Callback::from(move |_| {
            let message_content = (*input_value).clone();
            if message_content.trim().is_empty() || *is_sending {
                return;
            }
            
            is_sending.set(true);
            input_value.set(String::new());
            
            // Create user message
            let user_message = ChatMessage {
                role: ChatRole::User,
                content: message_content.clone(),
                timestamp: Utc::now(),
            };
            
            // Add user message to chat
            let mut current_messages = (*messages).clone();
            current_messages.push(user_message.clone());
            messages.set(current_messages);
            
            // Emit message sent event
            on_message_sent.emit(user_message);
            
            // Send to API
            let messages_clone = messages.clone();
            let is_sending_clone = is_sending.clone();
            let language_clone = language.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                match ApiService::send_chat_message(
                    message_content,
                    conversation_id,
                    language_clone,
                ).await {
                    Ok(response) => {
                        let ai_message = ChatMessage {
                            role: ChatRole::Assistant,
                            content: response,
                            timestamp: Utc::now(),
                        };
                        
                        let mut current_messages = (*messages_clone).clone();
                        current_messages.push(ai_message);
                        messages_clone.set(current_messages);
                    }
                    Err(e) => {
                        log::error!("Failed to send message: {:?}", e);
                        let error_message = ChatMessage {
                            role: ChatRole::Assistant,
                            content: i18n.t("error.network"),
                            timestamp: Utc::now(),
                        };
                        
                        let mut current_messages = (*messages_clone).clone();
                        current_messages.push(error_message);
                        messages_clone.set(current_messages);
                    }
                }
                
                is_sending_clone.set(false);
            });
        })
    };
    
    // Input change handler
    let on_input_change = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };
    
    // Key press handler for Enter key
    let on_key_press = {
        let on_send_message = on_send_message.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" && !e.shift_key() {
                e.prevent_default();
                on_send_message.emit(());
            }
        })
    };
    
    html! {
        <div class="chat-window">
            <div class="chat-header">
                <h2 class="chat-title">
                    { i18n.t("app.title") }
                </h2>
                <div class="chat-status">
                    if *is_sending {
                        <span class="status-indicator sending">
                            { i18n.t("status.connecting") }
                        </span>
                    } else {
                        <span class="status-indicator connected">
                            { i18n.t("status.connected") }
                        </span>
                    }
                </div>
            </div>
            
            <div class="chat-messages">
                if messages.is_empty() {
                    <div class="welcome-message">
                        <div class="welcome-content">
                            <h3>{ i18n.t("welcome.title") }</h3>
                            <p>{ i18n.t("welcome.subtitle") }</p>
                            <div class="welcome-steps">
                                <p>{ i18n.t("welcome.how_to_use") }</p>
                                <ul>
                                    <li>{ i18n.t("welcome.step1") }</li>
                                    <li>{ i18n.t("welcome.step2") }</li>
                                    <li>{ i18n.t("welcome.step3") }</li>
                                    <li>{ i18n.t("welcome.step4") }</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                } else {
                    { for messages.iter().enumerate().map(|(index, message)| {
                        html! {
                            <MessageBubble
                                key={index}
                                message={message.clone()}
                                language={props.language.clone()}
                            />
                        }
                    })}
                }
                
                if *is_sending {
                    <div class="typing-indicator">
                        <div class="typing-dots">
                            <span class="dot"></span>
                            <span class="dot"></span>
                            <span class="dot"></span>
                        </div>
                        <span class="typing-text">
                            { i18n.t("chat.ai_typing") }
                        </span>
                    </div>
                }
            </div>
            
            <div class="chat-input">
                <div class="input-container">
                    <textarea
                        ref={input_ref}
                        class="message-input"
                        placeholder={i18n.t("chat.placeholder")}
                        value={(*input_value).clone()}
                        oninput={on_input_change}
                        onkeypress={on_key_press}
                        disabled={*is_sending}
                        rows="1"
                    />
                    <div class="input-actions">
                        <GradientButton
                            onclick={on_send_message}
                            disabled={input_value.trim().is_empty() || *is_sending}
                            loading={*is_sending}
                            size={crate::components::ui::ButtonSize::Medium}
                        >
                            { i18n.t("chat.send") }
                        </GradientButton>
                    </div>
                </div>
                
                <div class="input-tools">
                    <button class="tool-button" title={i18n.t("chat.upload_image")}>
                        <span class="icon">{"📷"}</span>
                        <span class="label">{ i18n.t("chat.take_photo") }</span>
                    </button>
                    <button class="tool-button" title={i18n.t("chat.voice_input")}>
                        <span class="icon">{"🎤"}</span>
                        <span class="label">{ i18n.t("chat.voice_input") }</span>
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Properties for message bubble component
#[derive(Properties, PartialEq)]
pub struct MessageBubbleProps {
    /// The chat message to display
    pub message: ChatMessage,
    
    /// Language for formatting
    #[prop_or(Language::Thai)]
    pub language: Language,
}

/// Individual message bubble component
#[function_component(MessageBubble)]
pub fn message_bubble(props: &MessageBubbleProps) -> Html {
    let message = &props.message;
    let is_user = matches!(message.role, ChatRole::User);
    
    let bubble_class = if is_user { "user-bubble" } else { "ai-bubble" };
    let message_class = if is_user { "user-message" } else { "ai-message" };
    
    html! {
        <div class={classes!("message", message_class)}>
            if !is_user {
                <div class="message-avatar">{"🤖"}</div>
            }
            
            <div class="message-content">
                <div class={classes!("message-bubble", bubble_class)}>
                    <div class="message-text">
                        { &message.content }
                    </div>
                </div>
                
                <div class="message-meta">
                    <span class="message-time">
                        { message.timestamp.format("%H:%M").to_string() }
                    </span>
                </div>
            </div>
            
            if is_user {
                <div class="message-avatar user-avatar">{"👤"}</div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_bubble_user() {
        let message = ChatMessage {
            role: ChatRole::User,
            content: "Test message".to_string(),
            timestamp: Utc::now(),
        };
        
        // Test that user messages are properly identified
        assert_eq!(message.role, ChatRole::User);
        assert_eq!(message.content, "Test message");
    }

    #[test]
    fn test_message_bubble_assistant() {
        let message = ChatMessage {
            role: ChatRole::Assistant,
            content: "AI response".to_string(),
            timestamp: Utc::now(),
        };
        
        // Test that assistant messages are properly identified
        assert_eq!(message.role, ChatRole::Assistant);
        assert_eq!(message.content, "AI response");
    }
}
