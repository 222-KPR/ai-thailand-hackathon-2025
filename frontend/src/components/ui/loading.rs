// Loading Animations Component - 2025 Design System
// Dopamine-colored loading animations and progress indicators

use yew::prelude::*;
use crate::styles::{use_theme, Typography, TypographyVariant, TypographyColor};

#[derive(Debug, Clone, PartialEq)]
pub enum LoadingVariant {
    Spinner,
    Dots,
    Pulse,
    Wave,
    Progress,
    Skeleton,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoadingSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    pub variant: Option<LoadingVariant>,
    pub size: Option<LoadingSize>,
    pub message: Option<String>,
    pub progress: Option<f32>, // 0.0 to 1.0 for progress variant
    pub color: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub overlay: Option<bool>, // Full screen overlay
}

#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    let theme = use_theme();
    let colors = &theme.colors;
    
    let variant = props.variant.as_ref().unwrap_or(&LoadingVariant::Spinner);
    let size = props.size.as_ref().unwrap_or(&LoadingSize::Medium);
    let overlay = props.overlay.unwrap_or(false);
    let progress = props.progress.unwrap_or(0.0).clamp(0.0, 1.0);
    
    // Size-based dimensions
    let (spinner_size, dot_size, font_size) = match size {
        LoadingSize::Small => ("24px", "6px", "0.75rem"),
        LoadingSize::Medium => ("40px", "8px", "0.875rem"),
        LoadingSize::Large => ("56px", "12px", "1rem"),
        LoadingSize::ExtraLarge => ("80px", "16px", "1.125rem"),
    };
    
    let loading_content = match variant {
        LoadingVariant::Spinner => html! {
            <div class="loading-spinner" style={format!("width: {}; height: {};", spinner_size, spinner_size)}>
                <div class="spinner-ring"></div>
                <div class="spinner-ring"></div>
                <div class="spinner-ring"></div>
            </div>
        },
        
        LoadingVariant::Dots => html! {
            <div class="loading-dots">
                <div class="dot" style={format!("width: {}; height: {};", dot_size, dot_size)}></div>
                <div class="dot" style={format!("width: {}; height: {};", dot_size, dot_size)}></div>
                <div class="dot" style={format!("width: {}; height: {};", dot_size, dot_size)}></div>
                <div class="dot" style={format!("width: {}; height: {};", dot_size, dot_size)}></div>
            </div>
        },
        
        LoadingVariant::Pulse => html! {
            <div class="loading-pulse" style={format!("width: {}; height: {};", spinner_size, spinner_size)}>
                <div class="pulse-circle"></div>
                <div class="pulse-circle"></div>
                <div class="pulse-circle"></div>
            </div>
        },
        
        LoadingVariant::Wave => html! {
            <div class="loading-wave">
                <div class="wave-bar" style={format!("width: {}; height: {};", dot_size, spinner_size)}></div>
                <div class="wave-bar" style={format!("width: {}; height: {};", dot_size, spinner_size)}></div>
                <div class="wave-bar" style={format!("width: {}; height: {};", dot_size, spinner_size)}></div>
                <div class="wave-bar" style={format!("width: {}; height: {};", dot_size, spinner_size)}></div>
                <div class="wave-bar" style={format!("width: {}; height: {};", dot_size, spinner_size)}></div>
            </div>
        },
        
        LoadingVariant::Progress => html! {
            <div class="loading-progress">
                <div class="progress-track">
                    <div 
                        class="progress-fill"
                        style={format!("width: {}%;", progress * 100.0)}
                    ></div>
                </div>
                <Typography variant={TypographyVariant::Caption} color={TypographyColor::Secondary}>
                    {format!("{}%", (progress * 100.0) as u8)}
                </Typography>
            </div>
        },
        
        LoadingVariant::Skeleton => html! {
            <div class="loading-skeleton">
                <div class="skeleton-line skeleton-line-title"></div>
                <div class="skeleton-line skeleton-line-text"></div>
                <div class="skeleton-line skeleton-line-text"></div>
                <div class="skeleton-line skeleton-line-short"></div>
            </div>
        },
    };
    
    let container_style = format!(
        "{}{}",
        if overlay {
            "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(255, 255, 255, 0.9); z-index: 9999; display: flex; align-items: center; justify-content: center; backdrop-filter: blur(4px);"
        } else {
            "display: flex; flex-direction: column; align-items: center; justify-content: center; gap: var(--space-md);"
        },
        props.style.as_deref().unwrap_or("")
    );
    
    let container_classes = classes!(
        "loading-container",
        match variant {
            LoadingVariant::Spinner => "loading-spinner-container",
            LoadingVariant::Dots => "loading-dots-container",
            LoadingVariant::Pulse => "loading-pulse-container",
            LoadingVariant::Wave => "loading-wave-container",
            LoadingVariant::Progress => "loading-progress-container",
            LoadingVariant::Skeleton => "loading-skeleton-container",
        },
        match size {
            LoadingSize::Small => "loading-sm",
            LoadingSize::Medium => "loading-md",
            LoadingSize::Large => "loading-lg",
            LoadingSize::ExtraLarge => "loading-xl",
        },
        if overlay { "loading-overlay" } else { "" },
        props.class.clone()
    );

    html! {
        <div class={container_classes} style={container_style}>
            {loading_content}
            
            if let Some(message) = &props.message {
                <Typography 
                    variant={TypographyVariant::Body2} 
                    color={TypographyColor::Secondary}
                    style={format!("font-size: {}; text-align: center; margin-top: var(--space-sm);", font_size)}
                    class="loading-message thai-text"
                >
                    {message}
                </Typography>
            }
        </div>
    }
}

// Specialized loading components
#[derive(Properties, PartialEq)]
pub struct LoadingOverlayProps {
    pub message: Option<String>,
    pub progress: Option<f32>,
}

#[function_component(LoadingOverlay)]
pub fn loading_overlay(props: &LoadingOverlayProps) -> Html {
    html! {
        <Loading
            variant={LoadingVariant::Spinner}
            size={LoadingSize::Large}
            message={props.message.clone()}
            overlay={true}
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub progress: f32,
    pub message: Option<String>,
    pub color: Option<String>,
    pub class: Option<String>,
}

#[function_component(ProgressBar)]
pub fn progress_bar(props: &ProgressBarProps) -> Html {
    let theme = use_theme();
    let colors = &theme.colors;
    
    let progress = props.progress.clamp(0.0, 1.0);
    let color = props.color.as_deref().unwrap_or(colors.primary_electric_blue);
    
    html! {
        <div class={classes!("progress-bar-container", props.class.clone())}>
            if let Some(message) = &props.message {
                <Typography variant={TypographyVariant::Body2} color={TypographyColor::Primary} class="progress-message thai-text">
                    {message}
                </Typography>
            }
            
            <div class="progress-bar-track">
                <div 
                    class="progress-bar-fill"
                    style={format!("width: {}%; background: {};", progress * 100.0, color)}
                ></div>
            </div>
            
            <Typography variant={TypographyVariant::Caption} color={TypographyColor::Secondary} class="progress-percentage">
                {format!("{}%", (progress * 100.0) as u8)}
            </Typography>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SkeletonProps {
    pub lines: Option<usize>,
    pub avatar: Option<bool>,
    pub class: Option<String>,
}

#[function_component(Skeleton)]
pub fn skeleton(props: &SkeletonProps) -> Html {
    let lines = props.lines.unwrap_or(3);
    let avatar = props.avatar.unwrap_or(false);
    
    html! {
        <div class={classes!("skeleton-container", props.class.clone())}>
            if avatar {
                <div class="skeleton-avatar"></div>
            }
            
            <div class="skeleton-content">
                { for (0..lines).map(|i| {
                    let line_class = if i == 0 {
                        "skeleton-line skeleton-line-title"
                    } else if i == lines - 1 {
                        "skeleton-line skeleton-line-short"
                    } else {
                        "skeleton-line skeleton-line-text"
                    };
                    
                    html! {
                        <div class={line_class}></div>
                    }
                })}
            </div>
        </div>
    }
}

// CSS for loading animations
pub fn generate_loading_css() -> String {
    r#"/* Loading Animations - 2025 Design System */

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-md);
}

.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(8px);
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* Spinner Animation */
.loading-spinner {
  position: relative;
  display: inline-block;
}

.spinner-ring {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  border: 3px solid transparent;
  border-radius: 50%;
  animation: spinner-rotate 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
}

.spinner-ring:nth-child(1) {
  border-top-color: var(--color-primary-electric-blue);
  animation-delay: -0.45s;
}

.spinner-ring:nth-child(2) {
  border-top-color: var(--color-primary-vibrant-orange);
  animation-delay: -0.3s;
}

.spinner-ring:nth-child(3) {
  border-top-color: var(--color-primary-energetic-pink);
  animation-delay: -0.15s;
}

@keyframes spinner-rotate {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

/* Dots Animation */
.loading-dots {
  display: flex;
  gap: var(--space-xs);
  align-items: center;
}

.loading-dots .dot {
  border-radius: 50%;
  animation: dots-bounce 1.4s ease-in-out infinite both;
}

.loading-dots .dot:nth-child(1) {
  background: var(--color-primary-electric-blue);
  animation-delay: -0.32s;
}

.loading-dots .dot:nth-child(2) {
  background: var(--color-primary-vibrant-orange);
  animation-delay: -0.16s;
}

.loading-dots .dot:nth-child(3) {
  background: var(--color-primary-energetic-pink);
  animation-delay: 0s;
}

.loading-dots .dot:nth-child(4) {
  background: var(--color-accent-lime-green);
  animation-delay: 0.16s;
}

@keyframes dots-bounce {
  0%, 80%, 100% {
    transform: scale(0);
    opacity: 0.5;
  }
  40% {
    transform: scale(1);
    opacity: 1;
  }
}

/* Pulse Animation */
.loading-pulse {
  position: relative;
  display: inline-block;
}

.pulse-circle {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  border-radius: 50%;
  opacity: 0;
  animation: pulse-scale 2s ease-in-out infinite;
}

.pulse-circle:nth-child(1) {
  background: var(--color-primary-electric-blue);
  animation-delay: 0s;
}

.pulse-circle:nth-child(2) {
  background: var(--color-primary-vibrant-orange);
  animation-delay: 0.6s;
}

.pulse-circle:nth-child(3) {
  background: var(--color-primary-energetic-pink);
  animation-delay: 1.2s;
}

@keyframes pulse-scale {
  0% {
    transform: scale(0);
    opacity: 1;
  }
  100% {
    transform: scale(1);
    opacity: 0;
  }
}

/* Wave Animation */
.loading-wave {
  display: flex;
  gap: var(--space-xs);
  align-items: flex-end;
}

.wave-bar {
  border-radius: var(--radius-sm);
  animation: wave-stretch 1.2s ease-in-out infinite;
}

.wave-bar:nth-child(1) {
  background: var(--color-primary-electric-blue);
  animation-delay: -0.4s;
}

.wave-bar:nth-child(2) {
  background: var(--color-primary-vibrant-orange);
  animation-delay: -0.3s;
}

.wave-bar:nth-child(3) {
  background: var(--color-primary-energetic-pink);
  animation-delay: -0.2s;
}

.wave-bar:nth-child(4) {
  background: var(--color-accent-lime-green);
  animation-delay: -0.1s;
}

.wave-bar:nth-child(5) {
  background: var(--color-accent-purple);
  animation-delay: 0s;
}

@keyframes wave-stretch {
  0%, 40%, 100% {
    transform: scaleY(0.4);
  }
  20% {
    transform: scaleY(1);
  }
}

/* Progress Bar */
.loading-progress {
  width: 100%;
  max-width: 300px;
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.progress-track {
  width: 100%;
  height: 8px;
  background: var(--color-bg-light);
  border-radius: var(--radius-full);
  overflow: hidden;
  box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.1);
}

.progress-fill {
  height: 100%;
  background: var(--gradient-primary);
  border-radius: var(--radius-full);
  transition: width 0.3s ease;
  position: relative;
  overflow: hidden;
}

.progress-fill::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.3),
    transparent
  );
  animation: progress-shimmer 2s infinite;
}

@keyframes progress-shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

/* Progress Bar Component */
.progress-bar-container {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.progress-message {
  text-align: center;
}

.progress-bar-track {
  width: 100%;
  height: 12px;
  background: var(--color-bg-light);
  border-radius: var(--radius-full);
  overflow: hidden;
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
}

.progress-bar-fill {
  height: 100%;
  border-radius: var(--radius-full);
  transition: width 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
}

.progress-bar-fill::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.4),
    transparent
  );
  animation: progress-shimmer 2s infinite;
}

.progress-percentage {
  text-align: center;
  font-weight: var(--weight-semibold);
}

/* Skeleton Loading */
.skeleton-container {
  display: flex;
  gap: var(--space-md);
  align-items: flex-start;
  width: 100%;
}

.skeleton-avatar {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-full);
  background: var(--color-bg-light);
  flex-shrink: 0;
  animation: skeleton-pulse 2s ease-in-out infinite;
}

.skeleton-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.skeleton-line {
  height: 16px;
  background: var(--color-bg-light);
  border-radius: var(--radius-sm);
  animation: skeleton-pulse 2s ease-in-out infinite;
}

.skeleton-line-title {
  height: 20px;
  width: 60%;
}

.skeleton-line-text {
  width: 100%;
}

.skeleton-line-short {
  width: 40%;
}

@keyframes skeleton-pulse {
  0% {
    background: var(--color-bg-light);
  }
  50% {
    background: #e0e0e0;
  }
  100% {
    background: var(--color-bg-light);
  }
}

/* Loading Message */
.loading-message {
  text-align: center;
  max-width: 200px;
}

/* Responsive Design */
@media (max-width: 768px) {
  .loading-overlay {
    padding: var(--space-lg);
  }
  
  .loading-message {
    font-size: var(--text-sm);
  }
  
  .progress-bar-container {
    padding: 0 var(--space-md);
  }
}

/* Accessibility */
@media (prefers-reduced-motion: reduce) {
  .spinner-ring,
  .loading-dots .dot,
  .pulse-circle,
  .wave-bar,
  .progress-fill::after,
  .progress-bar-fill::after,
  .skeleton-line,
  .skeleton-avatar {
    animation: none;
  }
  
  .progress-fill,
  .progress-bar-fill {
    transition: none;
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .progress-track,
  .progress-bar-track {
    border: 1px solid var(--color-text-secondary);
  }
  
  .skeleton-line,
  .skeleton-avatar {
    border: 1px solid var(--color-text-disabled);
  }
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_variants() {
        let spinner = LoadingVariant::Spinner;
        let dots = LoadingVariant::Dots;
        assert_ne!(spinner, dots);
    }

    #[test]
    fn test_loading_sizes() {
        let small = LoadingSize::Small;
        let large = LoadingSize::Large;
        assert_ne!(small, large);
    }

    #[test]
    fn test_css_generation() {
        let css = generate_loading_css();
        assert!(css.contains("loading-spinner"));
        assert!(css.contains("@keyframes"));
        assert!(css.contains("dopamine"));
    }
}
