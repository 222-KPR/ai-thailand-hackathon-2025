// Status Card Component - 2025 Design System
// Information display card with dopamine colors and animations

use yew::prelude::*;
use crate::styles::{use_theme, ColorPalette};

#[derive(Debug, Clone, PartialEq)]
pub enum StatusCardVariant {
    Default,
    Success,
    Warning,
    Error,
    Info,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    Up,
    Down,
    Neutral,
}

#[derive(Properties, PartialEq)]
pub struct StatusCardProps {
    pub title: String,
    pub value: String,
    pub trend: Option<String>,
    pub trend_direction: Option<TrendDirection>,
    pub variant: Option<StatusCardVariant>,
    pub icon: Option<String>,
    pub subtitle: Option<String>,
    pub onclick: Option<Callback<MouseEvent>>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub animated: Option<bool>,
    pub loading: Option<bool>,
}

#[function_component(StatusCard)]
pub fn status_card(props: &StatusCardProps) -> Html {
    let theme = use_theme();
    let colors = &theme.colors;
    
    let variant = props.variant.as_ref().unwrap_or(&StatusCardVariant::Default);
    let trend_direction = props.trend_direction.as_ref().unwrap_or(&TrendDirection::Neutral);
    let animated = props.animated.unwrap_or(true);
    let loading = props.loading.unwrap_or(false);
    let clickable = props.onclick.is_some();
    
    // Determine colors based on variant
    let (background_color, accent_color, text_color) = match variant {
        StatusCardVariant::Default => (
            colors.surface_light,
            colors.primary_electric_blue,
            colors.text_primary
        ),
        StatusCardVariant::Success => (
            colors.with_opacity(colors.accent_lime_green, 0.1),
            colors.accent_lime_green,
            colors.text_primary
        ),
        StatusCardVariant::Warning => (
            colors.with_opacity(colors.accent_yellow, 0.1),
            colors.accent_yellow,
            colors.text_primary
        ),
        StatusCardVariant::Error => (
            colors.with_opacity(colors.error, 0.1),
            colors.error,
            colors.text_primary
        ),
        StatusCardVariant::Info => (
            colors.with_opacity(colors.primary_electric_blue, 0.1),
            colors.primary_electric_blue,
            colors.text_primary
        ),
        StatusCardVariant::Custom(color) => (
            colors.with_opacity(color, 0.1),
            color.as_str(),
            colors.text_primary
        ),
    };
    
    // Trend styling
    let (trend_color, trend_icon) = match trend_direction {
        TrendDirection::Up => (colors.accent_lime_green, "📈"),
        TrendDirection::Down => (colors.error, "📉"),
        TrendDirection::Neutral => (colors.text_secondary, "➡️"),
    };
    
    let card_style = format!(
        "background: {};
         border-left: 4px solid {};
         padding: var(--space-lg);
         border-radius: var(--radius-xl);
         box-shadow: var(--shadow-md);
         transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
         cursor: {};
         position: relative;
         overflow: hidden;
         {}",
        background_color,
        accent_color,
        if clickable { "pointer" } else { "default" },
        props.style.as_deref().unwrap_or("")
    );
    
    let card_classes = classes!(
        "status-card",
        match variant {
            StatusCardVariant::Default => "status-card-default",
            StatusCardVariant::Success => "status-card-success",
            StatusCardVariant::Warning => "status-card-warning",
            StatusCardVariant::Error => "status-card-error",
            StatusCardVariant::Info => "status-card-info",
            StatusCardVariant::Custom(_) => "status-card-custom",
        },
        if clickable { "status-card-clickable" } else { "" },
        if animated { "status-card-animated" } else { "" },
        if loading { "status-card-loading" } else { "" },
        props.class.clone()
    );
    
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(onclick) = &onclick {
                onclick.emit(e);
            }
        })
    };
    
    html! {
        <div 
            class={card_classes}
            style={card_style}
            onclick={onclick}
        >
            // Loading overlay
            if loading {
                <div class="status-card-loading-overlay">
                    <div class="status-card-loading-spinner"></div>
                </div>
            }
            
            // Card header with icon and title
            <div class="status-card-header">
                if let Some(icon) = &props.icon {
                    <div class="status-card-icon" style={format!("color: {}", accent_color)}>
                        {icon}
                    </div>
                }
                <div class="status-card-title-group">
                    <h3 class="status-card-title">{&props.title}</h3>
                    if let Some(subtitle) = &props.subtitle {
                        <p class="status-card-subtitle">{subtitle}</p>
                    }
                </div>
            </div>
            
            // Main value display
            <div class="status-card-value" style={format!("color: {}", accent_color)}>
                {&props.value}
            </div>
            
            // Trend information
            if let Some(trend) = &props.trend {
                <div class="status-card-trend">
                    <span class="status-card-trend-icon">{trend_icon}</span>
                    <span 
                        class="status-card-trend-text"
                        style={format!("color: {}", trend_color)}
                    >
                        {trend}
                    </span>
                </div>
            }
            
            // Animated background effect
            if animated && !loading {
                <div 
                    class="status-card-bg-effect"
                    style={format!("background: linear-gradient(135deg, transparent, {})", 
                        colors.with_opacity(accent_color, 0.05))}
                ></div>
            }
        </div>
    }
}

// Quick Action Card - specialized status card for actions
#[derive(Properties, PartialEq)]
pub struct QuickActionProps {
    pub icon: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub onclick: Callback<MouseEvent>,
    pub variant: Option<StatusCardVariant>,
    pub class: Option<String>,
    pub disabled: Option<bool>,
}

#[function_component(QuickAction)]
pub fn quick_action(props: &QuickActionProps) -> Html {
    let disabled = props.disabled.unwrap_or(false);
    
    let onclick = {
        let onclick = props.onclick.clone();
        let disabled = disabled;
        Callback::from(move |e: MouseEvent| {
            if !disabled {
                onclick.emit(e);
            }
        })
    };
    
    html! {
        <StatusCard
            title={props.title.clone()}
            value=""
            icon={Some(props.icon.clone())}
            subtitle={props.subtitle.clone()}
            variant={props.variant.clone()}
            onclick={Some(onclick)}
            class={classes!("quick-action", if disabled { "quick-action-disabled" } else { "" }, props.class.clone())}
            animated={Some(true)}
        />
    }
}

// CSS for status cards
pub fn generate_status_card_css() -> String {
    r#"/* Status Card Styles */
.status-card {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
  min-height: 120px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.status-card-clickable:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.status-card-clickable:active {
  transform: translateY(0px);
}

.status-card-header {
  display: flex;
  align-items: flex-start;
  gap: var(--space-md);
}

.status-card-icon {
  font-size: 1.5rem;
  line-height: 1;
  flex-shrink: 0;
}

.status-card-title-group {
  flex: 1;
  min-width: 0;
}

.status-card-title {
  font-family: var(--font-heading);
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  color: var(--color-text-secondary);
  margin: 0;
  line-height: var(--leading-tight);
}

.status-card-subtitle {
  font-size: var(--text-xs);
  color: var(--color-text-disabled);
  margin: var(--space-xs) 0 0 0;
  line-height: var(--leading-normal);
}

.status-card-value {
  font-family: var(--font-heading);
  font-size: var(--text-3xl);
  font-weight: var(--weight-extrabold);
  line-height: var(--leading-tight);
  margin: var(--space-sm) 0;
}

.status-card-trend {
  display: flex;
  align-items: center;
  gap: var(--space-xs);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
}

.status-card-trend-icon {
  font-size: 1rem;
}

.status-card-trend-text {
  line-height: var(--leading-normal);
}

/* Loading states */
.status-card-loading {
  position: relative;
}

.status-card-loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: inherit;
  z-index: 1;
}

.status-card-loading-spinner {
  width: 24px;
  height: 24px;
  border: 2px solid var(--color-text-disabled);
  border-top: 2px solid var(--color-primary-electric-blue);
  border-radius: 50%;
  animation: status-card-spin 1s linear infinite;
}

@keyframes status-card-spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* Background effect */
.status-card-bg-effect {
  position: absolute;
  top: 0;
  right: 0;
  width: 60%;
  height: 100%;
  border-radius: inherit;
  opacity: 0;
  transition: opacity 0.3s ease;
  pointer-events: none;
}

.status-card-animated:hover .status-card-bg-effect {
  opacity: 1;
}

/* Variant-specific styles */
.status-card-success {
  border-left-color: var(--color-success);
}

.status-card-warning {
  border-left-color: var(--color-warning);
}

.status-card-error {
  border-left-color: var(--color-error);
}

.status-card-info {
  border-left-color: var(--color-info);
}

/* Quick Action specific styles */
.quick-action {
  cursor: pointer;
  text-align: center;
  justify-content: center;
  align-items: center;
  min-height: 100px;
}

.quick-action .status-card-header {
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: var(--space-sm);
}

.quick-action .status-card-icon {
  font-size: 2rem;
}

.quick-action .status-card-title {
  font-size: var(--text-base);
  color: var(--color-text-primary);
}

.quick-action-disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.quick-action-disabled:hover {
  transform: none;
  box-shadow: inherit;
}

/* Responsive design */
@media (max-width: 768px) {
  .status-card {
    min-height: 100px;
    padding: var(--space-md);
  }
  
  .status-card-value {
    font-size: var(--text-2xl);
  }
  
  .quick-action .status-card-icon {
    font-size: 1.5rem;
  }
}

/* Accessibility */
.status-card-clickable:focus {
  outline: 2px solid var(--color-primary-electric-blue);
  outline-offset: 2px;
}

@media (prefers-reduced-motion: reduce) {
  .status-card {
    transition: none;
  }
  
  .status-card-clickable:hover {
    transform: none;
  }
  
  .status-card-bg-effect {
    display: none;
  }
  
  .status-card-loading-spinner {
    animation: none;
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .status-card {
    border: 1px solid var(--color-text-secondary);
  }
}"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_card_variants() {
        let success = StatusCardVariant::Success;
        let error = StatusCardVariant::Error;
        assert_ne!(success, error);
    }

    #[test]
    fn test_trend_directions() {
        let up = TrendDirection::Up;
        let down = TrendDirection::Down;
        assert_ne!(up, down);
    }

    #[test]
    fn test_css_generation() {
        let css = generate_status_card_css();
        assert!(css.contains("status-card"));
        assert!(css.contains("quick-action"));
        assert!(css.contains("@keyframes"));
    }
}
