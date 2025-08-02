// Copyright (c) 2025 AI4Thai Crop Guardian Team
// Licensed under the MIT License

//! Unit test configuration and utilities for Rust components
//! Provides common test utilities, fixtures, and assertion helpers

use std::sync::Once;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use shared::{ChatMessage, Language, CropType, DiseaseSeverity};

static INIT: Once = Once::new();

/// Initialize test environment (logging, etc.)
pub fn init_test_env() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

/// Test fixture for creating chat messages
pub struct ChatMessageFixture;

impl ChatMessageFixture {
    pub fn user_message(content: &str) -> ChatMessage {
        ChatMessage {
            role: "user".to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn assistant_message(content: &str) -> ChatMessage {
        ChatMessage {
            role: "assistant".to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn system_message(content: &str) -> ChatMessage {
        ChatMessage {
            role: "system".to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
        }
    }
}

/// Test utilities for language operations
pub struct LanguageTestUtils;

impl LanguageTestUtils {
    pub fn all_supported_languages() -> Vec<Language> {
        vec![Language::Thai, Language::English]
    }

    pub fn validate_language_specific_content(content: &str, language: &Language) -> bool {
        match language {
            Language::Thai => content.chars().any(|c| {
                // Thai Unicode range: U+0E00–U+0E7F
                c >= '\u{0E00}' && c <= '\u{0E7F}'
            }),
            Language::English => content.chars().any(|c| c.is_ascii_alphabetic()),
        }
    }
}

/// Test utilities for crop type operations
pub struct CropTestUtils;

impl CropTestUtils {
    pub fn all_supported_crops() -> Vec<CropType> {
        vec![
            CropType::Rice,
            CropType::Cassava,
            CropType::Durian,
            CropType::Mango,
            CropType::Rubber,
        ]
    }

    pub fn get_test_diseases_for_crop(crop: &CropType) -> Vec<&'static str> {
        match crop {
            CropType::Rice => vec!["Rice Blast", "Brown Spot", "Leaf Blight"],
            CropType::Cassava => vec!["Cassava Mosaic Disease", "Brown Leaf Spot"],
            CropType::Durian => vec!["Phytophthora Fruit Rot", "Anthracnose"],
            CropType::Mango => vec!["Anthracnose", "Powdery Mildew", "Black Spot"],
            CropType::Rubber => vec!["Leaf Blight", "Root Rot", "Powdery Mildew"],
        }
    }
}

/// Test assertions for disease severity
pub struct SeverityTestUtils;

impl SeverityTestUtils {
    pub fn all_severity_levels() -> Vec<DiseaseSeverity> {
        vec![
            DiseaseSeverity::Low,
            DiseaseSeverity::Medium,
            DiseaseSeverity::High,
            DiseaseSeverity::Critical,
        ]
    }

    pub fn assert_severity_progression(from: &DiseaseSeverity, to: &DiseaseSeverity) -> bool {
        use DiseaseSeverity::*;
        matches!(
            (from, to),
            (Low, Medium) | (Low, High) | (Low, Critical) |
            (Medium, High) | (Medium, Critical) |
            (High, Critical)
        )
    }
}

/// Mock data generators for testing
pub struct MockDataGenerator;

impl MockDataGenerator {
    /// Generate a random UUID for testing
    pub fn random_uuid() -> Uuid {
        Uuid::new_v4()
    }

    /// Generate a test timestamp
    pub fn test_timestamp() -> DateTime<Utc> {
        Utc::now()
    }

    /// Generate mock image data (base64)
    pub fn mock_image_data() -> String {
        // 1x1 transparent PNG in base64
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==".to_string()
    }

    /// Generate mock Thai text
    pub fn mock_thai_text() -> String {
        "ใบไม้มีจุดสีน้ำตาล อาจเป็นโรคใบจุด".to_string()
    }

    /// Generate mock English text
    pub fn mock_english_text() -> String {
        "Leaves have brown spots, possibly leaf spot disease".to_string()
    }
}

/// Custom assertion macros for testing
#[macro_export]
macro_rules! assert_valid_uuid {
    ($val:expr) => {
        assert!($val.to_string().len() == 36, "Invalid UUID format: {}", $val);
    };
}

#[macro_export]
macro_rules! assert_valid_timestamp {
    ($val:expr) => {
        assert!($val <= Utc::now(), "Timestamp cannot be in the future: {}", $val);
    };
}

#[macro_export]
macro_rules! assert_language_content {
    ($content:expr, $language:expr) => {
        assert!(
            LanguageTestUtils::validate_language_specific_content($content, $language),
            "Content does not match expected language: {} for {:?}",
            $content,
            $language
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_fixture() {
        let msg = ChatMessageFixture::user_message("Test content");
        assert_eq!(msg.role, "user");
        assert_eq!(msg.content, "Test content");
        assert_valid_timestamp!(msg.timestamp);
    }

    #[test]
    fn test_language_validation() {
        assert!(LanguageTestUtils::validate_language_specific_content("Hello", &Language::English));
        assert!(LanguageTestUtils::validate_language_specific_content("สวัสดี", &Language::Thai));
    }

    #[test]
    fn test_crop_diseases() {
        let rice_diseases = CropTestUtils::get_test_diseases_for_crop(&CropType::Rice);
        assert!(!rice_diseases.is_empty());
        assert!(rice_diseases.contains(&"Rice Blast"));
    }

    #[test]
    fn test_severity_progression() {
        assert!(SeverityTestUtils::assert_severity_progression(
            &DiseaseSeverity::Low,
            &DiseaseSeverity::High
        ));
        assert!(!SeverityTestUtils::assert_severity_progression(
            &DiseaseSeverity::High,
            &DiseaseSeverity::Low
        ));
    }

    #[test]
    fn test_mock_data_generation() {
        let uuid = MockDataGenerator::random_uuid();
        assert_valid_uuid!(uuid);

        let timestamp = MockDataGenerator::test_timestamp();
        assert_valid_timestamp!(timestamp);

        let image_data = MockDataGenerator::mock_image_data();
        assert!(!image_data.is_empty());
    }
}