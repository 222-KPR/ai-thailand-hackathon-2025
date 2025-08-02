// UI Components module for AI4Thai Crop Guardian - 2025 Design System
// Exports all UI components with modern design and micro-interactions

pub mod gradient_button;
pub mod status_card;

pub use gradient_button::{
    GradientButton, GradientButtonProps, ButtonSize, ButtonVariant,
    generate_gradient_button_css
};

pub use status_card::{
    StatusCard, StatusCardProps, StatusCardVariant, TrendDirection,
    QuickAction, QuickActionProps,
    generate_status_card_css
};

// Re-export commonly used types
pub use gradient_button::{ButtonSize as BtnSize, ButtonVariant as BtnVariant};
pub use status_card::{StatusCardVariant as CardVariant, TrendDirection as Trend};
