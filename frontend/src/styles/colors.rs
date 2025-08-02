// Color system for AI4Thai Crop Guardian - 2025 Design
// Implements dopamine color palette for positive user experience

use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ColorPalette {
    // Primary dopamine colors - vibrant and energetic
    pub primary_electric_blue: &'static str,
    pub primary_vibrant_orange: &'static str,
    pub primary_energetic_pink: &'static str,
    
    // Accent colors for variety and visual interest
    pub accent_lime_green: &'static str,
    pub accent_purple: &'static str,
    pub accent_yellow: &'static str,
    
    // Balanced backgrounds for readability
    pub bg_light: &'static str,
    pub bg_dark: &'static str,
    pub surface_light: &'static str,
    pub surface_dark: &'static str,
    
    // Semantic colors
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
    pub info: &'static str,
    
    // Text colors
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_disabled: &'static str,
    pub text_inverse: &'static str,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            // Primary dopamine colors
            primary_electric_blue: "#0066FF",
            primary_vibrant_orange: "#FF6B35",
            primary_energetic_pink: "#FF1B8D",
            
            // Accent colors
            accent_lime_green: "#32D74B",
            accent_purple: "#AF52DE",
            accent_yellow: "#FFD60A",
            
            // Backgrounds
            bg_light: "#FAFAFA",
            bg_dark: "#1C1C1E",
            surface_light: "#FFFFFF",
            surface_dark: "#2C2C2E",
            
            // Semantic colors
            success: "#32D74B",
            warning: "#FFD60A",
            error: "#FF453A",
            info: "#0066FF",
            
            // Text colors
            text_primary: "#1C1C1E",
            text_secondary: "#6B7280",
            text_disabled: "#9CA3AF",
            text_inverse: "#FFFFFF",
        }
    }
}

// Color utility functions
impl ColorPalette {
    pub fn get_gradient(&self, color1: &str, color2: &str, direction: u16) -> String {
        format!("linear-gradient({}deg, {}, {})", direction, color1, color2)
    }
    
    pub fn get_primary_gradient(&self) -> String {
        self.get_gradient(self.primary_electric_blue, self.primary_energetic_pink, 135)
    }
    
    pub fn get_success_gradient(&self) -> String {
        self.get_gradient(self.accent_lime_green, self.primary_electric_blue, 135)
    }
    
    pub fn get_warning_gradient(&self) -> String {
        self.get_gradient(self.accent_yellow, self.primary_vibrant_orange, 135)
    }
    
    pub fn with_opacity(&self, color: &str, opacity: f32) -> String {
        // Convert hex to rgba with opacity
        if let Ok(hex) = u32::from_str_radix(&color[1..], 16) {
            let r = (hex >> 16) & 0xFF;
            let g = (hex >> 8) & 0xFF;
            let b = hex & 0xFF;
            format!("rgba({}, {}, {}, {})", r, g, b, opacity)
        } else {
            color.to_string()
        }
    }
}

// CSS custom properties generator
pub fn generate_css_variables(palette: &ColorPalette) -> String {
    format!(
        r#":root {{
  /* Primary dopamine colors */
  --color-primary-electric-blue: {};
  --color-primary-vibrant-orange: {};
  --color-primary-energetic-pink: {};
  
  /* Accent colors */
  --color-accent-lime-green: {};
  --color-accent-purple: {};
  --color-accent-yellow: {};
  
  /* Backgrounds */
  --color-bg-light: {};
  --color-bg-dark: {};
  --color-surface-light: {};
  --color-surface-dark: {};
  
  /* Semantic colors */
  --color-success: {};
  --color-warning: {};
  --color-error: {};
  --color-info: {};
  
  /* Text colors */
  --color-text-primary: {};
  --color-text-secondary: {};
  --color-text-disabled: {};
  --color-text-inverse: {};
  
  /* Gradients */
  --gradient-primary: {};
  --gradient-success: {};
  --gradient-warning: {};
  
  /* Shadows with dopamine colors */
  --shadow-primary: 0 4px 20px {};
  --shadow-success: 0 4px 20px {};
  --shadow-warning: 0 4px 20px {};
  --shadow-hover: 0 8px 30px rgba(0, 0, 0, 0.12);
}}"#,
        palette.primary_electric_blue,
        palette.primary_vibrant_orange,
        palette.primary_energetic_pink,
        palette.accent_lime_green,
        palette.accent_purple,
        palette.accent_yellow,
        palette.bg_light,
        palette.bg_dark,
        palette.surface_light,
        palette.surface_dark,
        palette.success,
        palette.warning,
        palette.error,
        palette.info,
        palette.text_primary,
        palette.text_secondary,
        palette.text_disabled,
        palette.text_inverse,
        palette.get_primary_gradient(),
        palette.get_success_gradient(),
        palette.get_warning_gradient(),
        palette.with_opacity(palette.primary_electric_blue, 0.2),
        palette.with_opacity(palette.accent_lime_green, 0.2),
        palette.with_opacity(palette.accent_yellow, 0.2),
    )
}

// Theme context for components
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub colors: ColorPalette,
    pub is_dark_mode: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: ColorPalette::default(),
            is_dark_mode: false,
        }
    }
}

// Theme context provider
pub type ThemeContext = UseStateHandle<Theme>;

#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    pub children: Children,
    pub theme: Option<Theme>,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    let theme = use_state(|| props.theme.clone().unwrap_or_default());
    
    html! {
        <ContextProvider<ThemeContext> context={theme}>
            { for props.children.iter() }
        </ContextProvider<ThemeContext>>
    }
}

// Hook to use theme in components
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("Theme context not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_palette_default() {
        let palette = ColorPalette::default();
        assert_eq!(palette.primary_electric_blue, "#0066FF");
        assert_eq!(palette.primary_vibrant_orange, "#FF6B35");
        assert_eq!(palette.primary_energetic_pink, "#FF1B8D");
    }

    #[test]
    fn test_gradient_generation() {
        let palette = ColorPalette::default();
        let gradient = palette.get_primary_gradient();
        assert!(gradient.contains("linear-gradient"));
        assert!(gradient.contains("#0066FF"));
        assert!(gradient.contains("#FF1B8D"));
    }

    #[test]
    fn test_opacity_conversion() {
        let palette = ColorPalette::default();
        let rgba = palette.with_opacity("#0066FF", 0.5);
        assert!(rgba.contains("rgba"));
        assert!(rgba.contains("0.5"));
    }
}
