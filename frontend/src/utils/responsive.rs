// Responsive Design Utilities - 2025 Design System
// Breakpoint management and responsive behavior utilities

use yew::prelude::*;
use web_sys::{window, MediaQueryList};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, PartialEq)]
pub enum Breakpoint {
    XS,  // < 475px
    SM,  // 475px - 639px
    MD,  // 640px - 767px
    LG,  // 768px - 1023px
    XL,  // 1024px - 1279px
    XXL, // >= 1280px
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakpointConfig {
    pub xs: u32,
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
    pub xxl: u32,
}

impl Default for BreakpointConfig {
    fn default() -> Self {
        Self {
            xs: 475,
            sm: 640,
            md: 768,
            lg: 1024,
            xl: 1280,
            xxl: 1536,
        }
    }
}

impl Breakpoint {
    pub fn from_width(width: u32) -> Self {
        let config = BreakpointConfig::default();

        if width < config.xs {
            Breakpoint::XS
        } else if width < config.sm {
            Breakpoint::SM
        } else if width < config.md {
            Breakpoint::MD
        } else if width < config.lg {
            Breakpoint::LG
        } else if width < config.xl {
            Breakpoint::XL
        } else {
            Breakpoint::XXL
        }
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, Breakpoint::XS | Breakpoint::SM)
    }

    pub fn is_tablet(&self) -> bool {
        matches!(self, Breakpoint::MD | Breakpoint::LG)
    }

    pub fn is_desktop(&self) -> bool {
        matches!(self, Breakpoint::XL | Breakpoint::XXL)
    }

    pub fn get_columns(&self, mobile: usize, tablet: usize, desktop: usize) -> usize {
        if self.is_mobile() {
            mobile
        } else if self.is_tablet() {
            tablet
        } else {
            desktop
        }
    }
}

// Hook for responsive breakpoint detection
#[hook]
pub fn use_breakpoint() -> Breakpoint {
    let breakpoint = use_state(|| Breakpoint::LG);

    use_effect_with_deps(
        {
            let breakpoint = breakpoint.clone();
            move |_| {
                let update_breakpoint = {
                    let breakpoint = breakpoint.clone();
                    move || {
                        if let Some(window) = window() {
                            if let Ok(width) = window.inner_width() {
                                if let Some(width) = width.as_f64() {
                                    let new_breakpoint = Breakpoint::from_width(width as u32);
                                    breakpoint.set(new_breakpoint);
                                }
                            }
                        }
                    }
                };

                // Initial check
                update_breakpoint();

                // Add resize listener
                let closure = Closure::wrap(Box::new(update_breakpoint) as Box<dyn Fn()>);
                if let Some(window) = window() {
                    let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
                }

                move || {
                    if let Some(window) = window() {
                        let _ = window.remove_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
                    }
                }
            }
        },
        (),
    );

    (*breakpoint).clone()
}

// Hook for media query matching
#[hook]
pub fn use_media_query(query: &str) -> bool {
    let matches = use_state(|| false);
    let query = query.to_string();

    use_effect_with_deps(
        {
            let matches = matches.clone();
            let query = query.clone();
            move |_| {
                if let Some(window) = window() {
                    if let Ok(media_query) = window.match_media(&query) {
                        if let Some(mql) = media_query {
                            // Set initial value
                            matches.set(mql.matches());

                            // Add change listener
                            let matches_clone = matches.clone();
                            let closure = Closure::wrap(Box::new(move |event: web_sys::MediaQueryListEvent| {
                                matches_clone.set(event.matches());
                            }) as Box<dyn Fn(web_sys::MediaQueryListEvent)>);

                            let _ = mql.add_listener_with_opt_callback(Some(closure.as_ref().unchecked_ref()));

                            return move || {
                                let _ = mql.remove_listener_with_opt_callback(Some(closure.as_ref().unchecked_ref()));
                            };
                        }
                    }
                }
                || {}
            }
        },
        query,
    );

    *matches
}

// Hook for window dimensions
#[hook]
pub fn use_window_size() -> (u32, u32) {
    let size = use_state(|| (1024u32, 768u32));

    use_effect_with_deps(
        {
            let size = size.clone();
            move |_| {
                let update_size = {
                    let size = size.clone();
                    move || {
                        if let Some(window) = window() {
                            let width = window.inner_width().unwrap_or_else(|_| 1024.into()).as_f64().unwrap_or(1024.0) as u32;
                            let height = window.inner_height().unwrap_or_else(|_| 768.into()).as_f64().unwrap_or(768.0) as u32;
                            size.set((width, height));
                        }
                    }
                };

                // Initial check
                update_size();

                // Add resize listener
                let closure = Closure::wrap(Box::new(update_size) as Box<dyn Fn()>);
                if let Some(window) = window() {
                    let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
                }

                move || {
                    if let Some(window) = window() {
                        let _ = window.remove_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
                    }
                }
            }
        },
        (),
    );

    *size
}

// Responsive component wrapper
#[derive(Properties, PartialEq)]
pub struct ResponsiveProps {
    pub children: Children,
    pub mobile: Option<Html>,
    pub tablet: Option<Html>,
    pub desktop: Option<Html>,
    pub class: Option<String>,
}

#[function_component(Responsive)]
pub fn responsive(props: &ResponsiveProps) -> Html {
    let breakpoint = use_breakpoint();

    let content = if breakpoint.is_mobile() && props.mobile.is_some() {
        props.mobile.clone().unwrap()
    } else if breakpoint.is_tablet() && props.tablet.is_some() {
        props.tablet.clone().unwrap()
    } else if breakpoint.is_desktop() && props.desktop.is_some() {
        props.desktop.clone().unwrap()
    } else {
        html! { { for props.children.iter() } }
    };

    html! {
        <div class={classes!("responsive-container", props.class.clone())}>
            {content}
        </div>
    }
}

// Show/Hide components based on breakpoint
#[derive(Properties, PartialEq)]
pub struct ShowProps {
    pub children: Children,
    pub mobile: Option<bool>,
    pub tablet: Option<bool>,
    pub desktop: Option<bool>,
    pub class: Option<String>,
}

#[function_component(Show)]
pub fn show(props: &ShowProps) -> Html {
    let breakpoint = use_breakpoint();

    let should_show = if breakpoint.is_mobile() {
        props.mobile.unwrap_or(true)
    } else if breakpoint.is_tablet() {
        props.tablet.unwrap_or(true)
    } else {
        props.desktop.unwrap_or(true)
    };

    if should_show {
        html! {
            <div class={classes!("show-container", props.class.clone())}>
                { for props.children.iter() }
            </div>
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
pub struct HideProps {
    pub children: Children,
    pub mobile: Option<bool>,
    pub tablet: Option<bool>,
    pub desktop: Option<bool>,
    pub class: Option<String>,
}

#[function_component(Hide)]
pub fn hide(props: &HideProps) -> Html {
    let breakpoint = use_breakpoint();

    let should_hide = if breakpoint.is_mobile() {
        props.mobile.unwrap_or(false)
    } else if breakpoint.is_tablet() {
        props.tablet.unwrap_or(false)
    } else {
        props.desktop.unwrap_or(false)
    };

    if !should_hide {
        html! {
            <div class={classes!("hide-container", props.class.clone())}>
                { for props.children.iter() }
            </div>
        }
    } else {
        html! {}
    }
}

// Container component with responsive padding and max-width
#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    pub children: Children,
    pub max_width: Option<String>,
    pub padding: Option<bool>,
    pub center: Option<bool>,
    pub class: Option<String>,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let breakpoint = use_breakpoint();
    let padding = props.padding.unwrap_or(true);
    let center = props.center.unwrap_or(true);

    let max_width = props.max_width.as_deref().unwrap_or_else(|| {
        match breakpoint {
            Breakpoint::XS => "100%",
            Breakpoint::SM => "640px",
            Breakpoint::MD => "768px",
            Breakpoint::LG => "1024px",
            Breakpoint::XL => "1280px",
            Breakpoint::XXL => "1536px",
        }
    });

    let padding_value = if padding {
        match breakpoint {
            Breakpoint::XS => "var(--space-md)",
            Breakpoint::SM => "var(--space-lg)",
            _ => "var(--space-xl)",
        }
    } else {
        "0"
    };

    let style = format!(
        "max-width: {}; padding: 0 {}; {}",
        max_width,
        padding_value,
        if center { "margin: 0 auto;" } else { "" }
    );

    html! {
        <div
            class={classes!("container", props.class.clone())}
            {style}
        >
            { for props.children.iter() }
        </div>
    }
}

// CSS utilities for responsive design
pub fn generate_responsive_css() -> String {
    r#"/* Responsive Design Utilities - 2025 Design System */

/* Container utilities */
.container {
  width: 100%;
  box-sizing: border-box;
}

.responsive-container {
  width: 100%;
}

.show-container,
.hide-container {
  width: 100%;
}

/* Responsive visibility utilities */
@media (max-width: 474px) {
  .hidden-xs { display: none !important; }
  .visible-xs { display: block !important; }
}

@media (min-width: 475px) and (max-width: 639px) {
  .hidden-sm { display: none !important; }
  .visible-sm { display: block !important; }
}

@media (min-width: 640px) and (max-width: 767px) {
  .hidden-md { display: none !important; }
  .visible-md { display: block !important; }
}

@media (min-width: 768px) and (max-width: 1023px) {
  .hidden-lg { display: none !important; }
  .visible-lg { display: block !important; }
}

@media (min-width: 1024px) and (max-width: 1279px) {
  .hidden-xl { display: none !important; }
  .visible-xl { display: block !important; }
}

@media (min-width: 1280px) {
  .hidden-xxl { display: none !important; }
  .visible-xxl { display: block !important; }
}

/* Mobile-first responsive utilities */
.mobile-only {
  display: block;
}

.tablet-up {
  display: none;
}

.desktop-up {
  display: none;
}

@media (min-width: 640px) {
  .mobile-only {
    display: none;
  }

  .tablet-up {
    display: block;
  }
}

@media (min-width: 1024px) {
  .desktop-up {
    display: block;
  }
}

/* Responsive text alignment */
.text-center-mobile {
  text-align: center;
}

@media (min-width: 768px) {
  .text-center-mobile {
    text-align: left;
  }
}

/* Responsive spacing */
.responsive-padding {
  padding: var(--space-md);
}

@media (min-width: 640px) {
  .responsive-padding {
    padding: var(--space-lg);
  }
}

@media (min-width: 1024px) {
  .responsive-padding {
    padding: var(--space-xl);
  }
}

.responsive-margin {
  margin: var(--space-md);
}

@media (min-width: 640px) {
  .responsive-margin {
    margin: var(--space-lg);
  }
}

@media (min-width: 1024px) {
  .responsive-margin {
    margin: var(--space-xl);
  }
}

/* Responsive grid columns */
.responsive-grid {
  display: grid;
  gap: var(--space-md);
  grid-template-columns: 1fr;
}

@media (min-width: 640px) {
  .responsive-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-lg);
  }
}

@media (min-width: 1024px) {
  .responsive-grid {
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-xl);
  }
}

@media (min-width: 1280px) {
  .responsive-grid {
    grid-template-columns: repeat(4, 1fr);
  }
}

/* Responsive flex direction */
.responsive-flex {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

@media (min-width: 768px) {
  .responsive-flex {
    flex-direction: row;
    gap: var(--space-lg);
  }
}

/* Responsive font sizes */
.responsive-text {
  font-size: var(--text-sm);
}

@media (min-width: 640px) {
  .responsive-text {
    font-size: var(--text-base);
  }
}

@media (min-width: 1024px) {
  .responsive-text {
    font-size: var(--text-lg);
  }
}

/* Responsive button sizes */
.responsive-button {
  width: 100%;
  padding: var(--space-sm) var(--space-md);
  font-size: var(--text-sm);
}

@media (min-width: 640px) {
  .responsive-button {
    width: auto;
    padding: var(--space-md) var(--space-lg);
    font-size: var(--text-base);
  }
}

/* Responsive card layouts */
.responsive-card {
  padding: var(--space-md);
  margin-bottom: var(--space-md);
}

@media (min-width: 640px) {
  .responsive-card {
    padding: var(--space-lg);
    margin-bottom: var(--space-lg);
  }
}

@media (min-width: 1024px) {
  .responsive-card {
    padding: var(--space-xl);
    margin-bottom: var(--space-xl);
  }
}

/* Responsive image sizing */
.responsive-image {
  width: 100%;
  height: auto;
  max-width: 100%;
}

@media (min-width: 768px) {
  .responsive-image {
    max-width: 500px;
  }
}

@media (min-width: 1024px) {
  .responsive-image {
    max-width: 600px;
  }
}

/* Touch-friendly sizing for mobile */
@media (max-width: 767px) {
  .touch-friendly {
    min-height: 44px;
    min-width: 44px;
    padding: var(--space-sm);
  }

  .touch-friendly-large {
    min-height: 56px;
    min-width: 56px;
    padding: var(--space-md);
  }
}

/* High DPI display optimizations */
@media (-webkit-min-device-pixel-ratio: 2), (min-resolution: 192dpi) {
  .high-dpi-image {
    image-rendering: -webkit-optimize-contrast;
    image-rendering: crisp-edges;
  }
}

/* Landscape orientation adjustments */
@media (orientation: landscape) and (max-height: 500px) {
  .landscape-compact {
    padding: var(--space-sm);
    font-size: var(--text-sm);
  }
}

/* Print styles */
@media print {
  .no-print {
    display: none !important;
  }

  .print-only {
    display: block !important;
  }

  .responsive-container,
  .container {
    max-width: none !important;
    padding: 0 !important;
    margin: 0 !important;
  }
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_from_width() {
        assert_eq!(Breakpoint::from_width(400), Breakpoint::XS);
        assert_eq!(Breakpoint::from_width(800), Breakpoint::LG);
        assert_eq!(Breakpoint::from_width(1400), Breakpoint::XXL);
    }

    #[test]
    fn test_breakpoint_categories() {
        let mobile = Breakpoint::XS;
        let tablet = Breakpoint::MD;
        let desktop = Breakpoint::XL;

        assert!(mobile.is_mobile());
        assert!(tablet.is_tablet());
        assert!(desktop.is_desktop());
    }

    #[test]
    fn test_responsive_columns() {
        let breakpoint = Breakpoint::MD;
        let columns = breakpoint.get_columns(1, 2, 4);
        assert_eq!(columns, 2);
    }

    #[test]
    fn test_css_generation() {
        let css = generate_responsive_css();
        assert!(css.contains("responsive-grid"));
        assert!(css.contains("@media"));
        assert!(css.contains("touch-friendly"));
    }
}
