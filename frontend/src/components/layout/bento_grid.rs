// Copyright (c) 2025 AI4Thai Crop Guardian
// Licensed under the MIT License

//! Bento Grid Layout Components
//!
//! This module implements the bento grid layout system for flexible,
//! modular content organization inspired by Japanese bento boxes.

use yew::prelude::*;
use crate::styles::{colors::*, spacing::*, typography::*};

/// Properties for the main bento grid container
#[derive(Properties, PartialEq)]
pub struct BentoGridProps {
    /// Child components to be arranged in the grid
    pub children: Children,

    /// Number of columns in the grid (default: 12)
    #[prop_or(12)]
    pub columns: u8,

    /// Gap between grid items
    #[prop_or_else(|| Spacing::XL.to_string())]
    pub gap: String,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Grid container padding
    #[prop_or_else(|| Spacing::XL.to_string())]
    pub padding: String,
}

/// Main bento grid container component
#[function_component(BentoGrid)]
pub fn bento_grid(props: &BentoGridProps) -> Html {
    let style = format!(
        "display: grid;
         grid-template-columns: repeat({}, 1fr);
         gap: {};
         padding: {};
         width: 100%;

         /* Responsive adjustments */
         @media (max-width: {}) {{
           grid-template-columns: 1fr;
           gap: {};
           padding: {};
         }}

         @media (min-width: {}) and (max-width: {}) {{
           grid-template-columns: repeat(2, 1fr);
         }}",
        props.columns,
        props.gap,
        props.padding,
        Breakpoints::MD,
        Spacing::MD,
        Spacing::MD,
        Breakpoints::MD,
        Breakpoints::LG
    );

    html! {
        <div class={classes!("bento-grid", props.class.clone())} style={style}>
            { for props.children.iter() }
        </div>
    }
}

/// Size variants for bento cards
#[derive(Clone, PartialEq)]
pub enum BentoSize {
    /// Small card (1x1)
    Small,
    /// Medium card (2x1)
    Medium,
    /// Large card (2x2)
    Large,
    /// Wide card (3x1)
    Wide,
    /// Tall card (1x2)
    Tall,
    /// Hero card (3x2)
    Hero,
    /// Custom size (columns x rows)
    Custom { columns: u8, rows: u8 },
}

impl BentoSize {
    /// Get grid column span for the size
    pub fn column_span(&self) -> u8 {
        match self {
            BentoSize::Small => 1,
            BentoSize::Medium => 2,
            BentoSize::Large => 2,
            BentoSize::Wide => 3,
            BentoSize::Tall => 1,
            BentoSize::Hero => 3,
            BentoSize::Custom { columns, .. } => *columns,
        }
    }

    /// Get grid row span for the size
    pub fn row_span(&self) -> u8 {
        match self {
            BentoSize::Small => 1,
            BentoSize::Medium => 1,
            BentoSize::Large => 2,
            BentoSize::Wide => 1,
            BentoSize::Tall => 2,
            BentoSize::Hero => 2,
            BentoSize::Custom { rows, .. } => *rows,
        }
    }
}

/// Properties for individual bento cards
#[derive(Properties, PartialEq)]
pub struct BentoCardProps {
    /// Card content
    pub children: Children,

    /// Card size variant
    #[prop_or(BentoSize::Small)]
    pub size: BentoSize,

    /// Background color (uses dopamine palette)
    #[prop_or_else(|| SurfaceColors::SURFACE_LIGHT.to_string())]
    pub background: String,

    /// Text color
    #[prop_or_else(|| TextColors::PRIMARY_LIGHT.to_string())]
    pub color: String,

    /// Border radius
    #[prop_or_else(|| BorderRadius::LG.to_string())]
    pub radius: String,

    /// Shadow depth
    #[prop_or_else(|| Shadows::MD.to_string())]
    pub shadow: String,

    /// Padding inside the card
    #[prop_or_else(|| Spacing::XL.to_string())]
    pub padding: String,

    /// Whether the card is interactive (hover effects)
    #[prop_or(false)]
    pub interactive: bool,

    /// Click handler for interactive cards
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Individual bento card component
#[function_component(BentoCard)]
pub fn bento_card(props: &BentoCardProps) -> Html {
    let interactive_styles = if props.interactive {
        format!(
            "cursor: pointer;
             transition: all 0.2s ease;

             &:hover {{
               transform: translateY(-2px);
               box-shadow: {};
             }}

             &:active {{
               transform: translateY(0);
             }}",
            Shadows::LG
        )
    } else {
        String::new()
    };

    let style = format!(
        "grid-column: span {};
         grid-row: span {};
         background: {};
         color: {};
         border-radius: {};
         box-shadow: {};
         padding: {};
         display: flex;
         flex-direction: column;
         overflow: hidden;
         position: relative;
         {}

         /* Mobile responsive */
         @media (max-width: {}) {{
           grid-column: span 1;
           grid-row: span 1;
         }}",
        props.size.column_span(),
        props.size.row_span(),
        props.background,
        props.color,
        props.radius,
        props.shadow,
        props.padding,
        interactive_styles,
        Breakpoints::MD
    );

    html! {
        <div
            class={classes!("bento-card", props.class.clone())}
            style={style}
            onclick={props.onclick.clone()}
        >
            { for props.children.iter() }
        </div>
    }
}

/// Properties for bento card header
#[derive(Properties, PartialEq)]
pub struct BentoCardHeaderProps {
    /// Header title
    pub title: String,

    /// Optional subtitle
    #[prop_or_default]
    pub subtitle: Option<String>,

    /// Optional icon (as HTML)
    #[prop_or_default]
    pub icon: Option<Html>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Bento card header component
#[function_component(BentoCardHeader)]
pub fn bento_card_header(props: &BentoCardHeaderProps) -> Html {
    html! {
        <div class={classes!("bento-card-header", props.class.clone())}
             style="display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1rem;">

            if let Some(icon) = &props.icon {
                <div class="bento-card-icon" style="flex-shrink: 0;">
                    { icon.clone() }
                </div>
            }

            <div class="bento-card-title-group" style="flex: 1; min-width: 0;">
                <h3 class="bento-card-title"
                    style={format!("{}; margin: 0; font-size: {}; font-weight: {};",
                           TextStyles::section_heading(),
                           TypographyScale::H4_SIZE,
                           FontWeights::SEMIBOLD)}>
                    { &props.title }
                </h3>

                if let Some(subtitle) = &props.subtitle {
                    <p class="bento-card-subtitle"
                       style={format!("{}; margin: 0.25rem 0 0 0; color: {}; opacity: 0.7;",
                              TextStyles::caption_text(),
                              TextColors::SECONDARY_LIGHT)}>
                        { subtitle }
                    </p>
                }
            </div>
        </div>
    }
}

/// Properties for bento card content area
#[derive(Properties, PartialEq)]
pub struct BentoCardContentProps {
    /// Content to display
    pub children: Children,

    /// Whether content should grow to fill available space
    #[prop_or(true)]
    pub grow: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Bento card content component
#[function_component(BentoCardContent)]
pub fn bento_card_content(props: &BentoCardContentProps) -> Html {
    let style = if props.grow {
        "flex: 1; display: flex; flex-direction: column;"
    } else {
        "display: flex; flex-direction: column;"
    };

    html! {
        <div class={classes!("bento-card-content", props.class.clone())} style={style}>
            { for props.children.iter() }
        </div>
    }
}

/// Properties for bento card footer
#[derive(Properties, PartialEq)]
pub struct BentoCardFooterProps {
    /// Footer content
    pub children: Children,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Bento card footer component
#[function_component(BentoCardFooter)]
pub fn bento_card_footer(props: &BentoCardFooterProps) -> Html {
    html! {
        <div class={classes!("bento-card-footer", props.class.clone())}
             style="margin-top: auto; padding-top: 1rem;">
            { for props.children.iter() }
        </div>
    }
}

/// Predefined bento card variants for common use cases
pub struct BentoVariants;

impl BentoVariants {
    /// Create a status card with dopamine colors
    pub fn status_card(
        title: String,
        value: String,
        trend: Option<String>,
        color: String,
        onclick: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <BentoCard
                size={BentoSize::Small}
                background={color}
                color={SurfaceColors::SURFACE_LIGHT.to_string()}
                interactive={true}
                onclick={onclick}
            >
                <BentoCardHeader title={title} />
                <BentoCardContent>
                    <div style="text-align: center;">
                        <div style={format!("font-size: {}; font-weight: {}; margin-bottom: 0.5rem;",
                                          TypographyScale::H2_SIZE, FontWeights::BOLD)}>
                            { value }
                        </div>
                        if let Some(trend_text) = trend {
                            <div style={format!("font-size: {}; opacity: 0.8;",
                                              TypographyScale::CAPTION_SIZE)}>
                                { trend_text }
                            </div>
                        }
                    </div>
                </BentoCardContent>
            </BentoCard>
        }
    }

    /// Create a feature card with icon and description
    pub fn feature_card(
        title: String,
        description: String,
        icon: Html,
        onclick: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <BentoCard
                size={BentoSize::Medium}
                interactive={true}
                onclick={onclick}
            >
                <BentoCardHeader
                    title={title}
                    icon={Some(icon)}
                />
                <BentoCardContent>
                    <p style={format!("{}; margin: 0; opacity: 0.8;", TextStyles::body_text())}>
                        { description }
                    </p>
                </BentoCardContent>
            </BentoCard>
        }
    }

    /// Create a hero card for main actions
    pub fn hero_card(
        title: String,
        subtitle: String,
        background_gradient: String,
        onclick: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <BentoCard
                size={BentoSize::Hero}
                background={background_gradient}
                color={SurfaceColors::SURFACE_LIGHT.to_string()}
                interactive={true}
                onclick={onclick}
            >
                <BentoCardContent>
                    <div style="text-align: center; padding: 2rem;">
                        <h2 style={format!("{}; margin: 0 0 1rem 0;", TextStyles::hero())}>
                            { title }
                        </h2>
                        <p style={format!("{}; margin: 0; opacity: 0.9;", TextStyles::body_text())}>
                            { subtitle }
                        </p>
                    </div>
                </BentoCardContent>
            </BentoCard>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bento_size_spans() {
        assert_eq!(BentoSize::Small.column_span(), 1);
        assert_eq!(BentoSize::Small.row_span(), 1);

        assert_eq!(BentoSize::Hero.column_span(), 3);
        assert_eq!(BentoSize::Hero.row_span(), 2);

        let custom = BentoSize::Custom { columns: 4, rows: 3 };
        assert_eq!(custom.column_span(), 4);
        assert_eq!(custom.row_span(), 3);
    }
}
