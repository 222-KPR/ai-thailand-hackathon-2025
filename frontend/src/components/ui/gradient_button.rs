// Copyright (c) 2025 AI4Thai Crop Guardian
// Licensed under the MIT License

//! Gradient Button Component
//!
//! This module implements vibrant gradient buttons with dopamine colors,
//! micro-interactions, and accessibility features.

use yew::prelude::*;
use crate::styles::{colors::*, spacing::*, typography::*};

/// Button size variants
#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    /// Small button (32px height)
    Small,
    /// Medium button (40px height) - default
    Medium,
    /// Large button (48px height)
    Large,
    /// Extra large button (56px height)
    ExtraLarge,
}

impl ButtonSize {
    /// Get the height for the button size
    pub fn height(&self) -> &'static str {
        match self {
            ButtonSize::Small => "2rem",
            ButtonSize::Medium => "2.5rem",
            ButtonSize::Large => "3rem",
            ButtonSize::ExtraLarge => "3.5rem",
        }
    }

    /// Get the padding for the button size
    pub fn padding(&self) -> &'static str {
        match self {
            ButtonSize::Small => "0.5rem 1rem",
            ButtonSize::Medium => "0.75rem 1.5rem",
            ButtonSize::Large => "1rem 2rem",
            ButtonSize::ExtraLarge => "1.25rem 2.5rem",
        }
    }

    /// Get the font size for the button size
    pub fn font_size(&self) -> &'static str {
        match self {
            ButtonSize::Small => TypographyScale::CAPTION_SIZE,
            ButtonSize::Medium => TypographyScale::BUTTON_SIZE,
            ButtonSize::Large => TypographyScale::BODY_SIZE,
            ButtonSize::ExtraLarge => TypographyScale::BODY_LARGE_SIZE,
        }
    }
}

/// Button variant styles with dopamine colors
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    /// Primary button with electric blue gradient
    Primary,
    /// Secondary button with vibrant orange gradient
    Secondary,
    /// Accent button with energetic pink gradient
    Accent,
    /// Success button with lime green gradient
    Success,
    /// Warning button with yellow gradient
    Warning,
    /// Error button with red gradient
    Error,
    /// Ghost button with transparent background
    Ghost,
    /// Outline button with border only
    Outline,
    /// Custom gradient with specified colors
    Custom { from: String, to: String },
}

impl ButtonVariant {
    /// Get the background gradient for the variant
    pub fn background(&self) -> String {
        match self {
            ButtonVariant::Primary => format!(
                "linear-gradient(135deg, {} 0%, {} 100%)",
                PrimaryColors::ELECTRIC_BLUE,
                "#0052CC"
            ),
            ButtonVariant::Secondary => format!(
                "linear-gradient(135deg, {} 0%, {} 100%)",
                PrimaryColors::VIBRANT_ORANGE,
                "#E55A2B"
            ),
            ButtonVariant::Accent => format!(
                "linear-gradient(135deg, {} 0%, {} 100%)",
                PrimaryColors::ENERGETIC_PINK,
                "#E6177A"
            ),
            ButtonVariant::Success => format!(
                "linear-gradient(135deg, {} 0%, {} 100%)",
                AccentColors::LIME_GREEN,
                "#28A745"
            ),
            ButtonVariant::Warning => format!(
                "linear-gradient(135deg, {} 0%, {} 100%)",
                AccentColors::YELLOW,
                "#E6C200"
            ),
            ButtonVariant::Error => format!(
                "linear-gradient(135deg, {} 0%, {} 100%)",
                SemanticColors::ERROR,
                "#CC2E24"
            ),
            ButtonVariant::Ghost => "transparent".to_string(),
            ButtonVariant::Outline => "transparent".to_string(),
            ButtonVariant::Custom { from, to } => format!(
                "linear-gradient(135deg, {from} 0%, {to} 100%)"
            ),
        }
    }

    /// Get the text color for the variant
    pub fn color(&self) -> &'static str {
        match self {
            ButtonVariant::Ghost => TextColors::PRIMARY_LIGHT,
            ButtonVariant::Outline => PrimaryColors::ELECTRIC_BLUE,
            _ => SurfaceColors::SURFACE_LIGHT,
        }
    }

    /// Get the border style for the variant
    pub fn border(&self) -> String {
        match self {
            ButtonVariant::Outline => format!("2px solid {}", PrimaryColors::ELECTRIC_BLUE),
            _ => "none".to_string(),
        }
    }

    /// Get the hover background for the variant
    pub fn hover_background(&self) -> String {
        match self {
            ButtonVariant::Primary => format!(
                "linear-gradient(135deg, {} 0%, {} 100%)",
                "#0052CC",
                PrimaryColors::ELECTRIC_BLUE
            ),
            ButtonVariant::Ghost => format!("rgba({}, 0.1)", "0, 102, 255"),
            ButtonVariant::Outline => format!("rgba({}, 0.1)", "0, 102, 255"),
            _ => self.background(),
        }
    }
}

/// Properties for the gradient button component
#[derive(Properties, PartialEq)]
pub struct GradientButtonProps {
    /// Button text content
    pub children: Children,

    /// Button size variant
    #[prop_or(ButtonSize::Medium)]
    pub size: ButtonSize,

    /// Button style variant
    #[prop_or(ButtonVariant::Primary)]
    pub variant: ButtonVariant,

    /// Whether the button is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Whether the button is in loading state
    #[prop_or(false)]
    pub loading: bool,

    /// Whether the button should take full width
    #[prop_or(false)]
    pub full_width: bool,

    /// Optional icon to display before text
    #[prop_or_default]
    pub icon: Option<Html>,

    /// Optional icon to display after text
    #[prop_or_default]
    pub end_icon: Option<Html>,

    /// Click handler
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    /// Button type for forms
    #[prop_or_else(|| "button".to_string())]
    pub button_type: String,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// ARIA label for accessibility
    #[prop_or_default]
    pub aria_label: Option<String>,
}

/// Gradient button component with dopamine colors and micro-interactions
#[function_component(GradientButton)]
pub fn gradient_button(props: &GradientButtonProps) -> Html {
    let shimmer_animation = r#"
        @keyframes shimmer {
            0% { background-position: -200% 0; }
            100% { background-position: 200% 0; }
        }
    "#;

    let loading_animation = r#"
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
    "#;

    let base_styles = format!(
        "position: relative;
         display: inline-flex;
         align-items: center;
         justify-content: center;
         gap: {};
         height: {};
         padding: {};
         font-family: {};
         font-size: {};
         font-weight: {};
         line-height: {};
         letter-spacing: {};
         text-decoration: none;
         border: {};
         border-radius: {};
         background: {};
         color: {};
         cursor: {};
         transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
         overflow: hidden;
         user-select: none;
         white-space: nowrap;
         {}

         /* Shimmer effect overlay */
         &::before {{
           content: '';
           position: absolute;
           top: 0;
           left: 0;
           right: 0;
           bottom: 0;
           background: linear-gradient(
             90deg,
             transparent 0%,
             rgba(255, 255, 255, 0.2) 50%,
             transparent 100%
           );
           background-size: 200% 100%;
           animation: shimmer 2s infinite;
           opacity: 0;
           transition: opacity 0.2s ease;
         }}

         &:hover::before {{
           opacity: 1;
         }}

         &:hover {{
           transform: translateY(-1px);
           box-shadow: {};
           background: {};
         }}

         &:active {{
           transform: translateY(0);
           transition: transform 0.1s ease;
         }}

         &:focus-visible {{
           outline: 2px solid {};
           outline-offset: 2px;
         }}

         &:disabled {{
           opacity: 0.6;
           cursor: not-allowed;
           transform: none;
         }}

         &:disabled:hover {{
           transform: none;
           box-shadow: {};
         }}",
        Spacing::MD,
        props.size.height(),
        props.size.padding(),
        FontFamilies::BODY,
        props.size.font_size(),
        FontWeights::MEDIUM,
        TypographyScale::BUTTON_LINE_HEIGHT,
        TypographyScale::BUTTON_LETTER_SPACING,
        props.variant.border(),
        BorderRadius::MD,
        props.variant.background(),
        props.variant.color(),
        if props.disabled { "not-allowed" } else { "pointer" },
        if props.full_width { "width: 100%;" } else { "" },
        Shadows::LG,
        props.variant.hover_background(),
        PrimaryColors::ELECTRIC_BLUE,
        Shadows::MD,
    );

    let loading_spinner = if props.loading {
        html! {
            <div style="width: 1rem; height: 1rem; border: 2px solid currentColor;
                 border-top-color: transparent; border-radius: 50%;
                 animation: spin 1s linear infinite;">
            </div>
        }
    } else {
        html! {}
    };

    let content = if props.loading {
        html! {
            <>
                { loading_spinner }
                <span style="opacity: 0.7;">{ "Loading..." }</span>
            </>
        }
    } else {
        html! {
            <>
                if let Some(icon) = &props.icon {
                    <span class="button-icon-start">{ icon.clone() }</span>
                }
                <span class="button-text">{ for props.children.iter() }</span>
                if let Some(end_icon) = &props.end_icon {
                    <span class="button-icon-end">{ end_icon.clone() }</span>
                }
            </>
        }
    };

    html! {
        <>
            <style>{ shimmer_animation }{ loading_animation }</style>
            <button
                type={props.button_type.clone()}
                class={classes!("gradient-button", props.class.clone())}
                style={base_styles}
                onclick={props.onclick.clone()}
                disabled={props.disabled || props.loading}
                aria-label={props.aria_label.clone()}
            >
                { content }
            </button>
        </>
    }
}

/// Button group component for related actions
#[derive(Properties, PartialEq)]
pub struct ButtonGroupProps {
    /// Button components to group
    pub children: Children,

    /// Gap between buttons
    #[prop_or_else(|| Spacing::MD.to_string())]
    pub gap: String,

    /// Whether buttons should stack vertically on mobile
    #[prop_or(true)]
    pub responsive: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Button group component for organizing related buttons
#[function_component(ButtonGroup)]
pub fn button_group(props: &ButtonGroupProps) -> Html {
    let responsive_styles = if props.responsive {
        format!(
            "@media (max-width: {}) {{
               flex-direction: column;
               align-items: stretch;
             }}",
            Breakpoints::SM
        )
    } else {
        String::new()
    };

    let style = format!(
        "display: flex;
         align-items: center;
         gap: {};
         {}",
        props.gap,
        responsive_styles
    );

    html! {
        <div class={classes!("button-group", props.class.clone())} style={style}>
            { for props.children.iter() }
        </div>
    }
}

/// Predefined button variants for common use cases
pub struct ButtonVariants;

impl ButtonVariants {
    /// Create a primary action button
    pub fn primary(text: &str, onclick: Callback<MouseEvent>) -> Html {
        html! {
            <GradientButton variant={ButtonVariant::Primary} onclick={onclick}>
                { text }
            </GradientButton>
        }
    }

    /// Create a secondary action button
    pub fn secondary(text: &str, onclick: Callback<MouseEvent>) -> Html {
        html! {
            <GradientButton variant={ButtonVariant::Secondary} onclick={onclick}>
                { text }
            </GradientButton>
        }
    }

    /// Create a button with icon
    pub fn with_icon(text: &str, icon: Html, onclick: Callback<MouseEvent>) -> Html {
        html! {
            <GradientButton icon={Some(icon)} onclick={onclick}>
                { text }
            </GradientButton>
        }
    }

    /// Create a loading button
    pub fn loading(text: &str) -> Html {
        html! {
            <GradientButton loading={true}>
                { text }
            </GradientButton>
        }
    }

    /// Create a full-width button
    pub fn full_width(text: &str, onclick: Callback<MouseEvent>) -> Html {
        html! {
            <GradientButton full_width={true} onclick={onclick}>
                { text }
            </GradientButton>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_size_properties() {
        assert_eq!(ButtonSize::Small.height(), "2rem");
        assert_eq!(ButtonSize::Medium.height(), "2.5rem");
        assert_eq!(ButtonSize::Large.height(), "3rem");
    }

    #[test]
    fn test_button_variant_colors() {
        assert!(ButtonVariant::Primary.background().contains("#0066FF"));
        assert_eq!(ButtonVariant::Primary.color(), "#FFFFFF");
        assert_eq!(ButtonVariant::Ghost.color(), "#1D1D1F");
    }

    #[test]
    fn test_button_variant_borders() {
        assert_eq!(ButtonVariant::Primary.border(), "none");
        assert!(ButtonVariant::Outline.border().contains("solid"));
    }
}
