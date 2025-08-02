// Copyright (c) 2025 AI4Thai Crop Guardian
// Licensed under the MIT License

//! Testing utilities for the frontend application

use yew::prelude::*;
use web_sys::window;

/// Simple test helper for component rendering
pub fn render_test_component() -> Html {
    html! {
        <div class="test-container">
            <p>{ "Test component placeholder" }</p>
        </div>
    }
}

/// Mock performance metrics for testing
#[derive(Debug, Clone, PartialEq)]
pub struct MockPerformanceMetrics {
    pub load_time: f64,
    pub first_paint: f64,
    pub first_contentful_paint: f64,
}

impl Default for MockPerformanceMetrics {
    fn default() -> Self {
        Self {
            load_time: 1000.0,
            first_paint: 500.0,
            first_contentful_paint: 800.0,
        }
    }
}

/// Simple feedback component for testing
#[derive(Properties, PartialEq)]
pub struct FeedbackProps {
    pub on_submit: Callback<String>,
}

#[function_component(FeedbackComponent)]
pub fn feedback_component(props: &FeedbackProps) -> Html {
    let feedback_text = use_state(String::new);
    let show_feedback = use_state(|| false);
    
    let toggle_feedback = {
        let show_feedback = show_feedback.clone();
        Callback::from(move |_| {
            show_feedback.set(!*show_feedback);
        })
    };
    
    let submit_feedback = {
        let feedback_text = feedback_text.clone();
        let on_submit = props.on_submit.clone();
        let show_feedback = show_feedback.clone();
        Callback::from(move |_| {
            on_submit.emit((*feedback_text).clone());
            show_feedback.set(false);
        })
    };
    
    html! {
        <div class="feedback-widget">
            <button class="feedback-trigger" onclick={toggle_feedback.clone()}>
                { "Feedback" }
            </button>
            
            if *show_feedback {
                <div class="feedback-modal">
                    <div class="feedback-content">
                        <h3>{ "Send Feedback" }</h3>
                        <textarea 
                            placeholder="Your feedback..."
                            value={(*feedback_text).clone()}
                            oninput={
                                let feedback_text = feedback_text.clone();
                                Callback::from(move |e: InputEvent| {
                                    if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                                        feedback_text.set(input.value());
                                    }
                                })
                            }
                        />
                        <div class="feedback-actions">
                            <button class="btn-submit" onclick={submit_feedback}>
                                { "Submit" }
                            </button>
                            <button class="btn-cancel" onclick={toggle_feedback}>
                                { "Cancel" }
                            </button>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}

/// Simple rating component for testing
#[derive(Properties, PartialEq)]
pub struct RatingProps {
    pub max_rating: u8,
    pub on_rate: Callback<u8>,
}

#[function_component(RatingComponent)]
pub fn rating_component(props: &RatingProps) -> Html {
    let rating = use_state(|| 0u8);
    
    html! {
        <div class="rating-widget">
            <div class="stars">
                { for (1..=props.max_rating).map(|i| {
                    let rating_clone = rating.clone();
                    let on_rate = props.on_rate.clone();
                    let star_click = Callback::from(move |_| {
                        rating_clone.set(i);
                        on_rate.emit(i);
                    });
                    
                    html! {
                        <button 
                            class={if i <= *rating { "star active" } else { "star" }}
                            onclick={star_click}
                        >
                            { "★" }
                        </button>
                    }
                })}
            </div>
            <p>{ format!("Rating: {}/{}", *rating, props.max_rating) }</p>
        </div>
    }
}

/// Get mock device info for testing
pub fn get_mock_device_info() -> String {
    if let Some(window) = window() {
        let navigator = window.navigator();
        return navigator.user_agent().unwrap_or_else(|_| "Unknown".to_string());
    }
    "Mock Device".to_string()
}

/// Mock performance measurement
pub fn measure_mock_performance() -> MockPerformanceMetrics {
    MockPerformanceMetrics::default()
}
