// Bento Grid Layout System for AI4Thai Crop Guardian - 2025 Design
// Flexible, responsive grid system inspired by bento box layouts

use yew::prelude::*;
use web_sys::window;

#[derive(Debug, Clone, PartialEq)]
pub enum GridBreakpoint {
    Mobile,    // < 768px
    Tablet,    // 768px - 1024px
    Desktop,   // > 1024px
}

#[derive(Debug, Clone, PartialEq)]
pub struct GridConfig {
    pub columns: usize,
    pub gap: String,
    pub min_card_width: String,
    pub max_card_width: String,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            columns: 4,
            gap: "1.5rem".to_string(),
            min_card_width: "200px".to_string(),
            max_card_width: "1fr".to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BentoGridProps {
    pub children: Children,
    pub columns: Option<usize>,
    pub gap: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub auto_fit: Option<bool>,
    pub responsive: Option<bool>,
}

#[function_component(BentoGrid)]
pub fn bento_grid(props: &BentoGridProps) -> Html {
    let columns = props.columns.unwrap_or(4);
    let gap = props.gap.as_deref().unwrap_or("1.5rem");
    let auto_fit = props.auto_fit.unwrap_or(false);
    let responsive = props.responsive.unwrap_or(true);
    
    let grid_template = if auto_fit {
        format!("repeat(auto-fit, minmax(250px, 1fr))")
    } else {
        format!("repeat({}, 1fr)", columns)
    };
    
    let base_style = format!(
        "display: grid; 
         grid-template-columns: {}; 
         gap: {}; 
         width: 100%;
         padding: {};",
        grid_template, gap, gap
    );
    
    let responsive_style = if responsive {
        format!(
            "{}
            @media (max-width: 768px) {{
                grid-template-columns: 1fr !important;
                gap: 1rem;
                padding: 1rem;
            }}
            @media (min-width: 769px) and (max-width: 1024px) {{
                grid-template-columns: repeat({}, 1fr);
            }}",
            base_style,
            (columns / 2).max(1)
        )
    } else {
        base_style
    };
    
    let final_style = format!(
        "{}{}",
        responsive_style,
        props.style.as_deref().unwrap_or("")
    );
    
    html! {
        <div 
            class={classes!("bento-grid", props.class.clone())} 
            style={final_style}
        >
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BentoCardProps {
    pub children: Children,
    pub span_cols: Option<usize>,
    pub span_rows: Option<usize>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub color: Option<String>,
    pub gradient: Option<String>,
    pub hover_effect: Option<bool>,
    pub clickable: Option<bool>,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(BentoCard)]
pub fn bento_card(props: &BentoCardProps) -> Html {
    let span_cols = props.span_cols.unwrap_or(1);
    let span_rows = props.span_rows.unwrap_or(1);
    let hover_effect = props.hover_effect.unwrap_or(true);
    let clickable = props.clickable.unwrap_or(props.onclick.is_some());
    
    let grid_style = format!(
        "grid-column: span {}; 
         grid-row: span {};",
        span_cols, span_rows
    );
    
    let background_style = if let Some(gradient) = &props.gradient {
        format!("background: {};", gradient)
    } else if let Some(color) = &props.color {
        format!("background: {};", color)
    } else {
        "background: var(--color-surface-light);".to_string()
    };
    
    let interaction_style = if clickable {
        "cursor: pointer;".to_string()
    } else {
        String::new()
    };
    
    let final_style = format!(
        "{}{}{}{}",
        grid_style,
        background_style,
        interaction_style,
        props.style.as_deref().unwrap_or("")
    );
    
    let card_classes = classes!(
        "bento-card",
        if hover_effect { "bento-card-hover" } else { "" },
        if clickable { "bento-card-clickable" } else { "" },
        props.class.clone()
    );
    
    html! {
        <div 
            class={card_classes}
            style={final_style}
            onclick={props.onclick.clone()}
        >
            <div class="bento-card-content">
                { for props.children.iter() }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BentoSectionProps {
    pub children: Children,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub class: Option<String>,
    pub header_class: Option<String>,
}

#[function_component(BentoSection)]
pub fn bento_section(props: &BentoSectionProps) -> Html {
    html! {
        <section class={classes!("bento-section", props.class.clone())}>
            if props.title.is_some() || props.subtitle.is_some() {
                <header class={classes!("bento-section-header", props.header_class.clone())}>
                    if let Some(title) = &props.title {
                        <h2 class="bento-section-title">{title}</h2>
                    }
                    if let Some(subtitle) = &props.subtitle {
                        <p class="bento-section-subtitle">{subtitle}</p>
                    }
                </header>
            }
            <div class="bento-section-content">
                { for props.children.iter() }
            </div>
        </section>
    }
}

// Responsive grid hook
#[hook]
pub fn use_responsive_grid() -> GridBreakpoint {
    let breakpoint = use_state(|| GridBreakpoint::Desktop);
    
    use_effect_with_deps(
        {
            let breakpoint = breakpoint.clone();
            move |_| {
                let update_breakpoint = {
                    let breakpoint = breakpoint.clone();
                    move || {
                        if let Some(window) = window() {
                            let width = window.inner_width().unwrap().as_f64().unwrap();
                            let new_breakpoint = if width < 768.0 {
                                GridBreakpoint::Mobile
                            } else if width < 1024.0 {
                                GridBreakpoint::Tablet
                            } else {
                                GridBreakpoint::Desktop
                            };
                            breakpoint.set(new_breakpoint);
                        }
                    }
                };
                
                // Initial check
                update_breakpoint();
                
                // Add resize listener
                let closure = Closure::wrap(Box::new(update_breakpoint) as Box<dyn Fn()>);
                if let Some(window) = window() {
                    window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).ok();
                }
                
                move || {
                    if let Some(window) = window() {
                        window.remove_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).ok();
                    }
                }
            }
        },
        (),
    );
    
    (*breakpoint).clone()
}

// CSS generator for bento grid system
pub fn generate_bento_css() -> String {
    r#"/* Bento Grid System - 2025 Design */

.bento-grid {
  display: grid;
  width: 100%;
  box-sizing: border-box;
}

.bento-card {
  border-radius: var(--radius-lg, 16px);
  padding: var(--space-lg, 1.5rem);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  min-height: 120px;
  box-sizing: border-box;
}

.bento-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: var(--gradient-primary, linear-gradient(135deg, #0066FF, #FF1B8D));
  opacity: 0;
  transition: opacity 0.3s ease;
}

.bento-card-hover:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
}

.bento-card-hover:hover::before {
  opacity: 1;
}

.bento-card-clickable {
  cursor: pointer;
}

.bento-card-clickable:active {
  transform: translateY(-2px);
}

.bento-card-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: flex-start;
  height: 100%;
}

.bento-section {
  margin-bottom: var(--space-2xl, 3rem);
}

.bento-section-header {
  margin-bottom: var(--space-xl, 2rem);
  text-align: center;
}

.bento-section-title {
  font-family: var(--font-heading);
  font-size: var(--text-3xl);
  font-weight: var(--weight-bold);
  color: var(--color-text-primary);
  margin-bottom: var(--space-sm, 0.5rem);
}

.bento-section-subtitle {
  font-family: var(--font-body);
  font-size: var(--text-lg);
  color: var(--color-text-secondary);
  max-width: 600px;
  margin: 0 auto;
}

.bento-section-content {
  width: 100%;
}

/* Responsive behavior */
@media (max-width: 768px) {
  .bento-grid {
    grid-template-columns: 1fr !important;
    gap: 1rem !important;
    padding: 1rem !important;
  }
  
  .bento-card {
    grid-column: 1 !important;
    grid-row: auto !important;
    min-height: 100px;
    padding: var(--space-md, 1rem);
  }
  
  .bento-section-title {
    font-size: var(--text-2xl);
  }
  
  .bento-section-subtitle {
    font-size: var(--text-base);
  }
}

@media (min-width: 769px) and (max-width: 1024px) {
  .bento-card {
    min-height: 140px;
  }
}

@media (min-width: 1025px) {
  .bento-card {
    min-height: 160px;
  }
}

/* Animation keyframes */
@keyframes bento-fade-in {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.bento-card {
  animation: bento-fade-in 0.6s ease-out;
}

/* Staggered animation for multiple cards */
.bento-card:nth-child(1) { animation-delay: 0.1s; }
.bento-card:nth-child(2) { animation-delay: 0.2s; }
.bento-card:nth-child(3) { animation-delay: 0.3s; }
.bento-card:nth-child(4) { animation-delay: 0.4s; }
.bento-card:nth-child(5) { animation-delay: 0.5s; }
.bento-card:nth-child(6) { animation-delay: 0.6s; }

/* Accessibility */
.bento-card-clickable:focus {
  outline: 2px solid var(--color-primary-electric-blue);
  outline-offset: 2px;
}

@media (prefers-reduced-motion: reduce) {
  .bento-card {
    animation: none;
    transition: none;
  }
  
  .bento-card-hover:hover {
    transform: none;
  }
}"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_config_default() {
        let config = GridConfig::default();
        assert_eq!(config.columns, 4);
        assert_eq!(config.gap, "1.5rem");
    }

    #[test]
    fn test_css_generation() {
        let css = generate_bento_css();
        assert!(css.contains("bento-grid"));
        assert!(css.contains("bento-card"));
        assert!(css.contains("@media"));
    }
}
