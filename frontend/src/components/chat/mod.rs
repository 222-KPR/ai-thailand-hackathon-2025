// Chat components module for AI4Thai Crop Guardian - 2025 Design System
// Exports all chat-related components

pub mod chat_window;

pub use chat_window::{
    ChatWindow, ChatWindowProps,
    MessageBubble, MessageBubbleProps,
    ChatMessage, MessageSender, MessageType, MessageAttachment,
    generate_chat_css
};

// Re-export commonly used types
pub use chat_window::{
    MessageSender as Sender, MessageType as MsgType,
    ChatMessage as Message
};
