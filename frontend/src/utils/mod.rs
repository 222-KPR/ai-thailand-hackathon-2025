// Utils module for AI4Thai Crop Guardian - 2025 Design System
// Exports all utility functions, design tokens, and optimization tools

pub mod design_tokens;
pub mod responsive;
pub mod performance;
pub mod accessibility;

// Design tokens
pub use design_tokens::{
    DesignTokens, SpacingScale, SizingScale, BorderScale, ShadowScale, 
    AnimationScale, BreakpointScale, generate_design_tokens_css, use_design_tokens
};

// Responsive utilities
pub use responsive::{
    Breakpoint, BreakpointConfig, use_breakpoint, use_media_query, use_window_size,
    Responsive, Show, Hide, Container, generate_responsive_css
};

// Performance utilities
pub use performance::{
    PerformanceMetrics, use_performance_monitor, use_lazy_image, use_debounce, 
    use_throttle, use_virtual_scroll, use_memo, VirtualScrollConfig,
    LazyImage, ProgressBar, Skeleton, LoadingOverlay, PerformanceMonitor,
    generate_performance_css
};

// Accessibility utilities
pub use accessibility::{
    AriaRole, AriaAttributes, use_keyboard_navigation, use_focus_management, 
    use_screen_reader, SkipLink, AccessibleButton, LiveRegion, FocusTrap,
    check_color_contrast, generate_accessibility_css
};

// Re-export commonly used utilities
pub use design_tokens::{DesignTokens as Tokens};
pub use responsive::{Breakpoint as BP, use_breakpoint as use_bp};
pub use performance::{use_debounce as debounce, use_throttle as throttle};
pub use accessibility::{AriaRole as Role, AriaAttributes as Aria};
