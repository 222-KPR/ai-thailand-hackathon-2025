// Copyright (c) 2025 AI4Thai Crop Guardian
// Licensed under the MIT License

//! Status Card Component
//! 
//! This module implements status cards for displaying metrics, statistics,
//! and key information with trend indicators and dopamine colors.

use yew::prelude::*;
use crate::styles::{colors::*, spacing::*, typography::*};

/// Trend direction for status indicators
#[derive(Clone, PartialEq)]
pub enum TrendDirection {
    /// Upward trend (positive)
    Up,
    /// Downward trend (negative)
    Down,
    /// Neutral/stable trend
    Neutral,
}

impl TrendDirection {
    /// Get the color for the trend direction
    pub fn color(&self) -> &'static str {
        match self {
            TrendDirection::Up => SemanticColors::SUCCESS,
            TrendDirection::Down => SemanticColors::ERROR,
            TrendDirection::Neutral => TextColors::SECONDARY_LIGHT,
        }
    }
    
    /// Get the icon for the trend direction
    pub fn icon(&self) -> &'static str {
        match self {
            TrendDirection::Up => "↗",
            TrendDirection::Down => "↘",
            TrendDirection::Neutral => "→",
        }
    }
}

/// Status card size variants
#[derive(Clone, PartialEq)]
pub enum StatusCardSize {
    /// Compact card for dense layouts
    Compact,
    /// Standard card size
    Standard,
    /// Large card for prominent metrics
    Large,
}

impl StatusCardSize {
    /// Get the padding for the card size
    pub fn padding(&self) -> &'static str {
        match self {
            StatusCardSize::Compact => Spacing::MD,
            StatusCardSize::Standard => Spacing::XL,
            StatusCardSize::Large => Spacing::XXL,
        }
    }
    
    /// Get the value font size for the card size
    pub fn value_font_size(&self) -> &'static str {
        match self {
            StatusCardSize::Compact => TypographyScale::H4_SIZE,
            StatusCardSize::Standard => TypographyScale::H2_SIZE,
            StatusCardSize::Large => TypographyScale::H1_SIZE,
        }
    }
    
    /// Get the label font size for the card size
    pub fn label_font_size(&self) -> &'static str {
        match self {
            StatusCardSize::Compact => TypographyScale::CAPTION_SIZE,
            StatusCardSize::Standard => TypographyScale::BODY_SMALL_SIZE,
            StatusCardSize::Large => TypographyScale::BODY_SIZE,
        }
    }
}

/// Properties for the status card component
#[derive(Properties, PartialEq)]
pub struct StatusCardProps {
    /// Card title/label
    pub label: String,
    
    /// Main value to display
    pub value: String,
    
    /// Optional unit for the value (e.g., "%", "kg", "฿")
    #[prop_or_default]
    pub unit: Option<String>,
    
    /// Optional trend information
    #[prop_or_default]
    pub trend: Option<TrendInfo>,
    
    /// Card size variant
    #[prop_or(StatusCardSize::Standard)]
    pub size: StatusCardSize,
    
    /// Background color (uses dopamine palette)
    #[prop_or_else(|| SurfaceColors::SURFACE_LIGHT.to_string())]
    pub background: String,
    
    /// Accent color for highlights
    #[prop_or_else(|| PrimaryColors::ELECTRIC_BLUE.to_string())]
    pub accent_color: String,
    
    /// Whether the card is interactive
    #[prop_or(false)]
    pub interactive: bool,
    
    /// Click handler for interactive cards
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    
    /// Optional icon to display
    #[prop_or_default]
    pub icon: Option<Html>,
    
    /// Additional description text
    #[prop_or_default]
    pub description: Option<String>,
    
    /// Whether to show animated background
    #[prop_or(false)]
    pub animated_background: bool,
    
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Trend information for status cards
#[derive(Clone, PartialEq)]
pub struct TrendInfo {
    /// Trend direction
    pub direction: TrendDirection,
    /// Trend value (e.g., "12%", "+5")
    pub value: String,
    /// Trend description (e.g., "vs last month")
    pub description: String,
}

/// Status card component for displaying metrics and statistics
#[function_component(StatusCard)]
pub fn status_card(props: &StatusCardProps) -> Html {
    let animated_bg = if props.animated_background {
        format!(
            "background: linear-gradient(-45deg, {}, {}, {}, {});
             background-size: 400% 400%;
             animation: gradient-shift 8s ease infinite;",
            props.background,
            format!("{}20", props.accent_color.trim_start_matches('#')),
            props.background,
            format!("{}10", props.accent_color.trim_start_matches('#'))
        )
    } else {
        format!("background: {};", props.background)
    };
    
    let interactive_styles = if props.interactive {
        format!(
            "cursor: pointer;
             transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
             
             &:hover {{
               transform: translateY(-4px);
               box-shadow: {};
             }}
             
             &:active {{
               transform: translateY(-2px);
             }}",
            Shadows::XL
        )
    } else {
        "transition: box-shadow 0.3s ease;".to_string()
    };
    
    let card_styles = format!(
        "position: relative;
         padding: {};
         border-radius: {};
         box-shadow: {};
         overflow: hidden;
         {}
         {}
         
         /* Gradient animation */
         @keyframes gradient-shift {{
           0% {{ background-position: 0% 50%; }}
           50% {{ background-position: 100% 50%; }}
           100% {{ background-position: 0% 50%; }}
         }}",
        props.size.padding(),
        BorderRadius::LG,
        Shadows::MD,
        animated_bg,
        interactive_styles
    );
    
    let value_with_unit = if let Some(unit) = &props.unit {
        format!("{}{}", props.value, unit)
    } else {
        props.value.clone()
    };
    
    html! {
        <div 
            class={classes!("status-card", props.class.clone())}
            style={card_styles}
            onclick={props.onclick.clone()}
        >
            // Header with label and optional icon
            <div class="status-card-header" 
                 style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 1rem;">
                
                <div class="status-card-label-group" style="display: flex; align-items: center; gap: 0.5rem;">
                    if let Some(icon) = &props.icon {
                        <div class="status-card-icon" style={format!("color: {};", props.accent_color)}>
                            { icon.clone() }
                        </div>
                    }
                    <span class="status-card-label" 
                          style={format!("font-size: {}; font-weight: {}; color: {}; opacity: 0.8;",
                                        props.size.label_font_size(),
                                        FontWeights::MEDIUM,
                                        TextColors::SECONDARY_LIGHT)}>
                        { &props.label }
                    </span>
                </div>
                
                if let Some(trend) = &props.trend {
                    <div class="status-card-trend" 
                         style={format!("display: flex; align-items: center; gap: 0.25rem; 
                                        font-size: {}; color: {};",
                                       TypographyScale::CAPTION_SIZE,
                                       trend.direction.color())}>
                        <span class="trend-icon">{ trend.direction.icon() }</span>
                        <span class="trend-value">{ &trend.value }</span>
                    </div>
                }
            </div>
            
            // Main value display
            <div class="status-card-value" 
                 style={format!("font-size: {}; font-weight: {}; color: {}; line-height: 1.2; margin-bottom: 0.5rem;",
                               props.size.value_font_size(),
                               FontWeights::BOLD,
                               TextColors::PRIMARY_LIGHT)}>
                { value_with_unit }
            </div>
            
            // Optional description
            if let Some(description) = &props.description {
                <div class="status-card-description" 
                     style={format!("font-size: {}; color: {}; opacity: 0.7; margin-bottom: 0.5rem;",
                                   TypographyScale::CAPTION_SIZE,
                                   TextColors::SECONDARY_LIGHT)}>
                    { description }
                </div>
            }
            
            // Trend description
            if let Some(trend) = &props.trend {
                <div class="status-card-trend-description" 
                     style={format!("font-size: {}; color: {}; opacity: 0.6;",
                                   TypographyScale::CAPTION_SIZE,
                                   TextColors::SECONDARY_LIGHT)}>
                    { &trend.description }
                </div>
            }
            
            // Accent border
            <div class="status-card-accent" 
                 style={format!("position: absolute; bottom: 0; left: 0; right: 0; height: 3px; 
                                background: linear-gradient(90deg, {} 0%, {} 100%);",
                               props.accent_color,
                               format!("{}80", props.accent_color.trim_start_matches('#')))}>
            </div>
        </div>
    }
}

/// Properties for status card grid
#[derive(Properties, PartialEq)]
pub struct StatusCardGridProps {
    /// Status cards to display
    pub children: Children,
    
    /// Number of columns (responsive)
    #[prop_or(3)]
    pub columns: u8,
    
    /// Gap between cards
    #[prop_or_else(|| Spacing::XL.to_string())]
    pub gap: String,
    
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Grid container for organizing multiple status cards
#[function_component(StatusCardGrid)]
pub fn status_card_grid(props: &StatusCardGridProps) -> Html {
    let grid_styles = format!(
        "display: grid;
         grid-template-columns: repeat({}, 1fr);
         gap: {};
         
         /* Responsive adjustments */
         @media (max-width: {}) {{
           grid-template-columns: 1fr;
         }}
         
         @media (min-width: {}) and (max-width: {}) {{
           grid-template-columns: repeat(2, 1fr);
         }}",
        props.columns,
        props.gap,
        Breakpoints::MD,
        Breakpoints::MD,
        Breakpoints::LG
    );
    
    html! {
        <div class={classes!("status-card-grid", props.class.clone())} style={grid_styles}>
            { for props.children.iter() }
        </div>
    }
}

/// Predefined status card variants for common use cases
pub struct StatusCardVariants;

impl StatusCardVariants {
    /// Create a metric card with trend
    pub fn metric_card(
        label: &str,
        value: &str,
        unit: Option<&str>,
        trend_direction: TrendDirection,
        trend_value: &str,
        trend_description: &str,
        onclick: Callback<MouseEvent>,
    ) -> Html {
        let trend = TrendInfo {
            direction: trend_direction,
            value: trend_value.to_string(),
            description: trend_description.to_string(),
        };
        
        html! {
            <StatusCard
                label={label.to_string()}
                value={value.to_string()}
                unit={unit.map(|u| u.to_string())}
                trend={Some(trend)}
                interactive={true}
                onclick={onclick}
            />
        }
    }
    
    /// Create a colorful card with dopamine colors
    pub fn colorful_card(
        label: &str,
        value: &str,
        color: &str,
        icon: Html,
        onclick: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <StatusCard
                label={label.to_string()}
                value={value.to_string()}
                accent_color={color.to_string()}
                icon={Some(icon)}
                animated_background={true}
                interactive={true}
                onclick={onclick}
            />
        }
    }
    
    /// Create a compact card for dense layouts
    pub fn compact_card(
        label: &str,
        value: &str,
        onclick: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <StatusCard
                label={label.to_string()}
                value={value.to_string()}
                size={StatusCardSize::Compact}
                interactive={true}
                onclick={onclick}
            />
        }
    }
    
    /// Create a large featured card
    pub fn featured_card(
        label: &str,
        value: &str,
        description: &str,
        color: &str,
        onclick: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <StatusCard
                label={label.to_string()}
                value={value.to_string()}
                description={Some(description.to_string())}
                size={StatusCardSize::Large}
                accent_color={color.to_string()}
                animated_background={true}
                interactive={true}
                onclick={onclick}
            />
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trend_direction_colors() {
        assert_eq!(TrendDirection::Up.color(), "#34C759");
        assert_eq!(TrendDirection::Down.color(), "#FF3B30");
        assert_eq!(TrendDirection::Neutral.color(), "#6D6D80");
    }

    #[test]
    fn test_trend_direction_icons() {
        assert_eq!(TrendDirection::Up.icon(), "↗");
        assert_eq!(TrendDirection::Down.icon(), "↘");
        assert_eq!(TrendDirection::Neutral.icon(), "→");
    }

    #[test]
    fn test_status_card_size_properties() {
        assert_eq!(StatusCardSize::Compact.padding(), "0.5rem");
        assert_eq!(StatusCardSize::Standard.padding(), "1rem");
        assert_eq!(StatusCardSize::Large.padding(), "1.25rem");
    }

    #[test]
    fn test_trend_info_creation() {
        let trend = TrendInfo {
            direction: TrendDirection::Up,
            value: "12%".to_string(),
            description: "vs last month".to_string(),
        };
        
        assert_eq!(trend.direction, TrendDirection::Up);
        assert_eq!(trend.value, "12%");
        assert_eq!(trend.description, "vs last month");
    }
}
