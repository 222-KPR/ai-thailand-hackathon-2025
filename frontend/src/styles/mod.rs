// Styles module for AI4Thai Crop Guardian - 2025 Design System
// Exports all styling utilities and design system components

pub mod colors;
pub mod typography;

pub use colors::{ColorPalette, Theme, ThemeProvider, ThemeContext, use_theme, generate_css_variables};
pub use typography::{TypographyScale, TypographyVariant, TypographyColor, Typography, generate_typography_css};

// Re-export commonly used types
pub use colors::{ColorPalette as Colors, Theme as AppTheme};
pub use typography::{TypographyVariant as TextVariant, TypographyColor as TextColor};
