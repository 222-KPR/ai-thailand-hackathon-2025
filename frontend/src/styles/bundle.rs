// CSS Bundle Generator - 2025 Design System
// Combines all CSS from components and utilities into a single bundle

use crate::styles::{generate_css_variables, generate_typography_css};
use crate::components::layout::generate_bento_css;
use crate::components::ui::{
    generate_gradient_button_css, generate_status_card_css, generate_loading_css
};
use crate::components::camera::generate_camera_css;
use crate::components::chat::generate_chat_css;
use crate::pages::generate_homepage_css;
use crate::utils::{
    generate_design_tokens_css, generate_responsive_css,
    generate_performance_css, generate_accessibility_css, DesignTokens
};

// Generate complete CSS bundle
pub fn generate_complete_css_bundle() -> String {
    let tokens = DesignTokens::default();

    format!(
        r#"/* AI4Thai Crop Guardian - Complete CSS Bundle */
/* 2025 Design System with Dopamine Colors and Bento Grids */

/* ==========================================================================
   CSS Reset and Base Styles
   ========================================================================== */

/* Modern CSS Reset */
*,
*::before,
*::after {{
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}}

html {{
  font-size: 16px;
  scroll-behavior: smooth;
  -webkit-text-size-adjust: 100%;
  -ms-text-size-adjust: 100%;
}}

body {{
  font-family: 'Inter', 'Sarabun', -apple-system, BlinkMacSystemFont, sans-serif;
  font-size: 1rem;
  font-weight: 400;
  line-height: 1.625;
  color: var(--color-text-primary);
  background-color: var(--color-bg-light);
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-rendering: optimizeLegibility;
  overflow-x: hidden;
}}

/* ==========================================================================
   Design System Tokens
   ========================================================================== */

{}

/* ==========================================================================
   Typography System
   ========================================================================== */

{}

/* ==========================================================================
   Layout System - Bento Grids
   ========================================================================== */

{}

/* ==========================================================================
   UI Components
   ========================================================================== */

/* Gradient Buttons */
{}

/* Status Cards */
{}

/* Loading Animations */
{}

/* ==========================================================================
   Feature Components
   ========================================================================== */

/* Camera Interface */
{}

/* Chat Interface */
{}

/* Homepage */
{}

/* ==========================================================================
   Utility Systems
   ========================================================================== */

/* Responsive Design */
{}

/* Performance Optimizations */
{}

/* Accessibility Features */
{}

/* ==========================================================================
   Global Utilities
   ========================================================================== */

/* Spacing utilities */
.m-0 {{ margin: 0; }}
.m-xs {{ margin: var(--space-xs); }}
.m-sm {{ margin: var(--space-sm); }}
.m-md {{ margin: var(--space-md); }}
.m-lg {{ margin: var(--space-lg); }}
.m-xl {{ margin: var(--space-xl); }}
.m-2xl {{ margin: var(--space-2xl); }}

.mt-0 {{ margin-top: 0; }}
.mt-xs {{ margin-top: var(--space-xs); }}
.mt-sm {{ margin-top: var(--space-sm); }}
.mt-md {{ margin-top: var(--space-md); }}
.mt-lg {{ margin-top: var(--space-lg); }}
.mt-xl {{ margin-top: var(--space-xl); }}
.mt-2xl {{ margin-top: var(--space-2xl); }}

.mb-0 {{ margin-bottom: 0; }}
.mb-xs {{ margin-bottom: var(--space-xs); }}
.mb-sm {{ margin-bottom: var(--space-sm); }}
.mb-md {{ margin-bottom: var(--space-md); }}
.mb-lg {{ margin-bottom: var(--space-lg); }}
.mb-xl {{ margin-bottom: var(--space-xl); }}
.mb-2xl {{ margin-bottom: var(--space-2xl); }}

.ml-0 {{ margin-left: 0; }}
.ml-xs {{ margin-left: var(--space-xs); }}
.ml-sm {{ margin-left: var(--space-sm); }}
.ml-md {{ margin-left: var(--space-md); }}
.ml-lg {{ margin-left: var(--space-lg); }}
.ml-xl {{ margin-left: var(--space-xl); }}
.ml-2xl {{ margin-left: var(--space-2xl); }}

.mr-0 {{ margin-right: 0; }}
.mr-xs {{ margin-right: var(--space-xs); }}
.mr-sm {{ margin-right: var(--space-sm); }}
.mr-md {{ margin-right: var(--space-md); }}
.mr-lg {{ margin-right: var(--space-lg); }}
.mr-xl {{ margin-right: var(--space-xl); }}
.mr-2xl {{ margin-right: var(--space-2xl); }}

.p-0 {{ padding: 0; }}
.p-xs {{ padding: var(--space-xs); }}
.p-sm {{ padding: var(--space-sm); }}
.p-md {{ padding: var(--space-md); }}
.p-lg {{ padding: var(--space-lg); }}
.p-xl {{ padding: var(--space-xl); }}
.p-2xl {{ padding: var(--space-2xl); }}

.pt-0 {{ padding-top: 0; }}
.pt-xs {{ padding-top: var(--space-xs); }}
.pt-sm {{ padding-top: var(--space-sm); }}
.pt-md {{ padding-top: var(--space-md); }}
.pt-lg {{ padding-top: var(--space-lg); }}
.pt-xl {{ padding-top: var(--space-xl); }}
.pt-2xl {{ padding-top: var(--space-2xl); }}

.pb-0 {{ padding-bottom: 0; }}
.pb-xs {{ padding-bottom: var(--space-xs); }}
.pb-sm {{ padding-bottom: var(--space-sm); }}
.pb-md {{ padding-bottom: var(--space-md); }}
.pb-lg {{ padding-bottom: var(--space-lg); }}
.pb-xl {{ padding-bottom: var(--space-xl); }}
.pb-2xl {{ padding-bottom: var(--space-2xl); }}

.pl-0 {{ padding-left: 0; }}
.pl-xs {{ padding-left: var(--space-xs); }}
.pl-sm {{ padding-left: var(--space-sm); }}
.pl-md {{ padding-left: var(--space-md); }}
.pl-lg {{ padding-left: var(--space-lg); }}
.pl-xl {{ padding-left: var(--space-xl); }}
.pl-2xl {{ padding-left: var(--space-2xl); }}

.pr-0 {{ padding-right: 0; }}
.pr-xs {{ padding-right: var(--space-xs); }}
.pr-sm {{ padding-right: var(--space-sm); }}
.pr-md {{ padding-right: var(--space-md); }}
.pr-lg {{ padding-right: var(--space-lg); }}
.pr-xl {{ padding-right: var(--space-xl); }}
.pr-2xl {{ padding-right: var(--space-2xl); }}

/* Display utilities */
.block {{ display: block; }}
.inline-block {{ display: inline-block; }}
.inline {{ display: inline; }}
.flex {{ display: flex; }}
.inline-flex {{ display: inline-flex; }}
.grid {{ display: grid; }}
.inline-grid {{ display: inline-grid; }}
.hidden {{ display: none; }}

/* Flexbox utilities */
.flex-row {{ flex-direction: row; }}
.flex-col {{ flex-direction: column; }}
.flex-wrap {{ flex-wrap: wrap; }}
.flex-nowrap {{ flex-wrap: nowrap; }}

.items-start {{ align-items: flex-start; }}
.items-center {{ align-items: center; }}
.items-end {{ align-items: flex-end; }}
.items-stretch {{ align-items: stretch; }}

.justify-start {{ justify-content: flex-start; }}
.justify-center {{ justify-content: center; }}
.justify-end {{ justify-content: flex-end; }}
.justify-between {{ justify-content: space-between; }}
.justify-around {{ justify-content: space-around; }}
.justify-evenly {{ justify-content: space-evenly; }}

.flex-1 {{ flex: 1 1 0%; }}
.flex-auto {{ flex: 1 1 auto; }}
.flex-initial {{ flex: 0 1 auto; }}
.flex-none {{ flex: none; }}

/* Text utilities */
.text-left {{ text-align: left; }}
.text-center {{ text-align: center; }}
.text-right {{ text-align: right; }}
.text-justify {{ text-align: justify; }}

.text-xs {{ font-size: var(--text-xs); }}
.text-sm {{ font-size: var(--text-sm); }}
.text-base {{ font-size: var(--text-base); }}
.text-lg {{ font-size: var(--text-lg); }}
.text-xl {{ font-size: var(--text-xl); }}
.text-2xl {{ font-size: var(--text-2xl); }}
.text-3xl {{ font-size: var(--text-3xl); }}
.text-4xl {{ font-size: var(--text-4xl); }}
.text-5xl {{ font-size: var(--text-5xl); }}

.font-light {{ font-weight: var(--weight-light); }}
.font-normal {{ font-weight: var(--weight-normal); }}
.font-medium {{ font-weight: var(--weight-medium); }}
.font-semibold {{ font-weight: var(--weight-semibold); }}
.font-bold {{ font-weight: var(--weight-bold); }}
.font-extrabold {{ font-weight: var(--weight-extrabold); }}

/* Color utilities */
.text-primary {{ color: var(--color-text-primary); }}
.text-secondary {{ color: var(--color-text-secondary); }}
.text-disabled {{ color: var(--color-text-disabled); }}
.text-inverse {{ color: var(--color-text-inverse); }}
.text-success {{ color: var(--color-success); }}
.text-warning {{ color: var(--color-warning); }}
.text-error {{ color: var(--color-error); }}
.text-info {{ color: var(--color-info); }}

.bg-primary {{ background-color: var(--color-primary-electric-blue); }}
.bg-secondary {{ background-color: var(--color-primary-vibrant-orange); }}
.bg-accent {{ background-color: var(--color-primary-energetic-pink); }}
.bg-success {{ background-color: var(--color-success); }}
.bg-warning {{ background-color: var(--color-warning); }}
.bg-error {{ background-color: var(--color-error); }}
.bg-info {{ background-color: var(--color-info); }}
.bg-light {{ background-color: var(--color-bg-light); }}
.bg-dark {{ background-color: var(--color-bg-dark); }}
.bg-surface {{ background-color: var(--color-surface-light); }}

/* Border utilities */
.border {{ border: 1px solid var(--color-text-secondary); }}
.border-0 {{ border: 0; }}
.border-t {{ border-top: 1px solid var(--color-text-secondary); }}
.border-r {{ border-right: 1px solid var(--color-text-secondary); }}
.border-b {{ border-bottom: 1px solid var(--color-text-secondary); }}
.border-l {{ border-left: 1px solid var(--color-text-secondary); }}

.rounded-none {{ border-radius: var(--radius-none); }}
.rounded-sm {{ border-radius: var(--radius-sm); }}
.rounded {{ border-radius: var(--radius-md); }}
.rounded-lg {{ border-radius: var(--radius-lg); }}
.rounded-xl {{ border-radius: var(--radius-xl); }}
.rounded-2xl {{ border-radius: var(--radius-2xl); }}
.rounded-full {{ border-radius: var(--radius-full); }}

/* Shadow utilities */
.shadow-none {{ box-shadow: var(--shadow-none); }}
.shadow-sm {{ box-shadow: var(--shadow-sm); }}
.shadow {{ box-shadow: var(--shadow-md); }}
.shadow-lg {{ box-shadow: var(--shadow-lg); }}
.shadow-xl {{ box-shadow: var(--shadow-xl); }}
.shadow-2xl {{ box-shadow: var(--shadow-2xl); }}
.shadow-inner {{ box-shadow: var(--shadow-inner); }}

/* Width and height utilities */
.w-auto {{ width: auto; }}
.w-full {{ width: 100%; }}
.w-screen {{ width: 100vw; }}
.w-min {{ width: min-content; }}
.w-max {{ width: max-content; }}

.h-auto {{ height: auto; }}
.h-full {{ height: 100%; }}
.h-screen {{ height: 100vh; }}
.h-min {{ height: min-content; }}
.h-max {{ height: max-content; }}

/* Position utilities */
.static {{ position: static; }}
.fixed {{ position: fixed; }}
.absolute {{ position: absolute; }}
.relative {{ position: relative; }}
.sticky {{ position: sticky; }}

.top-0 {{ top: 0; }}
.right-0 {{ right: 0; }}
.bottom-0 {{ bottom: 0; }}
.left-0 {{ left: 0; }}

/* Z-index utilities */
.z-0 {{ z-index: 0; }}
.z-10 {{ z-index: 10; }}
.z-20 {{ z-index: 20; }}
.z-30 {{ z-index: 30; }}
.z-40 {{ z-index: 40; }}
.z-50 {{ z-index: 50; }}
.z-auto {{ z-index: auto; }}

/* Overflow utilities */
.overflow-auto {{ overflow: auto; }}
.overflow-hidden {{ overflow: hidden; }}
.overflow-visible {{ overflow: visible; }}
.overflow-scroll {{ overflow: scroll; }}
.overflow-x-auto {{ overflow-x: auto; }}
.overflow-y-auto {{ overflow-y: auto; }}
.overflow-x-hidden {{ overflow-x: hidden; }}
.overflow-y-hidden {{ overflow-y: hidden; }}

/* Cursor utilities */
.cursor-auto {{ cursor: auto; }}
.cursor-default {{ cursor: default; }}
.cursor-pointer {{ cursor: pointer; }}
.cursor-wait {{ cursor: wait; }}
.cursor-text {{ cursor: text; }}
.cursor-move {{ cursor: move; }}
.cursor-help {{ cursor: help; }}
.cursor-not-allowed {{ cursor: not-allowed; }}

/* Select utilities */
.select-none {{ user-select: none; }}
.select-text {{ user-select: text; }}
.select-all {{ user-select: all; }}
.select-auto {{ user-select: auto; }}

/* Pointer events */
.pointer-events-none {{ pointer-events: none; }}
.pointer-events-auto {{ pointer-events: auto; }}

/* ==========================================================================
   Print Styles
   ========================================================================== */

@media print {{
  .no-print {{
    display: none !important;
  }}

  .print-only {{
    display: block !important;
  }}

  * {{
    background: white !important;
    color: black !important;
    box-shadow: none !important;
    text-shadow: none !important;
  }}

  a, a:visited {{
    text-decoration: underline;
  }}

  a[href]:after {{
    content: " (" attr(href) ")";
  }}

  abbr[title]:after {{
    content: " (" attr(title) ")";
  }}

  .bento-grid {{
    display: block !important;
  }}

  .bento-card {{
    break-inside: avoid;
    margin-bottom: 1rem;
    border: 1px solid #ccc;
  }}
}}

/* ==========================================================================
   End of CSS Bundle
   ========================================================================== */"#,
        generate_design_tokens_css(&tokens),
        generate_typography_css(&crate::styles::TypographyScale::default()),
        generate_bento_css(),
        generate_gradient_button_css(),
        generate_status_card_css(),
        generate_loading_css(),
        generate_camera_css(),
        generate_chat_css(),
        generate_homepage_css(),
        generate_responsive_css(),
        generate_performance_css(),
        generate_accessibility_css(),
    )
}

// Generate minified CSS bundle for production
pub fn generate_minified_css_bundle() -> String {
    let full_css = generate_complete_css_bundle();

    // Simple minification - remove comments, extra whitespace, and newlines
    full_css
        .lines()
        .filter(|line| !line.trim().starts_with("/*") && !line.trim().is_empty())
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("")
        .replace("  ", " ")
        .replace(" {", "{")
        .replace("{ ", "{")
        .replace(" }", "}")
        .replace("; ", ";")
        .replace(": ", ":")
}

// Generate CSS bundle with custom configuration
pub fn generate_custom_css_bundle(
    include_utilities: bool,
    include_components: bool,
    include_responsive: bool,
    include_accessibility: bool,
) -> String {
    let tokens = DesignTokens::default();
    let mut css_parts = Vec::new();

    // Always include base styles and tokens
    css_parts.push(generate_design_tokens_css(&tokens));
    css_parts.push(generate_typography_css(&crate::styles::TypographyScale::default()));
    css_parts.push(generate_bento_css());

    if include_components {
        css_parts.push(generate_gradient_button_css());
        css_parts.push(generate_status_card_css());
        css_parts.push(generate_loading_css());
        css_parts.push(generate_camera_css());
        css_parts.push(generate_chat_css());
        css_parts.push(generate_homepage_css());
    }

    if include_responsive {
        css_parts.push(generate_responsive_css());
    }

    if include_accessibility {
        css_parts.push(generate_accessibility_css());
    }

    if include_utilities {
        css_parts.push(generate_performance_css());
    }

    css_parts.join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_css_bundle_generation() {
        let css = generate_complete_css_bundle();
        assert!(css.contains("AI4Thai Crop Guardian"));
        assert!(css.contains("2025 Design System"));
        assert!(css.contains("--color-primary-electric-blue"));
        assert!(css.contains("bento-grid"));
        assert!(css.contains("gradient-btn"));
    }

    #[test]
    fn test_minified_css_bundle() {
        let minified = generate_minified_css_bundle();
        assert!(!minified.contains("/*"));
        assert!(!minified.contains("\n"));
    }

    #[test]
    fn test_custom_css_bundle() {
        let css = generate_custom_css_bundle(true, true, true, true);
        assert!(css.contains("--color-primary-electric-blue"));
        assert!(css.contains("gradient-btn"));
        assert!(css.contains("responsive-grid"));
        assert!(css.contains("sr-only"));
    }
}
