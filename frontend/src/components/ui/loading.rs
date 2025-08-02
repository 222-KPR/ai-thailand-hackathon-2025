// Copyright (c) 2025 AI4Thai Crop Guardian
// Licensed under the MIT License

//! Loading Animation Components
//!
//! This module implements various loading animations with dopamine colors
//! to provide engaging feedback during async operations.

use yew::prelude::*;
use crate::styles::{colors::*, spacing::*, typography::*};

/// Loading animation variants
#[derive(Debug, Clone, PartialEq)]
pub enum LoadingVariant {
    /// Spinning circle loader
    Spinner,
    /// Bouncing dots loader
    Dots,
    /// Pulsing circle loader
    Pulse,
    /// Wave animation loader
    Wave,
    /// Progress bar loader
    Progress,
    /// Skeleton loading placeholder
    Skeleton,
}

/// Loading animation sizes
#[derive(Clone, PartialEq)]
pub enum LoadingSize {
    /// Small loader (16px)
    Small,
    /// Medium loader (24px)
    Medium,
    /// Large loader (32px)
    Large,
    /// Extra large loader (48px)
    ExtraLarge,
}

impl LoadingSize {
    /// Get the size value in pixels
    pub fn size(&self) -> &'static str {
        match self {
            LoadingSize::Small => "1rem",
            LoadingSize::Medium => "1.5rem",
            LoadingSize::Large => "2rem",
            LoadingSize::ExtraLarge => "3rem",
        }
    }
}

/// Properties for the loading component
#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    /// Loading animation variant
    #[prop_or(LoadingVariant::Spinner)]
    pub variant: LoadingVariant,

    /// Loading animation size
    #[prop_or(LoadingSize::Medium)]
    pub size: LoadingSize,

    /// Primary color for the animation
    #[prop_or_else(|| PrimaryColors::ELECTRIC_BLUE.to_string())]
    pub color: String,

    /// Secondary color for multi-color animations
    #[prop_or_else(|| PrimaryColors::VIBRANT_ORANGE.to_string())]
    pub secondary_color: String,

    /// Optional loading text
    #[prop_or_default]
    pub text: Option<String>,

    /// Whether to center the loader
    #[prop_or(false)]
    pub centered: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Main loading component with multiple animation variants
#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    let animations = r#"
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }

        @keyframes bounce {
            0%, 80%, 100% { transform: scale(0); }
            40% { transform: scale(1); }
        }

        @keyframes pulse {
            0% { transform: scale(1); opacity: 1; }
            50% { transform: scale(1.2); opacity: 0.7; }
            100% { transform: scale(1); opacity: 1; }
        }

        @keyframes wave {
            0%, 60%, 100% { transform: initial; }
            30% { transform: translateY(-15px); }
        }

        @keyframes progress {
            0% { transform: translateX(-100%); }
            100% { transform: translateX(100%); }
        }

        @keyframes skeleton-loading {
            0% { background-position: -200px 0; }
            100% { background-position: calc(200px + 100%) 0; }
        }
    "#;

    let container_style = if props.centered {
        "display: flex; align-items: center; justify-content: center; flex-direction: column; gap: 1rem;"
    } else {
        "display: inline-flex; align-items: center; gap: 0.5rem;"
    };

    let loader_content = match props.variant {
        LoadingVariant::Spinner => render_spinner(props),
        LoadingVariant::Dots => render_dots(props),
        LoadingVariant::Pulse => render_pulse(props),
        LoadingVariant::Wave => render_wave(props),
        LoadingVariant::Progress => render_progress(props),
        LoadingVariant::Skeleton => render_skeleton(props),
    };

    html! {
        <>
            <style>{ animations }</style>
            <div class={classes!("loading-container", props.class.clone())} style={container_style}>
                { loader_content }
                if let Some(text) = &props.text {
                    <span class="loading-text"
                          style={format!("{}; color: {}; opacity: 0.8;",
                                        TextStyles::caption_text(),
                                        TextColors::SECONDARY_LIGHT)}>
                        { text }
                    </span>
                }
            </div>
        </>
    }
}

/// Render spinner animation
fn render_spinner(props: &LoadingProps) -> Html {
    let style = format!(
        "width: {}; height: {}; border: 2px solid {}; border-top: 2px solid {};
         border-radius: 50%; animation: spin 1s linear infinite;",
        props.size.size(),
        props.size.size(),
        format!("{}30", props.color.trim_start_matches('#')),
        props.color
    );

    html! {
        <div class="loading-spinner" style={style}></div>
    }
}

/// Render bouncing dots animation
fn render_dots(props: &LoadingProps) -> Html {
    let dot_size = match props.size {
        LoadingSize::Small => "0.25rem",
        LoadingSize::Medium => "0.375rem",
        LoadingSize::Large => "0.5rem",
        LoadingSize::ExtraLarge => "0.75rem",
    };

    let container_style = format!(
        "display: flex; align-items: center; gap: {}; height: {};",
        Spacing::SM,
        props.size.size()
    );

    html! {
        <div class="loading-dots" style={container_style}>
            { for (0..3).map(|i| {
                let delay = format!("{}s", i as f32 * 0.16);
                let dot_style = format!(
                    "width: {}; height: {}; background: {}; border-radius: 50%;
                     animation: bounce 1.4s ease-in-out {} infinite both;",
                    dot_size, dot_size, props.color, delay
                );
                html! {
                    <div class="loading-dot" style={dot_style}></div>
                }
            })}
        </div>
    }
}

/// Render pulsing animation
fn render_pulse(props: &LoadingProps) -> Html {
    let style = format!(
        "width: {}; height: {}; background: {}; border-radius: 50%;
         animation: pulse 2s ease-in-out infinite;",
        props.size.size(),
        props.size.size(),
        props.color
    );

    html! {
        <div class="loading-pulse" style={style}></div>
    }
}

/// Render wave animation
fn render_wave(props: &LoadingProps) -> Html {
    let bar_width = match props.size {
        LoadingSize::Small => "0.125rem",
        LoadingSize::Medium => "0.1875rem",
        LoadingSize::Large => "0.25rem",
        LoadingSize::ExtraLarge => "0.375rem",
    };

    let container_style = format!(
        "display: flex; align-items: center; gap: {}; height: {};",
        Spacing::XS,
        props.size.size()
    );

    html! {
        <div class="loading-wave" style={container_style}>
            { for (0..5).map(|i| {
                let delay = format!("{}s", i as f32 * 0.1);
                let bar_style = format!(
                    "width: {}; height: 100%; background: {}; border-radius: {};
                     animation: wave 1.2s ease-in-out {} infinite;",
                    bar_width, props.color, BorderRadius::SM, delay
                );
                html! {
                    <div class="loading-bar" style={bar_style}></div>
                }
            })}
        </div>
    }
}

/// Render progress bar animation
fn render_progress(props: &LoadingProps) -> Html {
    let height = match props.size {
        LoadingSize::Small => "0.25rem",
        LoadingSize::Medium => "0.375rem",
        LoadingSize::Large => "0.5rem",
        LoadingSize::ExtraLarge => "0.75rem",
    };

    let container_style = format!(
        "width: 100%; height: {}; background: {}; border-radius: {}; overflow: hidden; position: relative;",
        height,
        format!("{}20", props.color.trim_start_matches('#')),
        BorderRadius::FULL
    );

    let progress_style = format!(
        "position: absolute; top: 0; left: 0; height: 100%; width: 50%;
         background: linear-gradient(90deg, {} 0%, {} 100%);
         border-radius: {}; animation: progress 2s ease-in-out infinite;",
        props.color,
        props.secondary_color,
        BorderRadius::FULL
    );

    html! {
        <div class="loading-progress-container" style={container_style}>
            <div class="loading-progress-bar" style={progress_style}></div>
        </div>
    }
}

/// Render skeleton loading placeholder
fn render_skeleton(props: &LoadingProps) -> Html {
    let height = props.size.size();
    let width = match props.size {
        LoadingSize::Small => "8rem",
        LoadingSize::Medium => "12rem",
        LoadingSize::Large => "16rem",
        LoadingSize::ExtraLarge => "20rem",
    };

    let style = format!(
        "width: {}; height: {}; background: linear-gradient(90deg, {} 0%, {} 50%, {} 100%);
         background-size: 200px 100%; border-radius: {}; animation: skeleton-loading 1.5s ease-in-out infinite;",
        width,
        height,
        format!("{}20", props.color.trim_start_matches('#')),
        format!("{}40", props.color.trim_start_matches('#')),
        format!("{}20", props.color.trim_start_matches('#')),
        BorderRadius::MD
    );

    html! {
        <div class="loading-skeleton" style={style}></div>
    }
}

/// Properties for skeleton group component
#[derive(Properties, PartialEq)]
pub struct SkeletonGroupProps {
    /// Number of skeleton lines
    #[prop_or(3)]
    pub lines: u8,

    /// Height of each skeleton line
    #[prop_or_else(|| "1rem".to_string())]
    pub line_height: String,

    /// Gap between skeleton lines
    #[prop_or_else(|| Spacing::MD.to_string())]
    pub gap: String,

    /// Color for skeleton animation
    #[prop_or_else(|| TextColors::TERTIARY_LIGHT.to_string())]
    pub color: String,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Skeleton group component for loading placeholders
#[function_component(SkeletonGroup)]
pub fn skeleton_group(props: &SkeletonGroupProps) -> Html {
    let container_style = format!(
        "display: flex; flex-direction: column; gap: {};",
        props.gap
    );

    html! {
        <div class={classes!("skeleton-group", props.class.clone())} style={container_style}>
            { for (0..props.lines).map(|i| {
                let _width = match i {
                    0 => "100%",
                    n if n == props.lines - 1 => "60%",
                    _ => "80%",
                };

                html! {
                    <Loading
                        variant={LoadingVariant::Skeleton}
                        color={props.color.clone()}
                        class={classes!("skeleton-line")}
                    />
                }
            })}
        </div>
    }
}

/// Predefined loading variants for common use cases
pub struct LoadingVariants;

impl LoadingVariants {
    /// Create a primary spinner
    pub fn primary_spinner() -> Html {
        html! {
            <Loading
                variant={LoadingVariant::Spinner}
                color={PrimaryColors::ELECTRIC_BLUE}
            />
        }
    }

    /// Create colorful bouncing dots
    pub fn colorful_dots() -> Html {
        html! {
            <Loading
                variant={LoadingVariant::Dots}
                color={PrimaryColors::ENERGETIC_PINK}
                size={LoadingSize::Large}
            />
        }
    }

    /// Create a centered loading screen
    pub fn loading_screen(text: &str) -> Html {
        html! {
            <Loading
                variant={LoadingVariant::Pulse}
                color={PrimaryColors::VIBRANT_ORANGE}
                size={LoadingSize::ExtraLarge}
                text={Some(text.to_string())}
                centered={true}
            />
        }
    }

    /// Create a progress bar
    pub fn progress_bar() -> Html {
        html! {
            <Loading
                variant={LoadingVariant::Progress}
                color={PrimaryColors::ELECTRIC_BLUE}
                secondary_color={AccentColors::LIME_GREEN}
            />
        }
    }

    /// Create a skeleton placeholder
    pub fn skeleton_placeholder() -> Html {
        html! {
            <SkeletonGroup lines={4} />
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_size_values() {
        assert_eq!(LoadingSize::Small.size(), "1rem");
        assert_eq!(LoadingSize::Medium.size(), "1.5rem");
        assert_eq!(LoadingSize::Large.size(), "2rem");
        assert_eq!(LoadingSize::ExtraLarge.size(), "3rem");
    }

    #[test]
    fn test_loading_variants() {
        assert_eq!(LoadingVariant::Spinner, LoadingVariant::Spinner);
        assert_ne!(LoadingVariant::Spinner, LoadingVariant::Dots);
    }
}
