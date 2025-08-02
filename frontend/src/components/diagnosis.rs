//! Diagnosis Component
//!
//! This module provides components for displaying disease diagnosis results
//! and treatment recommendations.

use yew::prelude::*;
use crate::types::{VisionResponse, LLMResponse, DiseaseSeverity};
use crate::i18n::I18nContext;
use crate::components::ui::{GradientButton, StatusCard};
use crate::styles::colors::*;

/// Properties for the diagnosis result component
#[derive(Properties, PartialEq)]
pub struct DiagnosisResultProps {
    /// Vision service response with disease detection
    pub vision_result: VisionResponse,

    /// LLM service response with treatment advice (optional)
    pub treatment_advice: Option<LLMResponse>,

    /// Whether treatment advice is being loaded
    #[prop_or(false)]
    pub loading_advice: bool,

    /// Callback when user requests more details
    #[prop_or_default]
    pub on_details_request: Callback<()>,
}

/// Component for displaying diagnosis results
#[function_component(DiagnosisResult)]
pub fn diagnosis_result(props: &DiagnosisResultProps) -> Html {
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");

    let severity_color = match props.vision_result.severity {
        DiseaseSeverity::Low => SemanticColors::SUCCESS,
        DiseaseSeverity::Medium => SemanticColors::WARNING,
        DiseaseSeverity::High => SemanticColors::ERROR,
        DiseaseSeverity::Critical => "#DC2626", // Critical red
    };

    let severity_text = match props.vision_result.severity {
        DiseaseSeverity::Low => i18n.t("severity.low"),
        DiseaseSeverity::Medium => i18n.t("severity.medium"),
        DiseaseSeverity::High => i18n.t("severity.high"),
        DiseaseSeverity::Critical => i18n.t("severity.critical"),
    };

    let confidence_percentage = (props.vision_result.confidence * 100.0) as u32;

    html! {
        <div class="diagnosis-result">
            <div class="diagnosis-header">
                <h2 class="diagnosis-title">
                    { i18n.t("diagnosis.title") }
                </h2>
                <div class="diagnosis-meta">
                    <span class="processing-time">
                        { format!("{}ms", props.vision_result.processing_time_ms) }
                    </span>
                </div>
            </div>

            <div class="diagnosis-content">
                <StatusCard
                    title={i18n.tf("diagnosis.disease", &[("disease", &props.vision_result.disease)])}
                    value={format!("{}%", confidence_percentage)}
                    color={PrimaryColors::ELECTRIC_BLUE.to_string()}
                    subtitle={Some(i18n.tf("diagnosis.confidence", &[("confidence", &confidence_percentage.to_string())]))}
                />

                <div class="severity-indicator" style={format!("border-left: 4px solid {}", severity_color)}>
                    <span class="severity-label">
                        { i18n.t("diagnosis.severity") }
                    </span>
                    <span class="severity-value" style={format!("color: {}", severity_color)}>
                        { severity_text }
                    </span>
                </div>

                if !props.vision_result.affected_areas.is_empty() {
                    <div class="affected-areas">
                        <h3>{ "Affected Areas" }</h3>
                        <div class="bounding-boxes">
                            { for props.vision_result.affected_areas.iter().enumerate().map(|(i, bbox)| {
                                html! {
                                    <div key={i} class="bounding-box-info">
                                        <span>{ format!("Area {}: {:.1}% confidence", i + 1, bbox.confidence * 100.0) }</span>
                                    </div>
                                }
                            })}
                        </div>
                    </div>
                }
            </div>

            <div class="treatment-section">
                if props.loading_advice {
                    <div class="loading-advice">
                        <div class="loading-spinner"></div>
                        <span>{ i18n.t("diagnosis.getting_advice") }</span>
                    </div>
                } else if let Some(advice) = &props.treatment_advice {
                    <TreatmentAdvice advice={advice.clone()} />
                } else {
                    <GradientButton
                        onclick={props.on_details_request.clone()}
                        variant={crate::components::ui::ButtonVariant::Secondary}
                    >
                        { "Get Treatment Advice" }
                    </GradientButton>
                }
            </div>
        </div>
    }
}

/// Properties for treatment advice component
#[derive(Properties, PartialEq)]
pub struct TreatmentAdviceProps {
    /// LLM response with treatment recommendations
    pub advice: LLMResponse,
}

/// Component for displaying treatment advice
#[function_component(TreatmentAdvice)]
pub fn treatment_advice(props: &TreatmentAdviceProps) -> Html {
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");

    html! {
        <div class="treatment-advice">
            <h3 class="treatment-title">
                { i18n.t("treatment.title") }
            </h3>

            <div class="advice-content">
                <p class="advice-text">
                    { &props.advice.advice }
                </p>

                if !props.advice.recommended_actions.is_empty() {
                    <div class="recommended-actions">
                        <h4>{ i18n.t("treatment.steps") }</h4>
                        <div class="actions-list">
                            { for props.advice.recommended_actions.iter().enumerate().map(|(i, action)| {
                                let priority_color = match action.priority {
                                    crate::types::Priority::Low => SemanticColors::SUCCESS,
                                    crate::types::Priority::Medium => SemanticColors::WARNING,
                                    crate::types::Priority::High => SemanticColors::ERROR,
                                    crate::types::Priority::Urgent => "#DC2626",
                                };

                                html! {
                                    <div key={i} class="action-item">
                                        <div class="action-header">
                                            <span class="action-number">{ i + 1 }</span>
                                            <span class="action-type">
                                                { format!("{:?}", action.action_type) }
                                            </span>
                                            <span
                                                class="action-priority"
                                                style={format!("color: {}", priority_color)}
                                            >
                                                { format!("{:?}", action.priority) }
                                            </span>
                                        </div>
                                        <p class="action-description">
                                            { &action.description }
                                        </p>
                                        <div class="action-details">
                                            <span class="action-timeline">
                                                { &action.timeline }
                                            </span>
                                            if let Some(cost) = action.estimated_cost {
                                                <span class="action-cost">
                                                    { format!("~{:.0} THB", cost) }
                                                </span>
                                            }
                                        </div>
                                        if !action.materials_needed.is_empty() {
                                            <div class="materials-needed">
                                                <strong>{ i18n.t("treatment.materials") }</strong>
                                                <ul>
                                                    { for action.materials_needed.iter().map(|material| {
                                                        html! { <li>{ material }</li> }
                                                    })}
                                                </ul>
                                            </div>
                                        }
                                    </div>
                                }
                            })}
                        </div>
                    </div>
                }

                if !props.advice.sources.is_empty() {
                    <div class="advice-sources">
                        <h4>{ "Sources:" }</h4>
                        <ul>
                            { for props.advice.sources.iter().map(|source| {
                                html! { <li>{ source }</li> }
                            })}
                        </ul>
                    </div>
                }

                <div class="advice-meta">
                    <span class="confidence">
                        { format!("Confidence: {:.0}%", props.advice.confidence * 100.0) }
                    </span>
                    <span class="processing-time">
                        { format!("Generated in {}ms", props.advice.processing_time_ms) }
                    </span>
                </div>
            </div>
        </div>
    }
}

/// Compact diagnosis summary component
#[derive(Properties, PartialEq)]
pub struct DiagnosisSummaryProps {
    pub vision_result: VisionResponse,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(DiagnosisSummary)]
pub fn diagnosis_summary(props: &DiagnosisSummaryProps) -> Html {
    let confidence_percentage = (props.vision_result.confidence * 100.0) as u32;

    html! {
        <div class="diagnosis-summary" onclick={props.onclick.clone()}>
            <div class="summary-disease">
                { &props.vision_result.disease }
            </div>
            <div class="summary-confidence">
                { format!("{}%", confidence_percentage) }
            </div>
            <div class="summary-severity">
                { format!("{:?}", props.vision_result.severity) }
            </div>
        </div>
    }
}
