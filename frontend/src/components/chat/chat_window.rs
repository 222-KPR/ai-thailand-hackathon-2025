// Chat Window Component - 2025 Design System
// Modern chat interface with AI personality and multimodal input

use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::components::layout::{BentoGrid, BentoCard};
use crate::components::ui::{GradientButton, ButtonVariant, ButtonSize};
use crate::styles::{use_theme, Typography, TypographyVariant, TypographyColor};

#[derive(Debug, Clone, PartialEq)]
pub struct ChatMessage {
    pub id: String,
    pub sender: MessageSender,
    pub content: String,
    pub message_type: MessageType,
    pub timestamp: String,
    pub confidence: Option<f32>,
    pub sources: Option<Vec<String>>,
    pub attachments: Option<Vec<MessageAttachment>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageSender {
    User,
    AI,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    Text,
    Image,
    Voice,
    System,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageAttachment {
    pub attachment_type: String,
    pub url: String,
    pub filename: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct ChatWindowProps {
    pub messages: Vec<ChatMessage>,
    pub on_send_message: Callback<String>,
    pub on_send_image: Option<Callback<String>>,
    pub on_send_voice: Option<Callback<Vec<u8>>>,
    pub typing: Option<bool>,
    pub class: Option<String>,
}

#[function_component(ChatWindow)]
pub fn chat_window(props: &ChatWindowProps) -> Html {
    let theme = use_theme();
    let colors = &theme.colors;
    
    let input_ref = use_node_ref();
    let message_input = use_state(String::new);
    let is_recording = use_state(|| false);
    let show_quick_actions = use_state(|| true);
    let typing = props.typing.unwrap_or(false);
    
    // Send text message
    let send_message = {
        let message_input = message_input.clone();
        let on_send_message = props.on_send_message.clone();
        let input_ref = input_ref.clone();
        
        Callback::from(move |_| {
            let message = (*message_input).clone();
            if !message.trim().is_empty() {
                on_send_message.emit(message);
                message_input.set(String::new());
                
                // Clear input field
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    input.set_value("");
                }
            }
        })
    };
    
    // Handle input change
    let on_input_change = {
        let message_input = message_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            message_input.set(input.value());
        })
    };
    
    // Handle enter key
    let on_key_press = {
        let send_message = send_message.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" && !e.shift_key() {
                e.prevent_default();
                send_message.emit(());
            }
        })
    };
    
    // Quick action callbacks
    let take_photo = Callback::from(|_| {
        web_sys::console::log_1(&"Take photo".into());
    });
    
    let record_voice = {
        let is_recording = is_recording.clone();
        Callback::from(move |_| {
            is_recording.set(!*is_recording);
            web_sys::console::log_1(&"Toggle voice recording".into());
        })
    };
    
    let share_location = Callback::from(|_| {
        web_sys::console::log_1(&"Share location".into());
    });
    
    let toggle_quick_actions = {
        let show_quick_actions = show_quick_actions.clone();
        Callback::from(move |_| {
            show_quick_actions.set(!*show_quick_actions);
        })
    };

    html! {
        <div class={classes!("chat-window", props.class.clone())}>
            // Chat Header with AI Avatar
            <div class="chat-header">
                <div class="ai-avatar">
                    <div class="avatar-image">{"🤖"}</div>
                    <div class="avatar-status online"></div>
                </div>
                <div class="chat-title-group">
                    <Typography variant={TypographyVariant::H4} class="chat-title">
                        {"ผู้ช่วย AI เกษตรกร"}
                    </Typography>
                    <div class="chat-status">
                        if typing {
                            <div class="typing-indicator">
                                <span class="typing-dot"></span>
                                <span class="typing-dot"></span>
                                <span class="typing-dot"></span>
                                <Typography variant={TypographyVariant::Caption} color={TypographyColor::Secondary} class="thai-text">
                                    {"กำลังพิมพ์..."}
                                </Typography>
                            </div>
                        } else {
                            <Typography variant={TypographyVariant::Caption} color={TypographyColor::Success}>
                                {"🟢 พร้อมให้บริการ"}
                            </Typography>
                        }
                    </div>
                </div>
                <button class="chat-menu-btn" onclick={toggle_quick_actions}>
                    {"⋯"}
                </button>
            </div>

            // Messages Container
            <div class="messages-container">
                <div class="messages-list">
                    { for props.messages.iter().map(|message| {
                        html! {
                            <MessageBubble message={message.clone()} />
                        }
                    })}
                    
                    // Typing indicator message
                    if typing {
                        <div class="message ai-message typing-message">
                            <div class="message-avatar">{"🤖"}</div>
                            <div class="message-bubble ai-bubble">
                                <div class="typing-animation">
                                    <div class="typing-dot"></div>
                                    <div class="typing-dot"></div>
                                    <div class="typing-dot"></div>
                                </div>
                            </div>
                        </div>
                    }
                </div>
            </div>

            // Input Container
            <div class="chat-input-container">
                // Quick Actions (collapsible)
                if *show_quick_actions {
                    <div class="quick-actions">
                        <BentoGrid columns={3} gap="0.5rem">
                            <BentoCard 
                                color={colors.accent_lime_green} 
                                hover_effect={true}
                                clickable={true}
                                onclick={take_photo}
                            >
                                <div class="quick-action">
                                    <div class="quick-action-icon">{"📷"}</div>
                                    <Typography variant={TypographyVariant::Caption} color={TypographyColor::Inverse} class="thai-text">
                                        {"ถ่ายรูป"}
                                    </Typography>
                                </div>
                            </BentoCard>
                            
                            <BentoCard 
                                color={if *is_recording { colors.error } else { colors.accent_purple }}
                                hover_effect={true}
                                clickable={true}
                                onclick={record_voice}
                            >
                                <div class="quick-action">
                                    <div class="quick-action-icon">
                                        {if *is_recording { "⏹️" } else { "🎤" }}
                                    </div>
                                    <Typography variant={TypographyVariant::Caption} color={TypographyColor::Inverse} class="thai-text">
                                        {if *is_recording { "หยุด" } else { "บันทึกเสียง" }}
                                    </Typography>
                                </div>
                            </BentoCard>
                            
                            <BentoCard 
                                color={colors.accent_yellow} 
                                hover_effect={true}
                                clickable={true}
                                onclick={share_location}
                            >
                                <div class="quick-action">
                                    <div class="quick-action-icon">{"📍"}</div>
                                    <Typography variant={TypographyVariant::Caption} color={TypographyColor::Primary} class="thai-text">
                                        {"ตำแหน่ง"}
                                    </Typography>
                                </div>
                            </BentoCard>
                        </BentoGrid>
                    </div>
                }
                
                // Main Input Area
                <div class="input-area">
                    <BentoGrid columns={5} gap="0.5rem">
                        <BentoCard span_cols={4} class="input-card">
                            <div class="input-wrapper">
                                <input
                                    ref={input_ref}
                                    type="text"
                                    placeholder="พิมพ์คำถามของคุณ..."
                                    class="chat-input thai-text"
                                    value={(*message_input).clone()}
                                    oninput={on_input_change}
                                    onkeypress={on_key_press}
                                />
                                <button 
                                    class="input-action-btn"
                                    onclick={toggle_quick_actions}
                                    title="เพิ่มเติม"
                                >
                                    {"+""}
                                </button>
                            </div>
                        </BentoCard>
                        
                        <BentoCard gradient={colors.get_primary_gradient()} hover_effect={true}>
                            <GradientButton
                                variant={ButtonVariant::Primary}
                                size={ButtonSize::Medium}
                                onclick={send_message}
                                icon="📤"
                                full_width={true}
                                disabled={message_input.trim().is_empty()}
                            >
                                {""}
                            </GradientButton>
                        </BentoCard>
                    </BentoGrid>
                </div>
            </div>
        </div>
    }
}

// Message Bubble Component
#[derive(Properties, PartialEq)]
pub struct MessageBubbleProps {
    pub message: ChatMessage,
}

#[function_component(MessageBubble)]
pub fn message_bubble(props: &MessageBubbleProps) -> Html {
    let theme = use_theme();
    let colors = &theme.colors;
    let message = &props.message;
    
    let is_user = matches!(message.sender, MessageSender::User);
    let bubble_class = if is_user { "user-bubble" } else { "ai-bubble" };
    let message_class = if is_user { "user-message" } else { "ai-message" };

    html! {
        <div class={classes!("message", message_class)}>
            if !is_user {
                <div class="message-avatar">{"🤖"}</div>
            }
            
            <div class="message-content">
                <div class={classes!("message-bubble", bubble_class)}>
                    <Typography variant={TypographyVariant::Body1} class="message-text thai-text">
                        {&message.content}
                    </Typography>
                    
                    // Confidence indicator for AI messages
                    if let Some(confidence) = message.confidence {
                        <div class="message-confidence">
                            <div class="confidence-bar">
                                <div 
                                    class="confidence-fill"
                                    style={format!("width: {}%", confidence * 100.0)}
                                ></div>
                            </div>
                            <Typography variant={TypographyVariant::Caption} color={TypographyColor::Secondary}>
                                {format!("ความมั่นใจ {}%", (confidence * 100.0) as u8)}
                            </Typography>
                        </div>
                    }
                    
                    // Sources for AI messages
                    if let Some(sources) = &message.sources {
                        if !sources.is_empty() {
                            <div class="message-sources">
                                <Typography variant={TypographyVariant::Caption} color={TypographyColor::Secondary}>
                                    {"แหล่งข้อมูล: "}
                                    {sources.join(", ")}
                                </Typography>
                            </div>
                        }
                    }
                </div>
                
                <div class="message-meta">
                    <Typography variant={TypographyVariant::Caption} color={TypographyColor::Disabled}>
                        {&message.timestamp}
                    </Typography>
                </div>
            </div>
            
            if is_user {
                <div class="message-avatar user-avatar">{"👤"}</div>
            }
        </div>
    }
}

// CSS for chat interface
pub fn generate_chat_css() -> String {
    r#"/* Chat Interface Styles - 2025 Design */

.chat-window {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--color-bg-light);
}

/* Chat Header */
.chat-header {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-lg);
  background: var(--color-surface-light);
  border-bottom: 1px solid var(--color-bg-light);
  box-shadow: var(--shadow-sm);
}

.ai-avatar {
  position: relative;
  width: 48px;
  height: 48px;
  border-radius: var(--radius-full);
  background: var(--gradient-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  flex-shrink: 0;
}

.avatar-status {
  position: absolute;
  bottom: 2px;
  right: 2px;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: 2px solid var(--color-surface-light);
}

.avatar-status.online {
  background: var(--color-success);
}

.chat-title-group {
  flex: 1;
  min-width: 0;
}

.chat-title {
  margin-bottom: var(--space-xs);
}

.chat-status {
  display: flex;
  align-items: center;
  gap: var(--space-xs);
}

.chat-menu-btn {
  background: none;
  border: none;
  font-size: 1.25rem;
  color: var(--color-text-secondary);
  cursor: pointer;
  padding: var(--space-sm);
  border-radius: var(--radius-md);
  transition: background-color 0.2s ease;
}

.chat-menu-btn:hover {
  background: var(--color-bg-light);
}

/* Typing Indicator */
.typing-indicator {
  display: flex;
  align-items: center;
  gap: var(--space-xs);
}

.typing-dot {
  width: 4px;
  height: 4px;
  border-radius: 50%;
  background: var(--color-primary-electric-blue);
  animation: typing-bounce 1.4s ease-in-out infinite both;
}

.typing-dot:nth-child(1) { animation-delay: -0.32s; }
.typing-dot:nth-child(2) { animation-delay: -0.16s; }
.typing-dot:nth-child(3) { animation-delay: 0s; }

@keyframes typing-bounce {
  0%, 80%, 100% {
    transform: scale(0);
    opacity: 0.5;
  }
  40% {
    transform: scale(1);
    opacity: 1;
  }
}

/* Messages Container */
.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-lg);
  scroll-behavior: smooth;
}

.messages-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
  max-width: 800px;
  margin: 0 auto;
}

/* Message Styles */
.message {
  display: flex;
  gap: var(--space-md);
  align-items: flex-end;
  animation: message-fade-in 0.3s ease-out;
}

.user-message {
  flex-direction: row-reverse;
}

.message-avatar {
  width: 32px;
  height: 32px;
  border-radius: var(--radius-full);
  background: var(--gradient-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1rem;
  flex-shrink: 0;
}

.user-avatar {
  background: var(--color-text-secondary);
}

.message-content {
  flex: 1;
  max-width: 70%;
  min-width: 0;
}

.message-bubble {
  padding: var(--space-md) var(--space-lg);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-sm);
  position: relative;
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.ai-bubble {
  background: var(--color-surface-light);
  color: var(--color-text-primary);
  border-bottom-left-radius: var(--radius-md);
}

.user-bubble {
  background: var(--gradient-primary);
  color: var(--color-text-inverse);
  border-bottom-right-radius: var(--radius-md);
}

.message-text {
  margin: 0;
  line-height: var(--leading-relaxed);
}

.message-confidence {
  margin-top: var(--space-sm);
  padding-top: var(--space-sm);
  border-top: 1px solid rgba(0, 0, 0, 0.1);
}

.confidence-bar {
  width: 100%;
  height: 4px;
  background: rgba(0, 0, 0, 0.1);
  border-radius: var(--radius-full);
  overflow: hidden;
  margin-bottom: var(--space-xs);
}

.confidence-fill {
  height: 100%;
  background: var(--color-success);
  border-radius: var(--radius-full);
  transition: width 0.3s ease;
}

.message-sources {
  margin-top: var(--space-sm);
  padding-top: var(--space-sm);
  border-top: 1px solid rgba(0, 0, 0, 0.1);
}

.message-meta {
  margin-top: var(--space-xs);
  text-align: right;
}

.user-message .message-meta {
  text-align: left;
}

/* Typing Animation */
.typing-message .message-bubble {
  padding: var(--space-lg);
}

.typing-animation {
  display: flex;
  gap: var(--space-xs);
  justify-content: center;
}

.typing-animation .typing-dot {
  width: 8px;
  height: 8px;
  background: var(--color-text-secondary);
}

/* Input Container */
.chat-input-container {
  padding: var(--space-lg);
  background: var(--color-surface-light);
  border-top: 1px solid var(--color-bg-light);
  box-shadow: var(--shadow-sm);
}

.quick-actions {
  margin-bottom: var(--space-md);
}

.quick-action {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-xs);
  padding: var(--space-sm);
  text-align: center;
}

.quick-action-icon {
  font-size: 1.25rem;
}

.input-area {
  max-width: 800px;
  margin: 0 auto;
}

.input-card {
  background: var(--color-bg-light);
  border: 2px solid transparent;
  transition: border-color 0.2s ease;
}

.input-card:focus-within {
  border-color: var(--color-primary-electric-blue);
}

.input-wrapper {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  width: 100%;
}

.chat-input {
  flex: 1;
  border: none;
  background: transparent;
  font-family: var(--font-body);
  font-size: var(--text-base);
  color: var(--color-text-primary);
  padding: var(--space-md);
  outline: none;
  resize: none;
  min-height: 44px;
}

.chat-input::placeholder {
  color: var(--color-text-disabled);
}

.input-action-btn {
  background: var(--color-primary-electric-blue);
  color: var(--color-text-inverse);
  border: none;
  width: 32px;
  height: 32px;
  border-radius: var(--radius-full);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  font-size: 1.25rem;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.input-action-btn:hover {
  background: var(--color-primary-energetic-pink);
  transform: scale(1.05);
}

/* Animations */
@keyframes message-fade-in {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Responsive Design */
@media (max-width: 768px) {
  .chat-header {
    padding: var(--space-md);
  }
  
  .messages-container {
    padding: var(--space-md);
  }
  
  .chat-input-container {
    padding: var(--space-md);
  }
  
  .message-content {
    max-width: 85%;
  }
  
  .quick-actions {
    margin-bottom: var(--space-sm);
  }
  
  .quick-action {
    padding: var(--space-xs);
  }
  
  .quick-action-icon {
    font-size: 1rem;
  }
}

@media (max-width: 480px) {
  .ai-avatar {
    width: 40px;
    height: 40px;
    font-size: 1.25rem;
  }
  
  .message-avatar {
    width: 28px;
    height: 28px;
    font-size: 0.875rem;
  }
  
  .message-bubble {
    padding: var(--space-sm) var(--space-md);
  }
  
  .chat-input {
    padding: var(--space-sm);
  }
}

/* Accessibility */
@media (prefers-reduced-motion: reduce) {
  .typing-dot {
    animation: none;
  }
  
  .message {
    animation: none;
  }
  
  .input-action-btn:hover {
    transform: none;
  }
}

/* Scrollbar styling */
.messages-container::-webkit-scrollbar {
  width: 6px;
}

.messages-container::-webkit-scrollbar-track {
  background: var(--color-bg-light);
}

.messages-container::-webkit-scrollbar-thumb {
  background: var(--color-text-disabled);
  border-radius: var(--radius-full);
}

.messages-container::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-secondary);
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_sender() {
        let user = MessageSender::User;
        let ai = MessageSender::AI;
        assert_ne!(user, ai);
    }

    #[test]
    fn test_message_type() {
        let text = MessageType::Text;
        let image = MessageType::Image;
        assert_ne!(text, image);
    }

    #[test]
    fn test_css_generation() {
        let css = generate_chat_css();
        assert!(css.contains("chat-window"));
        assert!(css.contains("message-bubble"));
        assert!(css.contains("@keyframes"));
    }
}
