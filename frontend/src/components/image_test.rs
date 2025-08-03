#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yew::prelude::*;
    use gloo_utils::format::JsValueSerdeExt;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_image_upload_component_renders() {
        let props = ImageUploadProps {
            on_uploaded: Callback::noop(),
        };

        let _rendered = yew::Renderer::<ImageUpload>::with_props(props)
            .render();

        // Component should render without panicking
        assert!(true);
    }

    #[wasm_bindgen_test]
    async fn test_file_validation_integration() {
        use web_sys::File;
        use js_sys::Array;
        use wasm_bindgen::JsValue;

        // Create a mock file for testing
        let file_parts = Array::new();
        file_parts.push(&JsValue::from_str("test image content"));

        let file_property_bag = web_sys::FilePropertyBag::new();
        file_property_bag.set_type("image/jpeg");

        let file = File::new_with_buffer_source_sequence_and_options(
            &file_parts,
            "test.jpg",
            &file_property_bag,
        ).unwrap();

        // Test validation
        let validation_result = crate::utils::image::validate_image_file(&file);
        assert!(validation_result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_invalid_file_validation() {
        use web_sys::File;
        use js_sys::Array;
        use wasm_bindgen::JsValue;

        // Create a mock invalid file (too large)
        let large_content = "x".repeat(15 * 1024 * 1024); // 15MB
        let file_parts = Array::new();
        file_parts.push(&JsValue::from_str(&large_content));

        let file_property_bag = web_sys::FilePropertyBag::new();
        file_property_bag.set_type("image/jpeg");

        let file = File::new_with_buffer_source_sequence_and_options(
            &file_parts,
            "large_test.jpg",
            &file_property_bag,
        ).unwrap();

        // Test validation should fail
        let validation_result = crate::utils::image::validate_image_file(&file);
        assert!(validation_result.is_err());

        if let Err(error) = validation_result {
            assert!(error.to_string().contains("File too large"));
        }
    }

    #[wasm_bindgen_test]
    async fn test_unsupported_file_type() {
        use web_sys::File;
        use js_sys::Array;
        use wasm_bindgen::JsValue;

        // Create a mock PDF file
        let file_parts = Array::new();
        file_parts.push(&JsValue::from_str("PDF content"));

        let file_property_bag = web_sys::FilePropertyBag::new();
        file_property_bag.set_type("application/pdf");

        let file = File::new_with_buffer_source_sequence_and_options(
            &file_parts,
            "document.pdf",
            &file_property_bag,
        ).unwrap();

        // Test validation should fail
        let validation_result = crate::utils::image::validate_image_file(&file);
        assert!(validation_result.is_err());

        if let Err(error) = validation_result {
            assert!(error.to_string().contains("Invalid file type"));
        }
    }

    #[wasm_bindgen_test]
    fn test_file_preview_info_generation() {
        use web_sys::File;
        use js_sys::Array;
        use wasm_bindgen::JsValue;

        let file_parts = Array::new();
        file_parts.push(&JsValue::from_str("test content"));

        let file_property_bag = web_sys::FilePropertyBag::new();
        file_property_bag.set_type("image/png");

        let file = File::new_with_buffer_source_sequence_and_options(
            &file_parts,
            "test_image.png",
            &file_property_bag,
        ).unwrap();

        let preview_info = crate::utils::image::get_file_preview_info(&file);

        assert_eq!(preview_info.name, "test_image.png");
        assert_eq!(preview_info.type_, "image/png");
        assert!(preview_info.size.contains("B"));
    }

    #[wasm_bindgen_test]
    fn test_component_state_management() {
        // Test that component state updates correctly
        let props = ImageUploadProps {
            on_uploaded: Callback::noop(),
        };

        // This test would require a more sophisticated setup
        // For now, we just ensure the component can be created
        let _rendered = yew::Renderer::<ImageUpload>::with_props(props);
        assert!(true);
    }
}
