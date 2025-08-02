use wasm_bindgen::prelude::*;

pub mod app;
pub mod components;
pub mod services;
pub mod utils;
pub mod i18n;
pub mod styles;

pub use app::App;

#[wasm_bindgen(start)]
pub fn main() {
    // Initialize logging
    wasm_logger::init(wasm_logger::Config::default());
    
    // Initialize panic hook for better error reporting
    console_error_panic_hook::set_once();
    
    log::info!("AI4Thai Crop Guardian frontend starting...");
    
    // Mount the Yew app
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{Language, ChatMessage};
    use chrono::Utc;

    #[test]
    fn test_app_state_creation() {
        let state = app::AppState::default();
        assert_eq!(state.language, Language::Thai);
        assert_eq!(state.messages.len(), 0);
        assert!(!state.is_loading);
        assert!(state.error_message.is_none());
    }

    #[test]
    fn test_chat_message_creation() {
        let message = ChatMessage {
            role: "user".to_string(),
            content: "Test message".to_string(),
            timestamp: Utc::now(),
        };
        
        assert_eq!(message.role, "user");
        assert_eq!(message.content, "Test message");
    }

    #[test]
    fn test_i18n_context_creation() {
        let context = i18n::I18nContext::new(Language::Thai);
        assert_eq!(context.language, Language::Thai);
        
        let thai_text = context.t("welcome.title");
        assert!(!thai_text.is_empty());
        
        let context = i18n::I18nContext::new(Language::English);
        let english_text = context.t("welcome.title");
        assert!(!english_text.is_empty());
        assert_ne!(thai_text, english_text);
    }

    #[test]
    fn test_image_validation_utils() {
        use utils::image::*;
        
        assert_eq!(get_file_extension("test.jpg"), Some("jpg".to_string()));
        assert_eq!(get_file_extension("test.PNG"), Some("png".to_string()));
        assert_eq!(get_file_extension("noextension"), Some("noextension".to_string())); // Returns the whole filename if no dot
        
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_text_validation() {
        use utils::validation::*;
        
        assert!(validate_text_input("Valid text").is_ok());
        assert!(validate_text_input("").is_err());
        assert!(validate_text_input(&" ".repeat(1001)).is_err());
        
        assert!(validate_crop_type("rice"));
        assert!(validate_crop_type("cassava"));
        assert!(!validate_crop_type("invalidcrop"));
        
        assert!(validate_language("thai"));
        assert!(validate_language("english"));
        assert!(!validate_language("fr"));
    }

    #[test]
    fn test_sanitize_text() {
        use utils::validation::*;
        
        let clean_text = sanitize_text_input("Normal text");
        assert_eq!(clean_text, "Normal text");
        
        let script_text = sanitize_text_input("<script>alert('xss')</script>");
        assert!(!script_text.contains("<script>"));
        
        let html_text = sanitize_text_input("<div>content</div>");
        assert!(!html_text.contains("<div>"));
    }

    #[test]
    fn test_design_system_colors() {
        use styles::colors::*;
        
        assert_eq!(PrimaryColors::ELECTRIC_BLUE, "#0066FF");
        assert_eq!(PrimaryColors::VIBRANT_ORANGE, "#FF6B35");
        assert_eq!(PrimaryColors::ENERGETIC_PINK, "#FF1B8D");
        
        assert_eq!(ColorPalette::primary(0), "#0066FF");
        assert_eq!(ColorPalette::accent(0), "#32D74B");
        assert_eq!(ColorPalette::semantic("success"), "#34C759");
    }

    #[test]
    fn test_typography_system() {
        use styles::typography::*;
        
        assert!(FontFamilies::HEADING.contains("Poppins"));
        assert!(FontFamilies::BODY.contains("Inter"));
        assert!(FontFamilies::THAI.contains("Sarabun"));
        
        assert_eq!(FontWeights::REGULAR, 400);
        assert_eq!(FontWeights::BOLD, 700);
        
        let hero_style = TextStyles::hero();
        assert!(hero_style.contains("Poppins"));
        assert!(hero_style.contains("700"));
    }

    #[test]
    fn test_spacing_system() {
        use styles::spacing::*;
        
        assert_eq!(Spacing::NONE, "0");
        assert_eq!(Spacing::XL, "1rem");
        assert_eq!(Spacing::SECTION, "2rem");
        
        assert_eq!(Breakpoints::SM, "640px");
        assert_eq!(Breakpoints::LG, "1024px");
        
        assert_eq!(ZIndex::BASE, 0);
        assert!(ZIndex::MODAL > ZIndex::DROPDOWN);
        
        let center = Layout::flex_center();
        assert!(center.contains("display: flex"));
        assert!(center.contains("align-items: center"));
    }
}