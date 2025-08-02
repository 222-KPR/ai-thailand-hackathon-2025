// Gradient Button Component - 2025 Design System
// Interactive button with dopamine colors and micro-interactions

use yew::prelude::*;
use crate::styles::{use_theme, ColorPalette};

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Custom { primary: String, secondary: String },
}

#[derive(Properties, PartialEq)]
pub struct GradientButtonProps {
    pub children: Children,
    pub variant: Option<ButtonVariant>,
    pub size: Option<ButtonSize>,
    pub onclick: Option<Callback<MouseEvent>>,
    pub disabled: Option<bool>,
    pub loading: Option<bool>,
    pub full_width: Option<bool>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub icon: Option<String>,
    pub icon_position: Option<String>, // "left" or "right"
}

#[function_component(GradientButton)]
pub fn gradient_button(props: &GradientButtonProps) -> Html {
    let theme = use_theme();
    let colors = &theme.colors;
    
    let variant = props.variant.as_ref().unwrap_or(&ButtonVariant::Primary);
    let size = props.size.as_ref().unwrap_or(&ButtonSize::Medium);
    let disabled = props.disabled.unwrap_or(false);
    let loading = props.loading.unwrap_or(false);
    let full_width = props.full_width.unwrap_or(false);
    let icon_position = props.icon_position.as_deref().unwrap_or("left");
    
    // Determine gradient colors based on variant
    let (primary_color, secondary_color, text_color, shadow_color) = match variant {
        ButtonVariant::Primary => (
            colors.primary_electric_blue,
            colors.primary_energetic_pink,
            colors.text_inverse,
            "rgba(0, 102, 255, 0.3)"
        ),
        ButtonVariant::Secondary => (
            colors.surface_light,
            colors.surface_light,
            colors.text_primary,
            "rgba(0, 0, 0, 0.1)"
        ),
        ButtonVariant::Success => (
            colors.accent_lime_green,
            colors.primary_electric_blue,
            colors.text_inverse,
            "rgba(50, 215, 75, 0.3)"
        ),
        ButtonVariant::Warning => (
            colors.accent_yellow,
            colors.primary_vibrant_orange,
            colors.text_primary,
            "rgba(255, 214, 10, 0.3)"
        ),
        ButtonVariant::Error => (
            colors.error,
            colors.primary_energetic_pink,
            colors.text_inverse,
            "rgba(255, 69, 58, 0.3)"
        ),
        ButtonVariant::Custom { primary, secondary } => (
            primary.as_str(),
            secondary.as_str(),
            colors.text_inverse,
            "rgba(0, 0, 0, 0.2)"
        ),
    };
    
    // Size-based styling
    let (padding, font_size, min_height, border_radius) = match size {
        ButtonSize::Small => ("0.5rem 1rem", "0.75rem", "36px", "8px"),
        ButtonSize::Medium => ("1rem 1.5rem", "0.875rem", "44px", "12px"),
        ButtonSize::Large => ("1.25rem 2rem", "1rem", "52px", "16px"),
    };
    
    // Build gradient background
    let background = if primary_color == secondary_color {
        format!("background: {};", primary_color)
    } else {
        format!("background: linear-gradient(135deg, {}, {});", primary_color, secondary_color)
    };
    
    // Build complete style
    let button_style = format!(
        "{}
         color: {};
         padding: {};
         font-size: {};
         min-height: {};
         border-radius: {};
         box-shadow: 0 4px 15px {};
         border: none;
         font-family: var(--font-body);
         font-weight: var(--weight-semibold);
         cursor: {};
         transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
         position: relative;
         overflow: hidden;
         display: inline-flex;
         align-items: center;
         justify-content: center;
         gap: 0.5rem;
         text-decoration: none;
         opacity: {};
         width: {};
         {}",
        background,
        text_color,
        padding,
        font_size,
        min_height,
        border_radius,
        shadow_color,
        if disabled || loading { "not-allowed" } else { "pointer" },
        if disabled { "0.5" } else { "1" },
        if full_width { "100%" } else { "auto" },
        props.style.as_deref().unwrap_or("")
    );
    
    let button_classes = classes!(
        "gradient-btn",
        match size {
            ButtonSize::Small => "btn-sm",
            ButtonSize::Medium => "btn-md",
            ButtonSize::Large => "btn-lg",
        },
        match variant {
            ButtonVariant::Primary => "btn-primary",
            ButtonVariant::Secondary => "btn-secondary",
            ButtonVariant::Success => "btn-success",
            ButtonVariant::Warning => "btn-warning",
            ButtonVariant::Error => "btn-error",
            ButtonVariant::Custom { .. } => "btn-custom",
        },
        if full_width { "btn-full-width" } else { "" },
        if disabled { "btn-disabled" } else { "" },
        if loading { "btn-loading" } else { "" },
        props.class.clone()
    );
    
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            if !disabled && !loading {
                if let Some(onclick) = &onclick {
                    onclick.emit(e);
                }
            }
        })
    };
    
    html! {
        <button 
            class={button_classes}
            style={button_style}
            {onclick}
            disabled={disabled || loading}
        >
            // Shimmer effect overlay
            <div class="btn-shimmer"></div>
            
            // Loading spinner
            if loading {
                <div class="btn-loading-spinner"></div>
            }
            
            // Icon (left position)
            if let Some(icon) = &props.icon {
                if icon_position == "left" {
                    <span class="btn-icon btn-icon-left">{icon}</span>
                }
            }
            
            // Button content
            <span class="btn-content">
                { for props.children.iter() }
            </span>
            
            // Icon (right position)
            if let Some(icon) = &props.icon {
                if icon_position == "right" {
                    <span class="btn-icon btn-icon-right">{icon}</span>
                }
            }
        </button>
    }
}

// CSS for gradient button (to be included in main.scss)
pub fn generate_gradient_button_css() -> String {
    r#"/* Gradient Button Styles */
.gradient-btn {
  position: relative;
  overflow: hidden;
  transform: translateY(0px);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.gradient-btn:hover:not(.btn-disabled):not(.btn-loading) {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15) !important;
}

.gradient-btn:active:not(.btn-disabled):not(.btn-loading) {
  transform: translateY(0px);
}

.gradient-btn:focus {
  outline: 2px solid var(--color-primary-electric-blue);
  outline-offset: 2px;
}

/* Shimmer effect */
.btn-shimmer {
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s;
  pointer-events: none;
}

.gradient-btn:hover:not(.btn-disabled):not(.btn-loading) .btn-shimmer {
  left: 100%;
}

/* Loading spinner */
.btn-loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid transparent;
  border-top: 2px solid currentColor;
  border-radius: 50%;
  animation: btn-spin 1s linear infinite;
}

@keyframes btn-spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* Icon styles */
.btn-icon {
  display: flex;
  align-items: center;
  font-size: 1.1em;
}

.btn-icon-left {
  margin-right: -0.25rem;
}

.btn-icon-right {
  margin-left: -0.25rem;
}

/* Content wrapper */
.btn-content {
  display: flex;
  align-items: center;
  white-space: nowrap;
}

/* Secondary button specific styles */
.btn-secondary {
  border: 2px solid var(--color-primary-electric-blue) !important;
}

.btn-secondary:hover:not(.btn-disabled):not(.btn-loading) {
  background: var(--color-primary-electric-blue) !important;
  color: var(--color-text-inverse) !important;
}

/* Disabled state */
.btn-disabled {
  cursor: not-allowed !important;
  opacity: 0.5 !important;
}

.btn-disabled:hover {
  transform: none !important;
  box-shadow: inherit !important;
}

/* Loading state */
.btn-loading {
  cursor: wait !important;
}

.btn-loading .btn-content {
  opacity: 0.7;
}

/* Full width */
.btn-full-width {
  width: 100%;
  justify-content: center;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .gradient-btn:not(.btn-sm) {
    width: 100%;
    justify-content: center;
  }
  
  .btn-sm {
    width: auto;
  }
}

/* Accessibility */
@media (prefers-reduced-motion: reduce) {
  .gradient-btn {
    transition: none;
  }
  
  .gradient-btn:hover:not(.btn-disabled):not(.btn-loading) {
    transform: none;
  }
  
  .btn-shimmer {
    display: none;
  }
  
  .btn-loading-spinner {
    animation: none;
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .gradient-btn {
    border: 2px solid currentColor !important;
  }
}"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_variant_colors() {
        // Test that different variants produce different color schemes
        let primary = ButtonVariant::Primary;
        let secondary = ButtonVariant::Secondary;
        assert_ne!(primary, secondary);
    }

    #[test]
    fn test_button_size_values() {
        let small = ButtonSize::Small;
        let large = ButtonSize::Large;
        assert_ne!(small, large);
    }

    #[test]
    fn test_css_generation() {
        let css = generate_gradient_button_css();
        assert!(css.contains("gradient-btn"));
        assert!(css.contains("btn-shimmer"));
        assert!(css.contains("@keyframes"));
    }
}
