// Design Tokens for AI4Thai Crop Guardian - 2025 Design System
// Centralized design system tokens for consistent styling

use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DesignTokens {
    pub spacing: SpacingScale,
    pub sizing: SizingScale,
    pub borders: BorderScale,
    pub shadows: ShadowScale,
    pub animations: AnimationScale,
    pub breakpoints: BreakpointScale,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpacingScale {
    // Base spacing unit: 0.25rem (4px)
    pub xs: &'static str,      // 4px
    pub sm: &'static str,      // 8px
    pub md: &'static str,      // 16px
    pub lg: &'static str,      // 24px
    pub xl: &'static str,      // 32px
    pub xl2: &'static str,     // 48px
    pub xl3: &'static str,     // 64px
    pub xl4: &'static str,     // 80px
    pub xl5: &'static str,     // 96px
}

#[derive(Debug, Clone, PartialEq)]
pub struct SizingScale {
    // Common sizing values
    pub xs: &'static str,      // 20rem
    pub sm: &'static str,      // 24rem
    pub md: &'static str,      // 28rem
    pub lg: &'static str,      // 32rem
    pub xl: &'static str,      // 36rem
    pub xl2: &'static str,     // 42rem
    pub xl3: &'static str,     // 48rem
    pub xl4: &'static str,     // 56rem
    pub xl5: &'static str,     // 64rem
    pub xl6: &'static str,     // 72rem
    pub full: &'static str,    // 100%
    pub screen: &'static str,  // 100vw
}

#[derive(Debug, Clone, PartialEq)]
pub struct BorderScale {
    pub width_thin: &'static str,    // 1px
    pub width_normal: &'static str,  // 2px
    pub width_thick: &'static str,   // 4px

    pub radius_none: &'static str,   // 0
    pub radius_sm: &'static str,     // 4px
    pub radius_md: &'static str,     // 8px
    pub radius_lg: &'static str,     // 12px
    pub radius_xl: &'static str,     // 16px
    pub radius_xl2: &'static str,    // 24px
    pub radius_full: &'static str,   // 9999px
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShadowScale {
    pub none: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
    pub xl2: &'static str,
    pub inner: &'static str,

    // Dopamine-colored shadows
    pub primary: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationScale {
    // Duration
    pub duration_fast: &'static str,     // 150ms
    pub duration_normal: &'static str,   // 300ms
    pub duration_slow: &'static str,     // 500ms
    pub duration_slower: &'static str,   // 750ms

    // Easing functions
    pub ease_linear: &'static str,
    pub ease_in: &'static str,
    pub ease_out: &'static str,
    pub ease_in_out: &'static str,
    pub ease_bounce: &'static str,
    pub ease_elastic: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakpointScale {
    pub xs: &'static str,    // 475px
    pub sm: &'static str,    // 640px
    pub md: &'static str,    // 768px
    pub lg: &'static str,    // 1024px
    pub xl: &'static str,    // 1280px
    pub xl2: &'static str,   // 1536px
}

impl Default for DesignTokens {
    fn default() -> Self {
        Self {
            spacing: SpacingScale::default(),
            sizing: SizingScale::default(),
            borders: BorderScale::default(),
            shadows: ShadowScale::default(),
            animations: AnimationScale::default(),
            breakpoints: BreakpointScale::default(),
        }
    }
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            xs: "0.25rem",   // 4px
            sm: "0.5rem",    // 8px
            md: "1rem",      // 16px
            lg: "1.5rem",    // 24px
            xl: "2rem",      // 32px
            xl2: "3rem",     // 48px
            xl3: "4rem",     // 64px
            xl4: "5rem",     // 80px
            xl5: "6rem",     // 96px
        }
    }
}

impl Default for SizingScale {
    fn default() -> Self {
        Self {
            xs: "20rem",     // 320px
            sm: "24rem",     // 384px
            md: "28rem",     // 448px
            lg: "32rem",     // 512px
            xl: "36rem",     // 576px
            xl2: "42rem",    // 672px
            xl3: "48rem",    // 768px
            xl4: "56rem",    // 896px
            xl5: "64rem",    // 1024px
            xl6: "72rem",    // 1152px
            full: "100%",
            screen: "100vw",
        }
    }
}

impl Default for BorderScale {
    fn default() -> Self {
        Self {
            width_thin: "1px",
            width_normal: "2px",
            width_thick: "4px",

            radius_none: "0",
            radius_sm: "4px",
            radius_md: "8px",
            radius_lg: "12px",
            radius_xl: "16px",
            radius_xl2: "24px",
            radius_full: "9999px",
        }
    }
}

impl Default for ShadowScale {
    fn default() -> Self {
        Self {
            none: "none",
            sm: "0 1px 2px 0 rgba(0, 0, 0, 0.05)",
            md: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)",
            lg: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)",
            xl: "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)",
            xl2: "0 25px 50px -12px rgba(0, 0, 0, 0.25)",
            inner: "inset 0 2px 4px 0 rgba(0, 0, 0, 0.06)",

            // Dopamine-colored shadows
            primary: "0 4px 20px rgba(0, 102, 255, 0.3)",
            success: "0 4px 20px rgba(50, 215, 75, 0.3)",
            warning: "0 4px 20px rgba(255, 214, 10, 0.3)",
            error: "0 4px 20px rgba(255, 69, 58, 0.3)",
        }
    }
}

impl Default for AnimationScale {
    fn default() -> Self {
        Self {
            duration_fast: "150ms",
            duration_normal: "300ms",
            duration_slow: "500ms",
            duration_slower: "750ms",

            ease_linear: "linear",
            ease_in: "cubic-bezier(0.4, 0, 1, 1)",
            ease_out: "cubic-bezier(0, 0, 0.2, 1)",
            ease_in_out: "cubic-bezier(0.4, 0, 0.2, 1)",
            ease_bounce: "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
            ease_elastic: "cubic-bezier(0.175, 0.885, 0.32, 1.275)",
        }
    }
}

impl Default for BreakpointScale {
    fn default() -> Self {
        Self {
            xs: "475px",
            sm: "640px",
            md: "768px",
            lg: "1024px",
            xl: "1280px",
            xl2: "1536px",
        }
    }
}

// Utility functions for design tokens
impl DesignTokens {
    pub fn get_spacing(&self, size: &str) -> &str {
        match size {
            "xs" => self.spacing.xs,
            "sm" => self.spacing.sm,
            "md" => self.spacing.md,
            "lg" => self.spacing.lg,
            "xl" => self.spacing.xl,
            "2xl" => self.spacing.xl2,
            "3xl" => self.spacing.xl3,
            "4xl" => self.spacing.xl4,
            "5xl" => self.spacing.xl5,
            _ => self.spacing.md,
        }
    }

    pub fn get_border_radius(&self, size: &str) -> &str {
        match size {
            "none" => self.borders.radius_none,
            "sm" => self.borders.radius_sm,
            "md" => self.borders.radius_md,
            "lg" => self.borders.radius_lg,
            "xl" => self.borders.radius_xl,
            "2xl" => self.borders.radius_xl2,
            "full" => self.borders.radius_full,
            _ => self.borders.radius_md,
        }
    }

    pub fn get_shadow(&self, variant: &str) -> &str {
        match variant {
            "none" => self.shadows.none,
            "sm" => self.shadows.sm,
            "md" => self.shadows.md,
            "lg" => self.shadows.lg,
            "xl" => self.shadows.xl,
            "2xl" => self.shadows.xl2,
            "inner" => self.shadows.inner,
            "primary" => self.shadows.primary,
            "success" => self.shadows.success,
            "warning" => self.shadows.warning,
            "error" => self.shadows.error,
            _ => self.shadows.md,
        }
    }

    pub fn get_animation_duration(&self, speed: &str) -> &str {
        match speed {
            "fast" => self.animations.duration_fast,
            "normal" => self.animations.duration_normal,
            "slow" => self.animations.duration_slow,
            "slower" => self.animations.duration_slower,
            _ => self.animations.duration_normal,
        }
    }

    pub fn get_animation_easing(&self, easing: &str) -> &str {
        match easing {
            "linear" => self.animations.ease_linear,
            "in" => self.animations.ease_in,
            "out" => self.animations.ease_out,
            "in-out" => self.animations.ease_in_out,
            "bounce" => self.animations.ease_bounce,
            "elastic" => self.animations.ease_elastic,
            _ => self.animations.ease_in_out,
        }
    }
}

// CSS custom properties generator
pub fn generate_design_tokens_css(tokens: &DesignTokens) -> String {
    format!(
        r#":root {{
  /* Spacing Scale */
  --space-xs: {};
  --space-sm: {};
  --space-md: {};
  --space-lg: {};
  --space-xl: {};
  --space-2xl: {};
  --space-3xl: {};
  --space-4xl: {};
  --space-5xl: {};

  /* Sizing Scale */
  --size-xs: {};
  --size-sm: {};
  --size-md: {};
  --size-lg: {};
  --size-xl: {};
  --size-2xl: {};
  --size-3xl: {};
  --size-4xl: {};
  --size-5xl: {};
  --size-6xl: {};
  --size-full: {};
  --size-screen: {};

  /* Border Scale */
  --border-width-thin: {};
  --border-width-normal: {};
  --border-width-thick: {};
  --radius-none: {};
  --radius-sm: {};
  --radius-md: {};
  --radius-lg: {};
  --radius-xl: {};
  --radius-2xl: {};
  --radius-full: {};

  /* Shadow Scale */
  --shadow-none: {};
  --shadow-sm: {};
  --shadow-md: {};
  --shadow-lg: {};
  --shadow-xl: {};
  --shadow-2xl: {};
  --shadow-inner: {};
  --shadow-primary: {};
  --shadow-success: {};
  --shadow-warning: {};
  --shadow-error: {};

  /* Animation Scale */
  --duration-fast: {};
  --duration-normal: {};
  --duration-slow: {};
  --duration-slower: {};
  --ease-linear: {};
  --ease-in: {};
  --ease-out: {};
  --ease-in-out: {};
  --ease-bounce: {};
  --ease-elastic: {};

  /* Breakpoints */
  --breakpoint-xs: {};
  --breakpoint-sm: {};
  --breakpoint-md: {};
  --breakpoint-lg: {};
  --breakpoint-xl: {};
  --breakpoint-2xl: {};
}}"#,
        tokens.spacing.xs, tokens.spacing.sm, tokens.spacing.md, tokens.spacing.lg,
        tokens.spacing.xl, tokens.spacing.xl2, tokens.spacing.xl3, tokens.spacing.xl4, tokens.spacing.xl5,

        tokens.sizing.xs, tokens.sizing.sm, tokens.sizing.md, tokens.sizing.lg,
        tokens.sizing.xl, tokens.sizing.xl2, tokens.sizing.xl3, tokens.sizing.xl4,
        tokens.sizing.xl5, tokens.sizing.xl6, tokens.sizing.full, tokens.sizing.screen,

        tokens.borders.width_thin, tokens.borders.width_normal, tokens.borders.width_thick,
        tokens.borders.radius_none, tokens.borders.radius_sm, tokens.borders.radius_md,
        tokens.borders.radius_lg, tokens.borders.radius_xl, tokens.borders.radius_xl2, tokens.borders.radius_full,

        tokens.shadows.none, tokens.shadows.sm, tokens.shadows.md, tokens.shadows.lg,
        tokens.shadows.xl, tokens.shadows.xl2, tokens.shadows.inner,
        tokens.shadows.primary, tokens.shadows.success, tokens.shadows.warning, tokens.shadows.error,

        tokens.animations.duration_fast, tokens.animations.duration_normal,
        tokens.animations.duration_slow, tokens.animations.duration_slower,
        tokens.animations.ease_linear, tokens.animations.ease_in, tokens.animations.ease_out,
        tokens.animations.ease_in_out, tokens.animations.ease_bounce, tokens.animations.ease_elastic,

        tokens.breakpoints.xs, tokens.breakpoints.sm, tokens.breakpoints.md,
        tokens.breakpoints.lg, tokens.breakpoints.xl, tokens.breakpoints.xl2,
    )
}

// Hook to use design tokens in components
#[hook]
pub fn use_design_tokens() -> DesignTokens {
    DesignTokens::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_tokens_default() {
        let tokens = DesignTokens::default();
        assert_eq!(tokens.spacing.md, "1rem");
        assert_eq!(tokens.borders.radius_lg, "12px");
        assert_eq!(tokens.animations.duration_normal, "300ms");
    }

    #[test]
    fn test_spacing_getter() {
        let tokens = DesignTokens::default();
        assert_eq!(tokens.get_spacing("lg"), "1.5rem");
        assert_eq!(tokens.get_spacing("invalid"), "1rem"); // fallback to md
    }

    #[test]
    fn test_css_generation() {
        let tokens = DesignTokens::default();
        let css = generate_design_tokens_css(&tokens);
        assert!(css.contains("--space-md: 1rem"));
        assert!(css.contains("--radius-lg: 12px"));
        assert!(css.contains("--duration-normal: 300ms"));
    }
}
