// Copyright (c) 2025 AI4Thai Crop Guardian
// Licensed under the MIT License

//! Typography System with Thai Language Support
//!
//! This module defines the typography scale, font families, and text styles
//! optimized for both English and Thai language content.

/// Font families optimized for readability and Thai language support
pub struct FontFamilies;

impl FontFamilies {
    /// Primary heading font - Bold and expressive
    pub const HEADING: &'static str = r#""Poppins", "Sarabun", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif"#;

    /// Body text font - Clean and readable
    pub const BODY: &'static str = r#""Inter", "Sarabun", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif"#;

    /// Thai-optimized font for Thai language content
    pub const THAI: &'static str = r#""Sarabun", "Prompt", "Kanit", sans-serif"#;

    /// Monospace font for code and technical content
    pub const MONO: &'static str = r#""JetBrains Mono", "SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, "Courier New", monospace"#;
}

/// Font weights for different text emphasis levels
pub struct FontWeights;

impl FontWeights {
    pub const LIGHT: u16 = 300;
    pub const REGULAR: u16 = 400;
    pub const MEDIUM: u16 = 500;
    pub const SEMIBOLD: u16 = 600;
    pub const BOLD: u16 = 700;
    pub const EXTRABOLD: u16 = 800;
}

/// Typography scale with responsive sizing
pub struct TypographyScale;

impl TypographyScale {
    /// Display text - Largest heading for hero sections
    pub const DISPLAY_SIZE: &'static str = "clamp(2.5rem, 5vw, 4rem)";
    pub const DISPLAY_LINE_HEIGHT: f32 = 1.1;
    pub const DISPLAY_LETTER_SPACING: &'static str = "-0.02em";

    /// H1 - Main page headings
    pub const H1_SIZE: &'static str = "clamp(2rem, 4vw, 3rem)";
    pub const H1_LINE_HEIGHT: f32 = 1.2;
    pub const H1_LETTER_SPACING: &'static str = "-0.01em";

    /// H2 - Section headings
    pub const H2_SIZE: &'static str = "clamp(1.5rem, 3vw, 2.25rem)";
    pub const H2_LINE_HEIGHT: f32 = 1.3;
    pub const H2_LETTER_SPACING: &'static str = "-0.005em";

    /// H3 - Subsection headings
    pub const H3_SIZE: &'static str = "clamp(1.25rem, 2.5vw, 1.875rem)";
    pub const H3_LINE_HEIGHT: f32 = 1.4;
    pub const H3_LETTER_SPACING: &'static str = "0";

    /// H4 - Component headings
    pub const H4_SIZE: &'static str = "clamp(1.125rem, 2vw, 1.5rem)";
    pub const H4_LINE_HEIGHT: f32 = 1.4;
    pub const H4_LETTER_SPACING: &'static str = "0";

    /// H5 - Small headings
    pub const H5_SIZE: &'static str = "clamp(1rem, 1.5vw, 1.25rem)";
    pub const H5_LINE_HEIGHT: f32 = 1.5;
    pub const H5_LETTER_SPACING: &'static str = "0";

    /// H6 - Smallest headings
    pub const H6_SIZE: &'static str = "clamp(0.875rem, 1.25vw, 1.125rem)";
    pub const H6_LINE_HEIGHT: f32 = 1.5;
    pub const H6_LETTER_SPACING: &'static str = "0.01em";

    /// Body large - Prominent body text
    pub const BODY_LARGE_SIZE: &'static str = "1.125rem";
    pub const BODY_LARGE_LINE_HEIGHT: f32 = 1.6;

    /// Body regular - Standard body text
    pub const BODY_SIZE: &'static str = "1rem";
    pub const BODY_LINE_HEIGHT: f32 = 1.6;

    /// Body small - Secondary body text
    pub const BODY_SMALL_SIZE: &'static str = "0.875rem";
    pub const BODY_SMALL_LINE_HEIGHT: f32 = 1.5;

    /// Caption - Small descriptive text
    pub const CAPTION_SIZE: &'static str = "0.75rem";
    pub const CAPTION_LINE_HEIGHT: f32 = 1.4;
    pub const CAPTION_LETTER_SPACING: &'static str = "0.02em";

    /// Button text - UI button labels
    pub const BUTTON_SIZE: &'static str = "0.875rem";
    pub const BUTTON_LINE_HEIGHT: f32 = 1.2;
    pub const BUTTON_LETTER_SPACING: &'static str = "0.01em";

    /// Label - Form labels and small UI text
    pub const LABEL_SIZE: &'static str = "0.75rem";
    pub const LABEL_LINE_HEIGHT: f32 = 1.3;
    pub const LABEL_LETTER_SPACING: &'static str = "0.02em";
}

/// Thai language specific typography adjustments
pub struct ThaiTypography;

impl ThaiTypography {
    /// Increased line height for Thai text readability
    pub const THAI_LINE_HEIGHT_MULTIPLIER: f32 = 1.2;

    /// Thai text tends to be taller, so we adjust spacing
    pub const THAI_LETTER_SPACING: &'static str = "0.01em";

    /// Thai-specific font size adjustments
    pub const THAI_SIZE_MULTIPLIER: f32 = 1.05;
}

/// Text style presets for common use cases
pub struct TextStyles;

impl TextStyles {
    /// Hero text style for landing pages
    pub fn hero() -> String {
        format!(
            "font-family: {}; font-size: {}; font-weight: {}; line-height: {}; letter-spacing: {};",
            FontFamilies::HEADING,
            TypographyScale::DISPLAY_SIZE,
            FontWeights::BOLD,
            TypographyScale::DISPLAY_LINE_HEIGHT,
            TypographyScale::DISPLAY_LETTER_SPACING
        )
    }

    /// Page title style
    pub fn page_title() -> String {
        format!(
            "font-family: {}; font-size: {}; font-weight: {}; line-height: {}; letter-spacing: {};",
            FontFamilies::HEADING,
            TypographyScale::H1_SIZE,
            FontWeights::BOLD,
            TypographyScale::H1_LINE_HEIGHT,
            TypographyScale::H1_LETTER_SPACING
        )
    }

    /// Section heading style
    pub fn section_heading() -> String {
        format!(
            "font-family: {}; font-size: {}; font-weight: {}; line-height: {};",
            FontFamilies::HEADING,
            TypographyScale::H2_SIZE,
            FontWeights::SEMIBOLD,
            TypographyScale::H2_LINE_HEIGHT
        )
    }

    /// Body text style
    pub fn body_text() -> String {
        format!(
            "font-family: {}; font-size: {}; font-weight: {}; line-height: {};",
            FontFamilies::BODY,
            TypographyScale::BODY_SIZE,
            FontWeights::REGULAR,
            TypographyScale::BODY_LINE_HEIGHT
        )
    }

    /// Thai body text style with adjustments
    pub fn thai_body_text() -> String {
        format!(
            "font-family: {}; font-size: {}; font-weight: {}; line-height: {}; letter-spacing: {};",
            FontFamilies::THAI,
            TypographyScale::BODY_SIZE,
            FontWeights::REGULAR,
            TypographyScale::BODY_LINE_HEIGHT * ThaiTypography::THAI_LINE_HEIGHT_MULTIPLIER,
            ThaiTypography::THAI_LETTER_SPACING
        )
    }

    /// Button text style
    pub fn button_text() -> String {
        format!(
            "font-family: {}; font-size: {}; font-weight: {}; line-height: {}; letter-spacing: {};",
            FontFamilies::BODY,
            TypographyScale::BUTTON_SIZE,
            FontWeights::MEDIUM,
            TypographyScale::BUTTON_LINE_HEIGHT,
            TypographyScale::BUTTON_LETTER_SPACING
        )
    }

    /// Caption text style
    pub fn caption_text() -> String {
        format!(
            "font-family: {}; font-size: {}; font-weight: {}; line-height: {}; letter-spacing: {};",
            FontFamilies::BODY,
            TypographyScale::CAPTION_SIZE,
            FontWeights::REGULAR,
            TypographyScale::CAPTION_LINE_HEIGHT,
            TypographyScale::CAPTION_LETTER_SPACING
        )
    }
}

/// CSS custom properties for typography system
pub struct TypographyCSS;

impl TypographyCSS {
    /// Generate CSS custom properties for the typography system
    pub fn css_variables() -> String {
        format!(
            r#"
:root {{
  /* Font Families */
  --font-heading: {};
  --font-body: {};
  --font-thai: {};
  --font-mono: {};

  /* Font Weights */
  --font-weight-light: {};
  --font-weight-regular: {};
  --font-weight-medium: {};
  --font-weight-semibold: {};
  --font-weight-bold: {};
  --font-weight-extrabold: {};

  /* Font Sizes */
  --font-size-display: {};
  --font-size-h1: {};
  --font-size-h2: {};
  --font-size-h3: {};
  --font-size-h4: {};
  --font-size-h5: {};
  --font-size-h6: {};
  --font-size-body-large: {};
  --font-size-body: {};
  --font-size-body-small: {};
  --font-size-caption: {};
  --font-size-button: {};
  --font-size-label: {};

  /* Line Heights */
  --line-height-display: {};
  --line-height-h1: {};
  --line-height-h2: {};
  --line-height-h3: {};
  --line-height-h4: {};
  --line-height-h5: {};
  --line-height-h6: {};
  --line-height-body-large: {};
  --line-height-body: {};
  --line-height-body-small: {};
  --line-height-caption: {};
  --line-height-button: {};
  --line-height-label: {};

  /* Letter Spacing */
  --letter-spacing-display: {};
  --letter-spacing-h1: {};
  --letter-spacing-h2: {};
  --letter-spacing-caption: {};
  --letter-spacing-button: {};
  --letter-spacing-label: {};
  --letter-spacing-thai: {};
}}
"#,
            FontFamilies::HEADING,
            FontFamilies::BODY,
            FontFamilies::THAI,
            FontFamilies::MONO,
            FontWeights::LIGHT,
            FontWeights::REGULAR,
            FontWeights::MEDIUM,
            FontWeights::SEMIBOLD,
            FontWeights::BOLD,
            FontWeights::EXTRABOLD,
            TypographyScale::DISPLAY_SIZE,
            TypographyScale::H1_SIZE,
            TypographyScale::H2_SIZE,
            TypographyScale::H3_SIZE,
            TypographyScale::H4_SIZE,
            TypographyScale::H5_SIZE,
            TypographyScale::H6_SIZE,
            TypographyScale::BODY_LARGE_SIZE,
            TypographyScale::BODY_SIZE,
            TypographyScale::BODY_SMALL_SIZE,
            TypographyScale::CAPTION_SIZE,
            TypographyScale::BUTTON_SIZE,
            TypographyScale::LABEL_SIZE,
            TypographyScale::DISPLAY_LINE_HEIGHT,
            TypographyScale::H1_LINE_HEIGHT,
            TypographyScale::H2_LINE_HEIGHT,
            TypographyScale::H3_LINE_HEIGHT,
            TypographyScale::H4_LINE_HEIGHT,
            TypographyScale::H5_LINE_HEIGHT,
            TypographyScale::H6_LINE_HEIGHT,
            TypographyScale::BODY_LARGE_LINE_HEIGHT,
            TypographyScale::BODY_LINE_HEIGHT,
            TypographyScale::BODY_SMALL_LINE_HEIGHT,
            TypographyScale::CAPTION_LINE_HEIGHT,
            TypographyScale::BUTTON_LINE_HEIGHT,
            TypographyScale::LABEL_LINE_HEIGHT,
            TypographyScale::DISPLAY_LETTER_SPACING,
            TypographyScale::H1_LETTER_SPACING,
            TypographyScale::H2_LETTER_SPACING,
            TypographyScale::CAPTION_LETTER_SPACING,
            TypographyScale::BUTTON_LETTER_SPACING,
            TypographyScale::LABEL_LETTER_SPACING,
            ThaiTypography::THAI_LETTER_SPACING,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_families() {
        assert!(FontFamilies::HEADING.contains("Poppins"));
        assert!(FontFamilies::BODY.contains("Inter"));
        assert!(FontFamilies::THAI.contains("Sarabun"));
    }

    #[test]
    fn test_font_weights() {
        assert_eq!(FontWeights::REGULAR, 400);
        assert_eq!(FontWeights::BOLD, 700);
    }

    #[test]
    fn test_text_styles() {
        let hero_style = TextStyles::hero();
        assert!(hero_style.contains("Poppins"));
        assert!(hero_style.contains("700"));
    }

    #[test]
    fn test_thai_typography() {
        // Thai line height multiplier should be greater than 1.0 for better readability
        let multiplier = ThaiTypography::THAI_LINE_HEIGHT_MULTIPLIER;
        assert!(multiplier > 1.0, "Thai line height multiplier should be > 1.0, got {}", multiplier);
    }
}
