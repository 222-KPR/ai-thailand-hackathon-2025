// Copyright (c) 2025 AI4Thai Crop Guardian
// Licensed under the MIT License

//! Design System Styles Module
//!
//! This module exports all design system components including colors,
//! typography, spacing, and layout utilities.

pub mod colors;
pub mod typography;
pub mod spacing;

pub use colors::*;
pub use typography::*;
pub use spacing::*;

/// Design system utilities and helpers
pub struct DesignSystem;

impl DesignSystem {
    /// Generate complete CSS custom properties for the design system
    pub fn css_variables() -> String {
        format!(
            "{}{}{}",
            colors::ColorPalette::css_variables(),
            typography::TypographyCSS::css_variables(),
            spacing::SpacingCSS::css_variables()
        )
    }

    /// Generate base CSS reset and typography styles
    pub fn base_styles() -> String {
        format!(
            r#"
/* CSS Reset and Base Styles */
*, *::before, *::after {{
  box-sizing: border-box;
}}

html {{
  font-size: 16px;
  line-height: 1.5;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-rendering: optimizeLegibility;
}}

body {{
  margin: 0;
  padding: 0;
  font-family: var(--font-body);
  font-size: var(--font-size-body);
  font-weight: var(--font-weight-regular);
  line-height: var(--line-height-body);
  color: var(--text-primary-light);
  background-color: var(--bg-light);
  overflow-x: hidden;
}}

/* Typography Base Styles */
h1, h2, h3, h4, h5, h6 {{
  margin: 0;
  font-family: var(--font-heading);
  font-weight: var(--font-weight-bold);
  line-height: 1.2;
}}

h1 {{ font-size: var(--font-size-h1); }}
h2 {{ font-size: var(--font-size-h2); }}
h3 {{ font-size: var(--font-size-h3); }}
h4 {{ font-size: var(--font-size-h4); }}
h5 {{ font-size: var(--font-size-h5); }}
h6 {{ font-size: var(--font-size-h6); }}

p {{
  margin: 0 0 1rem 0;
  font-size: var(--font-size-body);
  line-height: var(--line-height-body);
}}

/* Link Styles */
a {{
  color: var(--color-primary-electric-blue);
  text-decoration: none;
  transition: color 0.2s ease;
}}

a:hover {{
  color: var(--color-primary-vibrant-orange);
  text-decoration: underline;
}}

/* Button Reset */
button {{
  border: none;
  background: none;
  padding: 0;
  margin: 0;
  font: inherit;
  cursor: pointer;
}}

/* Form Elements */
input, textarea, select {{
  font-family: inherit;
  font-size: inherit;
  border: 1px solid var(--border-light);
  border-radius: var(--radius-md);
  padding: var(--spacing-md);
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
}}

input:focus, textarea:focus, select:focus {{
  outline: none;
  border-color: var(--color-primary-electric-blue);
  box-shadow: 0 0 0 3px rgba(0, 102, 255, 0.1);
}}

/* Utility Classes */
.visually-hidden {{
  position: absolute !important;
  width: 1px !important;
  height: 1px !important;
  padding: 0 !important;
  margin: -1px !important;
  overflow: hidden !important;
  clip: rect(0, 0, 0, 0) !important;
  white-space: nowrap !important;
  border: 0 !important;
}}

.text-center {{ text-align: center; }}
.text-left {{ text-align: left; }}
.text-right {{ text-align: right; }}

.flex {{ display: flex; }}
.flex-center {{ display: flex; align-items: center; justify-content: center; }}
.flex-between {{ display: flex; align-items: center; justify-content: space-between; }}
.flex-column {{ display: flex; flex-direction: column; }}

.grid {{ display: grid; }}
.grid-center {{ display: grid; place-items: center; }}

/* Responsive Utilities */
.mobile-only {{
  @media (min-width: var(--breakpoint-md)) {{
    display: none !important;
  }}
}}

.desktop-only {{
  @media (max-width: calc(var(--breakpoint-md) - 1px)) {{
    display: none !important;
  }}
}}

/* Thai Language Styles */
.thai-text {{
  font-family: var(--font-thai);
  line-height: calc(var(--line-height-body) * 1.2);
  letter-spacing: var(--letter-spacing-thai);
}}

/* Animation Utilities */
.animate-fade-in {{
  animation: fadeIn 0.3s ease-in-out;
}}

.animate-slide-up {{
  animation: slideUp 0.3s ease-out;
}}

.animate-scale-in {{
  animation: scaleIn 0.2s ease-out;
}}

@keyframes fadeIn {{
  from {{ opacity: 0; }}
  to {{ opacity: 1; }}
}}

@keyframes slideUp {{
  from {{ transform: translateY(20px); opacity: 0; }}
  to {{ transform: translateY(0); opacity: 1; }}
}}

@keyframes scaleIn {{
  from {{ transform: scale(0.9); opacity: 0; }}
  to {{ transform: scale(1); opacity: 1; }}
}}

/* Dark Mode Support */
@media (prefers-color-scheme: dark) {{
  body {{
    color: var(--text-primary-dark);
    background-color: var(--bg-dark);
  }}

  input, textarea, select {{
    border-color: var(--border-dark);
    background-color: var(--surface-dark);
    color: var(--text-primary-dark);
  }}
}}

/* High Contrast Mode Support */
@media (prefers-contrast: high) {{
  * {{
    border-color: currentColor !important;
  }}
}}

/* Reduced Motion Support */
@media (prefers-reduced-motion: reduce) {{
  *, *::before, *::after {{
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }}
}}

/* Print Styles */
@media print {{
  * {{
    background: transparent !important;
    color: black !important;
    box-shadow: none !important;
    text-shadow: none !important;
  }}

  a, a:visited {{
    text-decoration: underline;
  }}

  .no-print {{
    display: none !important;
  }}
}}
"#
        )
    }
}
