// Performance Optimization Utilities - 2025 Design System
// WebAssembly performance optimizations and monitoring

use yew::prelude::*;
use web_sys::{window, Performance, PerformanceEntry, PerformanceObserver};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceMetrics {
    pub first_contentful_paint: Option<f64>,
    pub largest_contentful_paint: Option<f64>,
    pub first_input_delay: Option<f64>,
    pub cumulative_layout_shift: Option<f64>,
    pub time_to_interactive: Option<f64>,
    pub total_blocking_time: Option<f64>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            first_contentful_paint: None,
            largest_contentful_paint: None,
            first_input_delay: None,
            cumulative_layout_shift: None,
            time_to_interactive: None,
            total_blocking_time: None,
        }
    }
}

// Performance monitoring hook
#[hook]
pub fn use_performance_monitor() -> PerformanceMetrics {
    let metrics = use_state(PerformanceMetrics::default);
    
    use_effect_with_deps(
        {
            let metrics = metrics.clone();
            move |_| {
                if let Some(window) = window() {
                    if let Ok(performance) = window.performance() {
                        // Monitor Core Web Vitals
                        monitor_core_web_vitals(metrics.clone(), performance);
                    }
                }
                || {}
            }
        },
        (),
    );
    
    (*metrics).clone()
}

// Monitor Core Web Vitals
fn monitor_core_web_vitals(metrics: UseStateHandle<PerformanceMetrics>, performance: Performance) {
    // First Contentful Paint
    if let Ok(entries) = performance.get_entries_by_type("paint") {
        for i in 0..entries.length() {
            if let Some(entry) = entries.get(i) {
                if let Ok(paint_entry) = entry.dyn_into::<PerformanceEntry>() {
                    if paint_entry.name() == "first-contentful-paint" {
                        let mut current_metrics = (*metrics).clone();
                        current_metrics.first_contentful_paint = Some(paint_entry.start_time());
                        metrics.set(current_metrics);
                    }
                }
            }
        }
    }
    
    // Largest Contentful Paint
    let metrics_clone = metrics.clone();
    let lcp_observer = PerformanceObserver::new(&Closure::wrap(Box::new(move |entries: js_sys::Array| {
        for i in 0..entries.length() {
            if let Some(entry) = entries.get(i) {
                if let Ok(lcp_entry) = entry.dyn_into::<PerformanceEntry>() {
                    let mut current_metrics = (*metrics_clone).clone();
                    current_metrics.largest_contentful_paint = Some(lcp_entry.start_time());
                    metrics_clone.set(current_metrics);
                }
            }
        }
    }) as Box<dyn Fn(js_sys::Array)>).into_js_value().unchecked_ref()).unwrap();
    
    let _ = lcp_observer.observe(&js_sys::Object::from(JsValue::from_str(r#"{"entryTypes": ["largest-contentful-paint"]}"#)));
}

// Image lazy loading hook
#[hook]
pub fn use_lazy_image(src: &str, placeholder: Option<&str>) -> (String, bool) {
    let is_loaded = use_state(|| false);
    let current_src = use_state(|| placeholder.unwrap_or("").to_string());
    let src = src.to_string();
    
    use_effect_with_deps(
        {
            let is_loaded = is_loaded.clone();
            let current_src = current_src.clone();
            let src = src.clone();
            move |_| {
                if let Some(window) = window() {
                    if let Some(document) = window.document() {
                        // Create intersection observer for lazy loading
                        let callback = Closure::wrap(Box::new({
                            let is_loaded = is_loaded.clone();
                            let current_src = current_src.clone();
                            let src = src.clone();
                            move |entries: js_sys::Array| {
                                for i in 0..entries.length() {
                                    if let Some(entry) = entries.get(i) {
                                        if let Ok(intersection_entry) = entry.dyn_into::<web_sys::IntersectionObserverEntry>() {
                                            if intersection_entry.is_intersecting() {
                                                current_src.set(src.clone());
                                                is_loaded.set(true);
                                            }
                                        }
                                    }
                                }
                            }
                        }) as Box<dyn Fn(js_sys::Array)>);
                        
                        if let Ok(observer) = web_sys::IntersectionObserver::new(callback.as_ref().unchecked_ref()) {
                            // Observer would be attached to image element in component
                            callback.forget();
                        }
                    }
                }
                || {}
            }
        },
        src,
    );
    
    ((*current_src).clone(), *is_loaded)
}

// Debounce hook for performance optimization
#[hook]
pub fn use_debounce<T: Clone + PartialEq + 'static>(value: T, delay: u32) -> T {
    let debounced_value = use_state(|| value.clone());
    
    use_effect_with_deps(
        {
            let debounced_value = debounced_value.clone();
            let value = value.clone();
            move |_| {
                let timeout = gloo_timers::callback::Timeout::new(delay, move || {
                    debounced_value.set(value);
                });
                
                move || {
                    timeout.cancel();
                }
            }
        },
        value,
    );
    
    (*debounced_value).clone()
}

// Throttle hook for performance optimization
#[hook]
pub fn use_throttle<F: Fn() + 'static>(callback: F, delay: u32) -> Callback<()> {
    let last_call = use_state(|| 0.0);
    
    Callback::from({
        let last_call = last_call.clone();
        move |_| {
            if let Some(window) = window() {
                if let Ok(performance) = window.performance() {
                    let now = performance.now();
                    if now - *last_call >= delay as f64 {
                        callback();
                        last_call.set(now);
                    }
                }
            }
        }
    })
}

// Virtual scrolling hook for large lists
#[derive(Debug, Clone)]
pub struct VirtualScrollConfig {
    pub item_height: f64,
    pub container_height: f64,
    pub overscan: usize,
}

#[hook]
pub fn use_virtual_scroll<T: Clone>(
    items: Vec<T>,
    config: VirtualScrollConfig,
) -> (Vec<(usize, T)>, f64, Callback<f64>) {
    let scroll_top = use_state(|| 0.0);
    
    let visible_items = {
        let start_index = (*scroll_top / config.item_height).floor() as usize;
        let visible_count = (config.container_height / config.item_height).ceil() as usize;
        let end_index = (start_index + visible_count + config.overscan).min(items.len());
        let start_index = start_index.saturating_sub(config.overscan);
        
        items
            .iter()
            .enumerate()
            .skip(start_index)
            .take(end_index - start_index)
            .map(|(i, item)| (i, item.clone()))
            .collect::<Vec<_>>()
    };
    
    let total_height = items.len() as f64 * config.item_height;
    
    let on_scroll = {
        let scroll_top = scroll_top.clone();
        Callback::from(move |new_scroll_top: f64| {
            scroll_top.set(new_scroll_top);
        })
    };
    
    (visible_items, total_height, on_scroll)
}

// Memoization hook for expensive computations
#[hook]
pub fn use_memo<T, F, D>(compute: F, deps: D) -> T
where
    T: Clone + PartialEq + 'static,
    F: Fn() -> T + 'static,
    D: PartialEq + 'static,
{
    let memoized = use_state(|| None::<T>);
    let prev_deps = use_state(|| None::<D>);
    
    if prev_deps.as_ref() != Some(&deps) || memoized.is_none() {
        let value = compute();
        memoized.set(Some(value.clone()));
        prev_deps.set(Some(deps));
        value
    } else {
        memoized.as_ref().unwrap().clone()
    }
}

// Component for lazy loading images
#[derive(Properties, PartialEq)]
pub struct LazyImageProps {
    pub src: String,
    pub alt: String,
    pub placeholder: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub on_load: Option<Callback<()>>,
    pub on_error: Option<Callback<()>>,
}

#[function_component(LazyImage)]
pub fn lazy_image(props: &LazyImageProps) -> Html {
    let img_ref = use_node_ref();
    let is_loaded = use_state(|| false);
    let has_error = use_state(|| false);
    let is_intersecting = use_state(|| false);
    
    // Intersection Observer for lazy loading
    use_effect_with_deps(
        {
            let img_ref = img_ref.clone();
            let is_intersecting = is_intersecting.clone();
            move |_| {
                if let Some(img_element) = img_ref.cast::<web_sys::HtmlImageElement>() {
                    let callback = Closure::wrap(Box::new({
                        let is_intersecting = is_intersecting.clone();
                        move |entries: js_sys::Array| {
                            for i in 0..entries.length() {
                                if let Some(entry) = entries.get(i) {
                                    if let Ok(intersection_entry) = entry.dyn_into::<web_sys::IntersectionObserverEntry>() {
                                        if intersection_entry.is_intersecting() {
                                            is_intersecting.set(true);
                                        }
                                    }
                                }
                            }
                        }
                    }) as Box<dyn Fn(js_sys::Array)>);
                    
                    if let Ok(observer) = web_sys::IntersectionObserver::new(callback.as_ref().unchecked_ref()) {
                        let _ = observer.observe(&img_element);
                        callback.forget();
                        
                        return move || {
                            let _ = observer.unobserve(&img_element);
                        };
                    }
                }
                || {}
            }
        },
        (),
    );
    
    let on_load = {
        let is_loaded = is_loaded.clone();
        let on_load = props.on_load.clone();
        Callback::from(move |_| {
            is_loaded.set(true);
            if let Some(on_load) = &on_load {
                on_load.emit(());
            }
        })
    };
    
    let on_error = {
        let has_error = has_error.clone();
        let on_error = props.on_error.clone();
        Callback::from(move |_| {
            has_error.set(true);
            if let Some(on_error) = &on_error {
                on_error.emit(());
            }
        })
    };
    
    let src = if *is_intersecting {
        &props.src
    } else {
        props.placeholder.as_deref().unwrap_or("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='1' height='1'%3E%3C/svg%3E")
    };
    
    let img_classes = classes!(
        "lazy-image",
        if *is_loaded { "lazy-image-loaded" } else { "lazy-image-loading" },
        if *has_error { "lazy-image-error" } else { "" },
        props.class.clone()
    );
    
    html! {
        <img
            ref={img_ref}
            src={src}
            alt={props.alt.clone()}
            class={img_classes}
            style={props.style.clone()}
            onload={on_load}
            onerror={on_error}
            loading="lazy"
        />
    }
}

// Performance monitoring component
#[derive(Properties, PartialEq)]
pub struct PerformanceMonitorProps {
    pub enabled: Option<bool>,
    pub on_metrics: Option<Callback<PerformanceMetrics>>,
}

#[function_component(PerformanceMonitor)]
pub fn performance_monitor(props: &PerformanceMonitorProps) -> Html {
    let enabled = props.enabled.unwrap_or(true);
    let metrics = use_performance_monitor();
    
    use_effect_with_deps(
        {
            let on_metrics = props.on_metrics.clone();
            let metrics = metrics.clone();
            move |_| {
                if enabled {
                    if let Some(on_metrics) = &on_metrics {
                        on_metrics.emit(metrics);
                    }
                }
                || {}
            }
        },
        (enabled, metrics),
    );
    
    html! {}
}

// CSS for performance optimizations
pub fn generate_performance_css() -> String {
    r#"/* Performance Optimization Styles - 2025 Design System */

/* Lazy loading images */
.lazy-image {
  transition: opacity 0.3s ease;
  will-change: opacity;
}

.lazy-image-loading {
  opacity: 0;
  background: var(--color-bg-light);
}

.lazy-image-loaded {
  opacity: 1;
}

.lazy-image-error {
  opacity: 0.5;
  background: var(--color-bg-light);
}

/* GPU acceleration for animations */
.gpu-accelerated {
  transform: translateZ(0);
  will-change: transform;
}

.gpu-accelerated-opacity {
  will-change: opacity;
}

.gpu-accelerated-scale {
  will-change: transform;
}

/* Optimize repaints */
.contain-layout {
  contain: layout;
}

.contain-paint {
  contain: paint;
}

.contain-size {
  contain: size;
}

.contain-style {
  contain: style;
}

.contain-strict {
  contain: strict;
}

/* Virtual scrolling container */
.virtual-scroll-container {
  overflow-y: auto;
  height: 100%;
  position: relative;
}

.virtual-scroll-spacer {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  pointer-events: none;
}

.virtual-scroll-items {
  position: relative;
  z-index: 1;
}

/* Optimize font rendering */
.optimized-text {
  text-rendering: optimizeSpeed;
  font-display: swap;
}

.crisp-text {
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* Reduce layout thrashing */
.stable-layout {
  position: relative;
  min-height: 1px;
}

.stable-width {
  min-width: 1px;
}

/* Optimize animations */
.optimized-animation {
  animation-fill-mode: both;
  animation-play-state: paused;
}

.optimized-animation.animate {
  animation-play-state: running;
}

/* Preload critical resources */
.preload-image {
  content: '';
  position: absolute;
  left: -9999px;
  top: -9999px;
  width: 1px;
  height: 1px;
  opacity: 0;
}

/* Optimize scrolling */
.smooth-scroll {
  scroll-behavior: smooth;
  -webkit-overflow-scrolling: touch;
}

.optimized-scroll {
  overflow-anchor: none;
  overscroll-behavior: contain;
}

/* Memory optimization */
.memory-optimized {
  pointer-events: none;
}

.memory-optimized.interactive {
  pointer-events: auto;
}

/* Critical rendering path optimization */
.above-fold {
  content-visibility: visible;
}

.below-fold {
  content-visibility: auto;
  contain-intrinsic-size: 200px;
}

/* Reduce paint complexity */
.simple-paint {
  border-radius: 0;
  box-shadow: none;
  background-image: none;
}

.complex-paint {
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-md);
}

/* Optimize for mobile */
@media (max-width: 768px) {
  .mobile-optimized {
    transform: translateZ(0);
    -webkit-backface-visibility: hidden;
    backface-visibility: hidden;
  }
  
  .mobile-simple {
    border-radius: var(--radius-sm);
    box-shadow: var(--shadow-sm);
  }
}

/* Optimize for low-end devices */
@media (max-width: 480px) {
  .low-end-optimized {
    animation: none !important;
    transition: none !important;
    transform: none !important;
    filter: none !important;
  }
  
  .low-end-optimized .complex-paint {
    border-radius: 0;
    box-shadow: none;
    background-image: none;
  }
}

/* Reduce motion for accessibility and performance */
@media (prefers-reduced-motion: reduce) {
  .respect-motion-preference {
    animation: none !important;
    transition: none !important;
  }
  
  .respect-motion-preference * {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}

/* High contrast optimizations */
@media (prefers-contrast: high) {
  .high-contrast-optimized {
    border: 1px solid currentColor;
    background-image: none;
    box-shadow: none;
  }
}

/* Print optimizations */
@media print {
  .print-optimized {
    background: white !important;
    color: black !important;
    box-shadow: none !important;
    border-radius: 0 !important;
  }
  
  .print-hidden {
    display: none !important;
  }
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        assert!(metrics.first_contentful_paint.is_none());
        assert!(metrics.largest_contentful_paint.is_none());
    }

    #[test]
    fn test_virtual_scroll_config() {
        let config = VirtualScrollConfig {
            item_height: 50.0,
            container_height: 500.0,
            overscan: 5,
        };
        assert_eq!(config.item_height, 50.0);
        assert_eq!(config.overscan, 5);
    }

    #[test]
    fn test_css_generation() {
        let css = generate_performance_css();
        assert!(css.contains("lazy-image"));
        assert!(css.contains("gpu-accelerated"));
        assert!(css.contains("virtual-scroll"));
    }
}
