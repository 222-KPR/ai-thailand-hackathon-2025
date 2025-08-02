# Design System

This document outlines the comprehensive design system for AI4Thai Crop Guardian, implementing 2025 design trends with dopamine colors, modern typography, and bento grid layouts.

## 🎨 Color Palette: Dopamine Colors

### Primary Colors
```rust
pub struct DopamineColors {
    // Primary dopamine colors for main UI elements
    pub primary_electric_blue: &'static str = "#0066FF";
    pub primary_vibrant_orange: &'static str = "#FF6B35";
    pub primary_energetic_pink: &'static str = "#FF1B8D";
    
    // Accent colors for variety and emphasis
    pub accent_lime_green: &'static str = "#32D74B";
    pub accent_purple: &'static str = "#AF52DE";
    pub accent_yellow: &'static str = "#FFD60A";
    
    // Agricultural theme colors
    pub agricultural_green: &'static str = "#4CAF50";
    pub earth_brown: &'static str = "#8D6E63";
    pub sky_blue: &'static str = "#87CEEB";
    
    // Semantic colors
    pub success_green: &'static str = "#00C851";
    pub warning_amber: &'static str = "#FFB300";
    pub error_red: &'static str = "#FF3547";
    pub info_cyan: &'static str = "#00BCD4";
    
    // Neutral colors
    pub neutral_white: &'static str = "#FFFFFF";
    pub neutral_light_gray: &'static str = "#F5F5F5";
    pub neutral_gray: &'static str = "#9E9E9E";
    pub neutral_dark_gray: &'static str = "#424242";
    pub neutral_black: &'static str = "#212121";
}
```

### Color Usage Guidelines

#### Primary Actions
- **Electric Blue (#0066FF)**: Primary buttons, links, active states
- **Vibrant Orange (#FF6B35)**: Secondary actions, highlights, CTAs
- **Energetic Pink (#FF1B8D)**: Accent elements, notifications, badges

#### Agricultural Context
- **Agricultural Green (#4CAF50)**: Healthy crop indicators, success states
- **Earth Brown (#8D6E63)**: Soil-related content, grounding elements
- **Sky Blue (#87CEEB)**: Weather information, background elements

#### Semantic Meanings
- **Success**: Green tones for positive outcomes
- **Warning**: Amber/Orange for caution states
- **Error**: Red for error states and critical alerts
- **Info**: Cyan for informational content

## 🔤 Typography System

### Font Families
```scss
// Primary font stack
$font-primary: 'Poppins', 'Sarabun', -apple-system, BlinkMacSystemFont, sans-serif;

// Secondary font stack
$font-secondary: 'Inter', 'Sarabun', -apple-system, BlinkMacSystemFont, sans-serif;

// Thai language optimized
$font-thai: 'Sarabun', 'Prompt', 'Kanit', sans-serif;
```

### Typography Scale
```scss
// Font sizes (responsive)
$font-size-xs: clamp(0.75rem, 0.7rem + 0.25vw, 0.875rem);    // 12-14px
$font-size-sm: clamp(0.875rem, 0.8rem + 0.375vw, 1rem);      // 14-16px
$font-size-base: clamp(1rem, 0.9rem + 0.5vw, 1.125rem);      // 16-18px
$font-size-lg: clamp(1.125rem, 1rem + 0.625vw, 1.25rem);     // 18-20px
$font-size-xl: clamp(1.25rem, 1.1rem + 0.75vw, 1.5rem);      // 20-24px
$font-size-2xl: clamp(1.5rem, 1.3rem + 1vw, 2rem);           // 24-32px
$font-size-3xl: clamp(2rem, 1.7rem + 1.5vw, 2.5rem);         // 32-40px
$font-size-4xl: clamp(2.5rem, 2rem + 2.5vw, 3.5rem);         // 40-56px

// Line heights
$line-height-tight: 1.2;
$line-height-normal: 1.5;
$line-height-relaxed: 1.75;
$line-height-thai: 1.8; // Optimized for Thai characters
```

### Typography Components
```rust
// Heading component with dopamine colors
#[function_component(Heading)]
pub fn heading(props: &HeadingProps) -> Html {
    let class = classes!(
        "heading",
        format!("heading--{}", props.level),
        props.color.as_ref().map(|c| format!("heading--{}", c)),
        props.class.clone()
    );
    
    html! {
        <h1 class={class}>
            { &props.children }
        </h1>
    }
}

// Text component with Thai language support
#[function_component(Text)]
pub fn text(props: &TextProps) -> Html {
    let class = classes!(
        "text",
        format!("text--{}", props.size),
        if props.thai { Some("text--thai") } else { None },
        props.class.clone()
    );
    
    html! {
        <p class={class}>
            { &props.children }
        </p>
    }
}
```

## 📐 Spacing System

### Spacing Scale
```scss
// Base spacing unit: 4px
$spacing-base: 0.25rem; // 4px

// Spacing scale
$spacing-0: 0;
$spacing-1: #{$spacing-base * 1};    // 4px
$spacing-2: #{$spacing-base * 2};    // 8px
$spacing-3: #{$spacing-base * 3};    // 12px
$spacing-4: #{$spacing-base * 4};    // 16px
$spacing-5: #{$spacing-base * 5};    // 20px
$spacing-6: #{$spacing-base * 6};    // 24px
$spacing-8: #{$spacing-base * 8};    // 32px
$spacing-10: #{$spacing-base * 10};  // 40px
$spacing-12: #{$spacing-base * 12};  // 48px
$spacing-16: #{$spacing-base * 16};  // 64px
$spacing-20: #{$spacing-base * 20};  // 80px
$spacing-24: #{$spacing-base * 24};  // 96px
$spacing-32: #{$spacing-base * 32};  // 128px
```

### Layout Utilities
```scss
// Margin utilities
.m-0 { margin: $spacing-0; }
.m-1 { margin: $spacing-1; }
.m-2 { margin: $spacing-2; }
// ... continue for all spacing values

// Padding utilities
.p-0 { padding: $spacing-0; }
.p-1 { padding: $spacing-1; }
.p-2 { padding: $spacing-2; }
// ... continue for all spacing values

// Gap utilities for flexbox/grid
.gap-0 { gap: $spacing-0; }
.gap-1 { gap: $spacing-1; }
.gap-2 { gap: $spacing-2; }
// ... continue for all spacing values
```

## 🎛️ Bento Grid System

### Grid Configuration
```scss
// Bento grid container
.bento-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: $spacing-6;
    padding: $spacing-6;
    
    @media (min-width: 768px) {
        grid-template-columns: repeat(12, 1fr);
        gap: $spacing-8;
        padding: $spacing-8;
    }
}

// Bento card sizes
.bento-card {
    background: $neutral-white;
    border-radius: 16px;
    padding: $spacing-6;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    transition: all 0.3s ease;
    
    &:hover {
        transform: translateY(-2px);
        box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
    }
    
    // Size variants
    &--small { grid-column: span 1; grid-row: span 1; }
    &--medium { grid-column: span 2; grid-row: span 1; }
    &--large { grid-column: span 2; grid-row: span 2; }
    &--wide { grid-column: span 3; grid-row: span 1; }
    &--tall { grid-column: span 1; grid-row: span 2; }
    &--hero { grid-column: span 3; grid-row: span 2; }
    
    @media (max-width: 767px) {
        &--small,
        &--medium,
        &--large,
        &--wide,
        &--tall,
        &--hero {
            grid-column: span 1;
            grid-row: span 1;
        }
    }
}
```

### Bento Grid Component
```rust
#[derive(Properties, PartialEq)]
pub struct BentoGridProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(BentoGrid)]
pub fn bento_grid(props: &BentoGridProps) -> Html {
    html! {
        <div class={classes!("bento-grid", props.class.clone())}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BentoCardProps {
    pub children: Children,
    #[prop_or(BentoSize::Medium)]
    pub size: BentoSize,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub color: Option<String>,
}

#[derive(PartialEq, Clone)]
pub enum BentoSize {
    Small,
    Medium,
    Large,
    Wide,
    Tall,
    Hero,
}

#[function_component(BentoCard)]
pub fn bento_card(props: &BentoCardProps) -> Html {
    let size_class = match props.size {
        BentoSize::Small => "bento-card--small",
        BentoSize::Medium => "bento-card--medium",
        BentoSize::Large => "bento-card--large",
        BentoSize::Wide => "bento-card--wide",
        BentoSize::Tall => "bento-card--tall",
        BentoSize::Hero => "bento-card--hero",
    };
    
    let style = props.color.as_ref().map(|color| {
        format!("--card-accent-color: {}", color)
    });
    
    html! {
        <div 
            class={classes!("bento-card", size_class, props.class.clone())}
            style={style}
        >
            { for props.children.iter() }
        </div>
    }
}
```

## 🎯 Component Library

### Button Components
```rust
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Children,
    #[prop_or(ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[prop_or(ButtonSize::Medium)]
    pub size: ButtonSize,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Error,
    Ghost,
}

#[derive(PartialEq, Clone)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let variant_class = match props.variant {
        ButtonVariant::Primary => "btn--primary",
        ButtonVariant::Secondary => "btn--secondary",
        ButtonVariant::Accent => "btn--accent",
        ButtonVariant::Success => "btn--success",
        ButtonVariant::Warning => "btn--warning",
        ButtonVariant::Error => "btn--error",
        ButtonVariant::Ghost => "btn--ghost",
    };
    
    let size_class = match props.size {
        ButtonSize::Small => "btn--sm",
        ButtonSize::Medium => "btn--md",
        ButtonSize::Large => "btn--lg",
    };
    
    html! {
        <button
            class={classes!(
                "btn",
                variant_class,
                size_class,
                if props.disabled { Some("btn--disabled") } else { None },
                if props.loading { Some("btn--loading") } else { None },
                props.class.clone()
            )}
            disabled={props.disabled || props.loading}
            onclick={props.onclick.clone()}
        >
            if props.loading {
                <span class="btn__spinner"></span>
            }
            <span class="btn__content">
                { for props.children.iter() }
            </span>
        </button>
    }
}
```

### Status Card Component
```rust
#[derive(Properties, PartialEq)]
pub struct StatusCardProps {
    pub title: String,
    pub value: String,
    pub trend: Option<TrendDirection>,
    pub color: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or_default]
    pub icon: Option<String>,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(PartialEq, Clone)]
pub enum TrendDirection {
    Up,
    Down,
    Neutral,
}

#[function_component(StatusCard)]
pub fn status_card(props: &StatusCardProps) -> Html {
    let trend_class = props.trend.as_ref().map(|trend| match trend {
        TrendDirection::Up => "status-card--trend-up",
        TrendDirection::Down => "status-card--trend-down",
        TrendDirection::Neutral => "status-card--trend-neutral",
    });
    
    html! {
        <div 
            class={classes!(
                "status-card",
                trend_class,
                props.class.clone()
            )}
            style={format!("--card-color: {}", props.color)}
        >
            <div class="status-card__header">
                if let Some(icon) = &props.icon {
                    <div class="status-card__icon">
                        <i class={format!("icon-{}", icon)}></i>
                    </div>
                }
                <h3 class="status-card__title">{ &props.title }</h3>
            </div>
            
            <div class="status-card__content">
                <div class="status-card__value">{ &props.value }</div>
                if let Some(subtitle) = &props.subtitle {
                    <div class="status-card__subtitle">{ subtitle }</div>
                }
            </div>
            
            if let Some(trend) = &props.trend {
                <div class="status-card__trend">
                    { match trend {
                        TrendDirection::Up => "↗",
                        TrendDirection::Down => "↘",
                        TrendDirection::Neutral => "→",
                    }}
                </div>
            }
        </div>
    }
}
```

## 🎭 Animation System

### Micro-interactions
```scss
// Transition utilities
.transition-all { transition: all 0.3s ease; }
.transition-colors { transition: color 0.3s ease, background-color 0.3s ease, border-color 0.3s ease; }
.transition-transform { transition: transform 0.3s ease; }
.transition-opacity { transition: opacity 0.3s ease; }

// Hover effects
.hover-lift {
    transition: transform 0.3s ease, box-shadow 0.3s ease;
    
    &:hover {
        transform: translateY(-2px);
        box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
    }
}

.hover-scale {
    transition: transform 0.3s ease;
    
    &:hover {
        transform: scale(1.05);
    }
}

// Loading animations
@keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
}

@keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
}

@keyframes shimmer {
    0% { background-position: -200px 0; }
    100% { background-position: calc(200px + 100%) 0; }
}

.animate-pulse { animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite; }
.animate-spin { animation: spin 1s linear infinite; }
.animate-shimmer { animation: shimmer 1.5s ease-in-out infinite; }
```

## 📱 Responsive Design

### Breakpoints
```scss
// Breakpoint system
$breakpoints: (
    xs: 0,
    sm: 576px,
    md: 768px,
    lg: 992px,
    xl: 1200px,
    xxl: 1400px
);

// Media query mixins
@mixin media-up($breakpoint) {
    @media (min-width: map-get($breakpoints, $breakpoint)) {
        @content;
    }
}

@mixin media-down($breakpoint) {
    @media (max-width: map-get($breakpoints, $breakpoint) - 1px) {
        @content;
    }
}

@mixin media-between($lower, $upper) {
    @media (min-width: map-get($breakpoints, $lower)) and (max-width: map-get($breakpoints, $upper) - 1px) {
        @content;
    }
}
```

### Container System
```scss
.container {
    width: 100%;
    margin: 0 auto;
    padding: 0 $spacing-4;
    
    @include media-up(sm) { max-width: 540px; }
    @include media-up(md) { max-width: 720px; }
    @include media-up(lg) { max-width: 960px; }
    @include media-up(xl) { max-width: 1140px; }
    @include media-up(xxl) { max-width: 1320px; }
}

.container-fluid {
    width: 100%;
    padding: 0 $spacing-4;
}
```

## 🌏 Thai Language Optimization

### Typography Adjustments
```scss
.text--thai {
    font-family: $font-thai;
    line-height: $line-height-thai;
    
    // Improved readability for Thai characters
    letter-spacing: 0.01em;
    word-spacing: 0.1em;
    
    // Better rendering on different devices
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-rendering: optimizeLegibility;
}

// Thai-specific heading styles
.heading--thai {
    font-family: $font-thai;
    line-height: 1.4;
    font-weight: 600;
}
```

### Cultural Color Considerations
```scss
// Colors that resonate with Thai culture
$thai-gold: #FFD700;
$thai-red: #DC143C;
$thai-blue: #0066CC;
$thai-green: #228B22;

// Agricultural context colors
$rice-green: #9ACD32;
$soil-brown: #8B4513;
$water-blue: #4682B4;
$sun-yellow: #FFD700;
```

This design system provides a comprehensive foundation for building a modern, accessible, and culturally appropriate interface for Thai farmers while maintaining the vibrant, energetic aesthetic of dopamine colors and the flexible organization of bento grids.
