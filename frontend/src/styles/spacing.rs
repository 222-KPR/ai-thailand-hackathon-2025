// Copyright (c) 2025 AI4Thai Crop Guardian
// Licensed under the MIT License

//! Spacing and Layout System
//! 
//! This module defines the spacing scale, layout utilities, and responsive
//! breakpoints for consistent spacing throughout the application.

/// Base spacing unit (in rem) for consistent spacing scale
pub const BASE_SPACING: f32 = 0.25; // 4px at default font size

/// Spacing scale based on multiples of the base unit
pub struct Spacing;

impl Spacing {
    /// 0px - No spacing
    pub const NONE: &'static str = "0";
    
    /// 2px - Minimal spacing
    pub const XS: &'static str = "0.125rem";
    
    /// 4px - Base unit
    pub const SM: &'static str = "0.25rem";
    
    /// 8px - Small spacing
    pub const MD: &'static str = "0.5rem";
    
    /// 12px - Medium spacing
    pub const LG: &'static str = "0.75rem";
    
    /// 16px - Standard spacing
    pub const XL: &'static str = "1rem";
    
    /// 20px - Large spacing
    pub const XXL: &'static str = "1.25rem";
    
    /// 24px - Extra large spacing
    pub const XXXL: &'static str = "1.5rem";
    
    /// 32px - Section spacing
    pub const SECTION: &'static str = "2rem";
    
    /// 48px - Large section spacing
    pub const SECTION_LG: &'static str = "3rem";
    
    /// 64px - Page spacing
    pub const PAGE: &'static str = "4rem";
    
    /// 96px - Large page spacing
    pub const PAGE_LG: &'static str = "6rem";
    
    /// 128px - Hero spacing
    pub const HERO: &'static str = "8rem";
}

/// Container widths for different breakpoints
pub struct ContainerWidths;

impl ContainerWidths {
    /// Small container - Mobile
    pub const SM: &'static str = "100%";
    
    /// Medium container - Tablet
    pub const MD: &'static str = "768px";
    
    /// Large container - Desktop
    pub const LG: &'static str = "1024px";
    
    /// Extra large container - Large desktop
    pub const XL: &'static str = "1280px";
    
    /// Maximum container width
    pub const MAX: &'static str = "1440px";
}

/// Responsive breakpoints
pub struct Breakpoints;

impl Breakpoints {
    /// Mobile breakpoint
    pub const SM: &'static str = "640px";
    
    /// Tablet breakpoint
    pub const MD: &'static str = "768px";
    
    /// Desktop breakpoint
    pub const LG: &'static str = "1024px";
    
    /// Large desktop breakpoint
    pub const XL: &'static str = "1280px";
    
    /// Extra large desktop breakpoint
    pub const XXL: &'static str = "1536px";
}

/// Z-index scale for layering elements
pub struct ZIndex;

impl ZIndex {
    /// Behind content
    pub const BEHIND: i32 = -1;
    
    /// Base layer
    pub const BASE: i32 = 0;
    
    /// Elevated content
    pub const ELEVATED: i32 = 10;
    
    /// Dropdown menus
    pub const DROPDOWN: i32 = 100;
    
    /// Sticky elements
    pub const STICKY: i32 = 200;
    
    /// Fixed elements
    pub const FIXED: i32 = 300;
    
    /// Modal backdrop
    pub const MODAL_BACKDROP: i32 = 400;
    
    /// Modal content
    pub const MODAL: i32 = 500;
    
    /// Popover content
    pub const POPOVER: i32 = 600;
    
    /// Tooltip content
    pub const TOOLTIP: i32 = 700;
    
    /// Notification toasts
    pub const TOAST: i32 = 800;
    
    /// Maximum z-index
    pub const MAX: i32 = 9999;
}

/// Border radius scale for consistent rounded corners
pub struct BorderRadius;

impl BorderRadius {
    /// No radius - Sharp corners
    pub const NONE: &'static str = "0";
    
    /// Small radius - Subtle rounding
    pub const SM: &'static str = "0.125rem";
    
    /// Medium radius - Standard rounding
    pub const MD: &'static str = "0.375rem";
    
    /// Large radius - Prominent rounding
    pub const LG: &'static str = "0.5rem";
    
    /// Extra large radius - Very rounded
    pub const XL: &'static str = "0.75rem";
    
    /// Double extra large radius - Highly rounded
    pub const XXL: &'static str = "1rem";
    
    /// Triple extra large radius - Maximum rounding
    pub const XXXL: &'static str = "1.5rem";
    
    /// Full radius - Circular/pill shape
    pub const FULL: &'static str = "9999px";
}

/// Shadow scale for depth and elevation
pub struct Shadows;

impl Shadows {
    /// No shadow
    pub const NONE: &'static str = "none";
    
    /// Small shadow - Subtle elevation
    pub const SM: &'static str = "0 1px 2px 0 rgb(0 0 0 / 0.05)";
    
    /// Medium shadow - Standard elevation
    pub const MD: &'static str = "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)";
    
    /// Large shadow - Prominent elevation
    pub const LG: &'static str = "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)";
    
    /// Extra large shadow - High elevation
    pub const XL: &'static str = "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)";
    
    /// Double extra large shadow - Maximum elevation
    pub const XXL: &'static str = "0 25px 50px -12px rgb(0 0 0 / 0.25)";
    
    /// Inner shadow - Inset effect
    pub const INNER: &'static str = "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)";
    
    /// Colored shadow for dopamine design
    pub const COLORED: &'static str = "0 8px 32px rgb(0 102 255 / 0.2)";
}

/// Layout utilities for common patterns
pub struct Layout;

impl Layout {
    /// Flexbox center alignment
    pub fn flex_center() -> String {
        "display: flex; align-items: center; justify-content: center;".to_string()
    }
    
    /// Flexbox space between
    pub fn flex_between() -> String {
        "display: flex; align-items: center; justify-content: space-between;".to_string()
    }
    
    /// Flexbox column layout
    pub fn flex_column() -> String {
        "display: flex; flex-direction: column;".to_string()
    }
    
    /// Grid layout with auto-fit columns
    pub fn grid_auto_fit(min_width: &str) -> String {
        format!(
            "display: grid; grid-template-columns: repeat(auto-fit, minmax({}, 1fr)); gap: {};",
            min_width,
            Spacing::XL
        )
    }
    
    /// Responsive container with max width
    pub fn container() -> String {
        format!(
            "width: 100%; max-width: {}; margin-left: auto; margin-right: auto; padding-left: {}; padding-right: {};",
            ContainerWidths::MAX,
            Spacing::XL,
            Spacing::XL
        )
    }
    
    /// Aspect ratio container
    pub fn aspect_ratio(ratio: &str) -> String {
        format!("aspect-ratio: {}; overflow: hidden;", ratio)
    }
    
    /// Visually hidden but accessible to screen readers
    pub fn visually_hidden() -> String {
        "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0;".to_string()
    }
}

/// CSS custom properties for spacing system
pub struct SpacingCSS;

impl SpacingCSS {
    /// Generate CSS custom properties for the spacing system
    pub fn css_variables() -> String {
        format!(
            r#"
:root {{
  /* Spacing Scale */
  --spacing-none: {};
  --spacing-xs: {};
  --spacing-sm: {};
  --spacing-md: {};
  --spacing-lg: {};
  --spacing-xl: {};
  --spacing-xxl: {};
  --spacing-xxxl: {};
  --spacing-section: {};
  --spacing-section-lg: {};
  --spacing-page: {};
  --spacing-page-lg: {};
  --spacing-hero: {};
  
  /* Container Widths */
  --container-sm: {};
  --container-md: {};
  --container-lg: {};
  --container-xl: {};
  --container-max: {};
  
  /* Breakpoints */
  --breakpoint-sm: {};
  --breakpoint-md: {};
  --breakpoint-lg: {};
  --breakpoint-xl: {};
  --breakpoint-xxl: {};
  
  /* Border Radius */
  --radius-none: {};
  --radius-sm: {};
  --radius-md: {};
  --radius-lg: {};
  --radius-xl: {};
  --radius-xxl: {};
  --radius-xxxl: {};
  --radius-full: {};
  
  /* Shadows */
  --shadow-none: {};
  --shadow-sm: {};
  --shadow-md: {};
  --shadow-lg: {};
  --shadow-xl: {};
  --shadow-xxl: {};
  --shadow-inner: {};
  --shadow-colored: {};
  
  /* Z-Index */
  --z-behind: {};
  --z-base: {};
  --z-elevated: {};
  --z-dropdown: {};
  --z-sticky: {};
  --z-fixed: {};
  --z-modal-backdrop: {};
  --z-modal: {};
  --z-popover: {};
  --z-tooltip: {};
  --z-toast: {};
  --z-max: {};
}}
"#,
            Spacing::NONE,
            Spacing::XS,
            Spacing::SM,
            Spacing::MD,
            Spacing::LG,
            Spacing::XL,
            Spacing::XXL,
            Spacing::XXXL,
            Spacing::SECTION,
            Spacing::SECTION_LG,
            Spacing::PAGE,
            Spacing::PAGE_LG,
            Spacing::HERO,
            ContainerWidths::SM,
            ContainerWidths::MD,
            ContainerWidths::LG,
            ContainerWidths::XL,
            ContainerWidths::MAX,
            Breakpoints::SM,
            Breakpoints::MD,
            Breakpoints::LG,
            Breakpoints::XL,
            Breakpoints::XXL,
            BorderRadius::NONE,
            BorderRadius::SM,
            BorderRadius::MD,
            BorderRadius::LG,
            BorderRadius::XL,
            BorderRadius::XXL,
            BorderRadius::XXXL,
            BorderRadius::FULL,
            Shadows::NONE,
            Shadows::SM,
            Shadows::MD,
            Shadows::LG,
            Shadows::XL,
            Shadows::XXL,
            Shadows::INNER,
            Shadows::COLORED,
            ZIndex::BEHIND,
            ZIndex::BASE,
            ZIndex::ELEVATED,
            ZIndex::DROPDOWN,
            ZIndex::STICKY,
            ZIndex::FIXED,
            ZIndex::MODAL_BACKDROP,
            ZIndex::MODAL,
            ZIndex::POPOVER,
            ZIndex::TOOLTIP,
            ZIndex::TOAST,
            ZIndex::MAX,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_scale() {
        assert_eq!(Spacing::NONE, "0");
        assert_eq!(Spacing::XL, "1rem");
        assert_eq!(Spacing::SECTION, "2rem");
    }

    #[test]
    fn test_breakpoints() {
        assert_eq!(Breakpoints::SM, "640px");
        assert_eq!(Breakpoints::LG, "1024px");
    }

    #[test]
    fn test_z_index() {
        assert_eq!(ZIndex::BASE, 0);
        // Modal should have higher z-index than dropdown for proper layering
        let modal_z = ZIndex::MODAL;
        let dropdown_z = ZIndex::DROPDOWN;
        assert!(modal_z > dropdown_z, "Modal z-index ({}) should be > dropdown z-index ({})", modal_z, dropdown_z);
    }

    #[test]
    fn test_layout_utilities() {
        let center = Layout::flex_center();
        assert!(center.contains("display: flex"));
        assert!(center.contains("align-items: center"));
    }

    #[test]
    fn test_border_radius() {
        assert_eq!(BorderRadius::NONE, "0");
        assert_eq!(BorderRadius::FULL, "9999px");
    }
}
