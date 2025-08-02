# Frontend Testing Strategy

This document outlines the comprehensive testing strategy for the AI4Thai Crop Guardian frontend, ensuring high-quality, reliable, and maintainable code.

## 🎯 Testing Objectives

### Quality Goals
- **Reliability**: Ensure components work correctly under all conditions
- **Maintainability**: Tests should be easy to understand and update
- **Performance**: Validate performance requirements are met
- **Accessibility**: Ensure WCAG 2.1 compliance
- **User Experience**: Validate user workflows function correctly

### Coverage Requirements
- **Overall Coverage**: 85% minimum
- **Components**: 90% minimum
- **Design System**: 95% minimum
- **Critical Paths**: 100% required
- **Utilities**: 95% minimum

## 🏗️ Testing Pyramid

### Distribution Strategy
- **70% Unit Tests**: Component logic, utilities, and isolated functionality
- **20% Integration Tests**: Component interactions and API integration
- **10% End-to-End Tests**: Complete user workflows and critical paths

### Rationale
- **Fast Feedback**: Unit tests provide immediate feedback during development
- **Cost Effective**: Unit tests are cheaper to write and maintain
- **Confidence**: Integration and E2E tests provide confidence in user workflows
- **Debugging**: Smaller tests make it easier to identify issues

## 🧪 Testing Framework Stack

### Core Testing Tools
```toml
# Cargo.toml testing dependencies
[dev-dependencies]
wasm-bindgen-test = "0.3"
web-sys = "0.3"
js-sys = "0.3"
wasm-bindgen-futures = "0.4"
gloo-utils = "0.1"
```

### Testing Environment
- **Test Runner**: `wasm-pack test`
- **Browser Testing**: Chrome, Firefox, Safari
- **Headless Testing**: For CI/CD pipeline
- **Mock Framework**: Custom mocks for API calls

## 📊 Unit Testing Strategy

### Component Testing
```rust
// Example component test
use wasm_bindgen_test::*;
use yew::prelude::*;
use crate::components::ui::Button;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_button_renders_correctly() {
    let div = gloo_utils::document().create_element("div").unwrap();
    
    let props = ButtonProps {
        children: html! { "Click me" }.into(),
        variant: ButtonVariant::Primary,
        size: ButtonSize::Medium,
        disabled: false,
        loading: false,
        onclick: Callback::noop(),
        class: Classes::new(),
    };
    
    yew::Renderer::<Button>::with_root_and_props(div.clone(), props)
        .render();
    
    assert!(div.inner_html().contains("Click me"));
    assert!(div.inner_html().contains("btn--primary"));
    assert!(div.inner_html().contains("btn--md"));
}

#[wasm_bindgen_test]
fn test_button_loading_state() {
    let div = gloo_utils::document().create_element("div").unwrap();
    
    let props = ButtonProps {
        children: html! { "Loading..." }.into(),
        variant: ButtonVariant::Primary,
        size: ButtonSize::Medium,
        disabled: false,
        loading: true,
        onclick: Callback::noop(),
        class: Classes::new(),
    };
    
    yew::Renderer::<Button>::with_root_and_props(div.clone(), props)
        .render();
    
    assert!(div.inner_html().contains("btn--loading"));
    assert!(div.inner_html().contains("btn__spinner"));
}
```

### Design System Testing
```rust
// Test design system components
use crate::components::layout::{BentoGrid, BentoCard, BentoSize};

#[wasm_bindgen_test]
fn test_bento_grid_layout() {
    let div = gloo_utils::document().create_element("div").unwrap();
    
    let grid = html! {
        <BentoGrid>
            <BentoCard size={BentoSize::Small}>
                { "Small card" }
            </BentoCard>
            <BentoCard size={BentoSize::Large}>
                { "Large card" }
            </BentoCard>
        </BentoGrid>
    };
    
    yew::Renderer::<BentoGrid>::with_root(div.clone())
        .render();
    
    assert!(div.inner_html().contains("bento-grid"));
    assert!(div.inner_html().contains("bento-card--small"));
    assert!(div.inner_html().contains("bento-card--large"));
}

#[wasm_bindgen_test]
fn test_dopamine_colors_applied() {
    use crate::utils::design_tokens::DopamineColors;
    
    let colors = DopamineColors::new();
    
    assert_eq!(colors.primary_electric_blue, "#0066FF");
    assert_eq!(colors.primary_vibrant_orange, "#FF6B35");
    assert_eq!(colors.primary_energetic_pink, "#FF1B8D");
}
```

### Utility Function Testing
```rust
// Test utility functions
use crate::utils::responsive::get_breakpoint;
use crate::utils::thai_text::format_thai_text;

#[wasm_bindgen_test]
fn test_responsive_breakpoints() {
    assert_eq!(get_breakpoint(320), "xs");
    assert_eq!(get_breakpoint(768), "md");
    assert_eq!(get_breakpoint(1200), "xl");
}

#[wasm_bindgen_test]
fn test_thai_text_formatting() {
    let thai_text = "สวัสดีครับ";
    let formatted = format_thai_text(thai_text);
    
    assert!(formatted.contains("text--thai"));
    assert!(formatted.len() > thai_text.len());
}
```

## 🔗 Integration Testing

### Component Interaction Testing
```rust
// Test component interactions
use crate::components::chat::{ChatWindow, MessageInput};
use crate::services::api_client::MockApiClient;

#[wasm_bindgen_test]
async fn test_chat_message_flow() {
    let div = gloo_utils::document().create_element("div").unwrap();
    
    // Setup mock API client
    let mut mock_api = MockApiClient::new();
    mock_api.expect_send_message()
        .returning(|_| Ok(ChatResponse {
            message_id: "test_id".to_string(),
            content: "Test response".to_string(),
        }));
    
    let chat_window = html! {
        <ChatWindow api_client={mock_api} />
    };
    
    yew::Renderer::<ChatWindow>::with_root(div.clone())
        .render();
    
    // Simulate user input
    let input_element = div.query_selector("input[type='text']").unwrap().unwrap();
    input_element.set_attribute("value", "Hello").unwrap();
    
    // Simulate form submission
    let form = div.query_selector("form").unwrap().unwrap();
    let event = web_sys::Event::new("submit").unwrap();
    form.dispatch_event(&event).unwrap();
    
    // Wait for async operations
    gloo_timers::future::TimeoutFuture::new(100).await;
    
    // Verify message was sent and response received
    assert!(div.inner_html().contains("Hello"));
    assert!(div.inner_html().contains("Test response"));
}
```

### API Integration Testing
```rust
// Test API service integration
use crate::services::diagnosis_service::DiagnosisService;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen_test]
async fn test_diagnosis_api_integration() {
    let service = DiagnosisService::new("http://localhost:3000".to_string());
    
    // Create mock image data
    let image_data = vec![0u8; 1024]; // Mock image bytes
    
    // Test successful diagnosis
    match service.diagnose_image(image_data, "rice".to_string()).await {
        Ok(result) => {
            assert!(!result.diseases.is_empty());
            assert!(result.confidence_score > 0.0);
        }
        Err(e) => panic!("Diagnosis failed: {:?}", e),
    }
}
```

## 🎭 End-to-End Testing

### User Workflow Testing
```rust
// E2E test for complete user journey
use wasm_bindgen_test::*;
use web_sys::*;

#[wasm_bindgen_test]
async fn test_complete_diagnosis_workflow() {
    // Navigate to application
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // Wait for app to load
    gloo_timers::future::TimeoutFuture::new(1000).await;
    
    // Step 1: User uploads image
    let file_input = document
        .query_selector("input[type='file']")
        .unwrap()
        .unwrap();
    
    // Simulate file selection (mock)
    let event = Event::new("change").unwrap();
    file_input.dispatch_event(&event).unwrap();
    
    // Step 2: Select crop type
    let crop_select = document
        .query_selector("select[name='crop_type']")
        .unwrap()
        .unwrap();
    
    crop_select.set_attribute("value", "rice").unwrap();
    
    // Step 3: Submit for diagnosis
    let submit_button = document
        .query_selector("button[type='submit']")
        .unwrap()
        .unwrap();
    
    submit_button.click();
    
    // Step 4: Wait for results
    gloo_timers::future::TimeoutFuture::new(3000).await;
    
    // Step 5: Verify results displayed
    let results_section = document
        .query_selector(".diagnosis-results")
        .unwrap();
    
    assert!(results_section.is_some());
    
    let results = results_section.unwrap();
    assert!(results.inner_html().contains("confidence"));
    assert!(results.inner_html().contains("treatment"));
}
```

### Performance Testing
```rust
// Performance testing for critical components
use web_sys::Performance;

#[wasm_bindgen_test]
async fn test_component_render_performance() {
    let window = web_sys::window().unwrap();
    let performance = window.performance().unwrap();
    
    let start_time = performance.now();
    
    // Render large component tree
    let div = gloo_utils::document().create_element("div").unwrap();
    
    let large_grid = html! {
        <BentoGrid>
            { for (0..100).map(|i| html! {
                <BentoCard key={i}>
                    { format!("Card {}", i) }
                </BentoCard>
            })}
        </BentoGrid>
    };
    
    yew::Renderer::<BentoGrid>::with_root(div.clone())
        .render();
    
    let end_time = performance.now();
    let render_time = end_time - start_time;
    
    // Assert render time is under 100ms
    assert!(render_time < 100.0, "Render time {} exceeds 100ms", render_time);
}
```

## 🌐 Accessibility Testing

### WCAG Compliance Testing
```rust
// Accessibility testing utilities
use crate::utils::accessibility::{check_color_contrast, validate_aria_labels};

#[wasm_bindgen_test]
fn test_color_contrast_compliance() {
    use crate::utils::design_tokens::DopamineColors;
    
    let colors = DopamineColors::new();
    
    // Test primary color contrast against white background
    let contrast_ratio = check_color_contrast(
        colors.primary_electric_blue,
        colors.neutral_white
    );
    
    // WCAG AA requires 4.5:1 contrast ratio
    assert!(contrast_ratio >= 4.5, "Contrast ratio {} is below WCAG AA standard", contrast_ratio);
}

#[wasm_bindgen_test]
fn test_aria_labels_present() {
    let div = gloo_utils::document().create_element("div").unwrap();
    
    let button = html! {
        <Button aria_label="Submit diagnosis">
            { "Submit" }
        </Button>
    };
    
    yew::Renderer::<Button>::with_root(div.clone())
        .render();
    
    let button_element = div.query_selector("button").unwrap().unwrap();
    let aria_label = button_element.get_attribute("aria-label").unwrap();
    
    assert_eq!(aria_label, "Submit diagnosis");
}
```

### Keyboard Navigation Testing
```rust
#[wasm_bindgen_test]
async fn test_keyboard_navigation() {
    let div = gloo_utils::document().create_element("div").unwrap();
    
    let form = html! {
        <form>
            <input type="text" placeholder="Enter message" />
            <Button>{ "Send" }</Button>
        </form>
    };
    
    yew::Renderer::<form>::with_root(div.clone())
        .render();
    
    // Test Tab navigation
    let input = div.query_selector("input").unwrap().unwrap();
    let button = div.query_selector("button").unwrap().unwrap();
    
    // Simulate Tab key press
    let tab_event = KeyboardEvent::new_with_keyboard_event_init(
        "keydown",
        KeyboardEventInit::new().key_code(9)
    ).unwrap();
    
    input.dispatch_event(&tab_event.into()).unwrap();
    
    // Verify focus moved to button
    assert_eq!(
        button.as_ref(),
        &web_sys::window().unwrap().document().unwrap().active_element().unwrap()
    );
}
```

## 📱 Mobile and Responsive Testing

### Viewport Testing
```rust
#[wasm_bindgen_test]
async fn test_responsive_breakpoints() {
    let window = web_sys::window().unwrap();
    
    // Test mobile viewport
    window.resize_to(375, 667).unwrap();
    gloo_timers::future::TimeoutFuture::new(100).await;
    
    let div = gloo_utils::document().create_element("div").unwrap();
    
    let grid = html! {
        <BentoGrid>
            <BentoCard size={BentoSize::Large}>
                { "Large card" }
            </BentoCard>
        </BentoGrid>
    };
    
    yew::Renderer::<BentoGrid>::with_root(div.clone())
        .render();
    
    // On mobile, large cards should span full width
    let card = div.query_selector(".bento-card--large").unwrap().unwrap();
    let computed_style = window.get_computed_style(&card).unwrap().unwrap();
    let grid_column = computed_style.get_property_value("grid-column").unwrap();
    
    assert_eq!(grid_column, "span 1");
    
    // Test desktop viewport
    window.resize_to(1200, 800).unwrap();
    gloo_timers::future::TimeoutFuture::new(100).await;
    
    let computed_style = window.get_computed_style(&card).unwrap().unwrap();
    let grid_column = computed_style.get_property_value("grid-column").unwrap();
    
    assert_eq!(grid_column, "span 2");
}
```

## 🔧 Test Utilities and Helpers

### Mock Services
```rust
// Mock API client for testing
pub struct MockApiClient {
    pub responses: HashMap<String, serde_json::Value>,
}

impl MockApiClient {
    pub fn new() -> Self {
        Self {
            responses: HashMap::new(),
        }
    }
    
    pub fn set_response(&mut self, endpoint: &str, response: serde_json::Value) {
        self.responses.insert(endpoint.to_string(), response);
    }
}

impl ApiClient for MockApiClient {
    async fn post(&self, endpoint: &str, _body: serde_json::Value) -> Result<serde_json::Value, ApiError> {
        self.responses
            .get(endpoint)
            .cloned()
            .ok_or(ApiError::NotFound)
    }
}
```

### Test Data Factories
```rust
// Factory functions for test data
pub struct TestDataFactory;

impl TestDataFactory {
    pub fn create_diagnosis_result() -> DiagnosisResult {
        DiagnosisResult {
            diagnosis_id: "test_123".to_string(),
            crop_type: "rice".to_string(),
            diseases: vec![
                Disease {
                    name: "Rice Blast".to_string(),
                    confidence: 0.95,
                    severity: "moderate".to_string(),
                }
            ],
            confidence_score: 0.95,
            processing_time_ms: 1500,
        }
    }
    
    pub fn create_chat_message() -> ChatMessage {
        ChatMessage {
            message_id: "msg_123".to_string(),
            content: "How do I treat rice blast?".to_string(),
            sender: "user".to_string(),
            timestamp: js_sys::Date::now(),
        }
    }
}
```

## 📊 Test Execution and Reporting

### Running Tests
```bash
# Run all tests
wasm-pack test --headless --firefox

# Run tests in browser for debugging
wasm-pack test --firefox

# Run specific test file
wasm-pack test --headless --firefox -- --test component_tests

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/
```

### Continuous Integration
```yaml
# GitHub Actions workflow for testing
name: Frontend Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
          
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
        
      - name: Run tests
        run: |
          cd frontend
          wasm-pack test --headless --firefox
          
      - name: Generate coverage
        run: |
          cd frontend
          cargo tarpaulin --out Xml
          
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

### Test Metrics and Monitoring
- **Coverage Reports**: Generated after each test run
- **Performance Benchmarks**: Track component render times
- **Accessibility Scores**: WCAG compliance metrics
- **Bundle Size Impact**: Monitor test impact on bundle size

This comprehensive testing strategy ensures the AI4Thai Crop Guardian frontend maintains high quality, performance, and accessibility standards while providing confidence in the user experience.
