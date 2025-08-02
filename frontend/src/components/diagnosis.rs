use yew::prelude::*;
use shared::{VisionResponse, LLMResponse, DiseaseSeverity, TreatmentUrgency};

use crate::i18n::I18nContext;

#[derive(Properties, PartialEq)]
pub struct DiagnosisResultProps {
    pub vision_result: VisionResponse,
    pub llm_result: Option<LLMResponse>,
}

#[function_component(DiagnosisResult)]
pub fn diagnosis_result(props: &DiagnosisResultProps) -> Html {
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");
    
    let severity_class = match props.vision_result.severity {
        DiseaseSeverity::Low => "severity-low",
        DiseaseSeverity::Medium => "severity-medium", 
        DiseaseSeverity::High => "severity-high",
        DiseaseSeverity::Critical => "severity-critical",
    };
    
    let confidence_class = if props.vision_result.confidence > 0.8 {
        "confidence-high"
    } else if props.vision_result.confidence > 0.6 {
        "confidence-medium"
    } else {
        "confidence-low"
    };
    
    html! {
        <div class="diagnosis-result">
            <div class="diagnosis-header">
                <div class="diagnosis-icon">{"🔬"}</div>
                <h3 class="diagnosis-title">{i18n.t("diagnosis.title")}</h3>
            </div>
            
            <div class="diagnosis-content">
                <div class="diagnosis-main">
                    <div class="disease-info">
                        <div class="info-item">
                            <span class="info-label">{"โรคที่พบ:"}</span>
                            <span class="info-value disease-name">{&props.vision_result.disease}</span>
                        </div>
                        
                        <div class="info-item">
                            <span class="info-label">{"ความเชื่อมั่น:"}</span>
                            <span class={classes!("info-value", "confidence", confidence_class)}>
                                {format!("{:.1}%", props.vision_result.confidence * 100.0)}
                            </span>
                        </div>
                        
                        <div class="info-item">
                            <span class="info-label">{"ความรุนแรง:"}</span>
                            <span class={classes!("info-value", "severity", severity_class)}>
                                {severity_text(&props.vision_result.severity)}
                            </span>
                        </div>
                    </div>
                    
                    {if !props.vision_result.affected_areas.is_empty() {
                        html! {
                            <div class="affected-areas">
                                <h4 class="section-title">{"บริเวณที่ได้รับผลกระทบ:"}</h4>
                                <div class="areas-list">
                                    {props.vision_result.affected_areas.iter().enumerate().map(|(i, area)| {
                                        html! {
                                            <div class="area-item" key={i}>
                                                <span class="area-confidence">{format!("{:.1}%", area.confidence * 100.0)}</span>
                                                <span class="area-location">
                                                    {format!("x:{:.0}, y:{:.0}, ขนาด:{:.0}x{:.0}", 
                                                        area.x, area.y, area.width, area.height)}
                                                </span>
                                            </div>
                                        }
                                    }).collect::<Html>()}
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>
                
                {if let Some(llm_result) = &props.llm_result {
                    html! { <TreatmentAdvice llm_result={llm_result.clone()} /> }
                } else {
                    html! {
                        <div class="loading-treatment">
                            <div class="loading-spinner"></div>
                            <p class="loading-text">{i18n.t("diagnosis.getting_advice")}</p>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TreatmentAdviceProps {
    pub llm_result: LLMResponse,
}

#[function_component(TreatmentAdvice)]
pub fn treatment_advice(props: &TreatmentAdviceProps) -> Html {
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");
    
    let urgency_class = match props.llm_result.treatment_plan.urgency {
        TreatmentUrgency::Low => "urgency-low",
        TreatmentUrgency::Medium => "urgency-medium",
        TreatmentUrgency::High => "urgency-high",
        TreatmentUrgency::Critical => "urgency-critical",
    };
    
    html! {
        <div class="treatment-advice">
            <div class="treatment-header">
                <div class="treatment-icon">{"💊"}</div>
                <h3 class="treatment-title">{i18n.t("treatment.title")}</h3>
            </div>
            
            <div class="treatment-content">
                {if !props.llm_result.advice.is_empty() {
                    html! {
                        <div class="advice-summary">
                            <h4 class="section-title">{"คำแนะนำทั่วไป:"}</h4>
                            <p class="advice-text">{&props.llm_result.advice}</p>
                        </div>
                    }
                } else {
                    html! {}
                }}
                
                <div class="treatment-plan">
                    <div class="plan-header">
                        <h4 class="section-title">{i18n.t("treatment.steps")}</h4>
                        <div class="plan-meta">
                            <span class="timeline">
                                {"ระยะเวลา: "}{props.llm_result.treatment_plan.timeline_days}{" วัน"}
                            </span>
                            <span class={classes!("urgency", urgency_class)}>
                                {urgency_text(&props.llm_result.treatment_plan.urgency)}
                            </span>
                        </div>
                    </div>
                    
                    <div class="treatment-steps">
                        {props.llm_result.treatment_plan.steps.iter().map(|step| {
                            html! {
                                <div class="treatment-step" key={step.step_number}>
                                    <div class="step-number">{step.step_number}</div>
                                    <div class="step-content">
                                        <p class="step-description">{&step.description}</p>
                                        
                                        {if !step.materials_needed.is_empty() {
                                            html! {
                                                <div class="step-materials">
                                                    <h5 class="materials-title">{"วัสดุที่ต้องใช้:"}</h5>
                                                    <ul class="materials-list">
                                                        {step.materials_needed.iter().map(|material| {
                                                            html! {
                                                                <li class="material-item" key={&material.name}>
                                                                    <span class="material-name">{&material.name}</span>
                                                                    <span class="material-quantity">{&material.quantity}</span>
                                                                    {if let Some(cost) = material.estimated_cost_baht {
                                                                        html! {
                                                                            <span class="material-cost">
                                                                                {format!("~{:.0} บาท", cost)}
                                                                            </span>
                                                                        }
                                                                    } else {
                                                                        html! {}
                                                                    }}
                                                                </li>
                                                            }
                                                        }).collect::<Html>()}
                                                    </ul>
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }}
                                        
                                        {if !step.warnings.is_empty() {
                                            html! {
                                                <div class="step-warnings">
                                                    <h5 class="warnings-title">{"คำเตือน:"}</h5>
                                                    <ul class="warnings-list">
                                                        {step.warnings.iter().map(|warning| {
                                                            html! {
                                                                <li class="warning-item" key={warning}>
                                                                    <span class="warning-icon">{"⚠️"}</span>
                                                                    <span class="warning-text">{warning}</span>
                                                                </li>
                                                            }
                                                        }).collect::<Html>()}
                                                    </ul>
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }}
                                        
                                        <div class="step-timing">
                                            <span class="timing-label">{"เวลา: "}</span>
                                            <span class="timing-value">{&step.timing}</span>
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
                
                {if let Some(cost_estimate) = &props.llm_result.estimated_cost {
                    html! {
                        <div class="cost-estimate">
                            <h4 class="section-title">{"ค่าใช้จ่ายประมาณ:"}</h4>
                            <div class="cost-summary">
                                <span class="cost-range">
                                    {format!("{:.0} - {:.0} บาท", cost_estimate.min_baht, cost_estimate.max_baht)}
                                </span>
                            </div>
                            
                            {if !cost_estimate.breakdown.is_empty() {
                                html! {
                                    <div class="cost-breakdown">
                                        <h5 class="breakdown-title">{"รายละเอียด:"}</h5>
                                        <ul class="breakdown-list">
                                            {cost_estimate.breakdown.iter().map(|item| {
                                                html! {
                                                    <li class="breakdown-item" key={&item.item}>
                                                        <span class="item-name">{&item.item}</span>
                                                        <span class="item-cost">{format!("{:.0} บาท", item.cost_baht)}</span>
                                                        {if item.is_optional {
                                                            html! { <span class="item-optional">{"(ไม่จำเป็น)"}</span> }
                                                        } else {
                                                            html! {}
                                                        }}
                                                    </li>
                                                }
                                            }).collect::<Html>()}
                                        </ul>
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                        </div>
                    }
                } else {
                    html! {}
                }}
                
                {if !props.llm_result.prevention_tips.is_empty() {
                    html! {
                        <div class="prevention-tips">
                            <h4 class="section-title">{i18n.t("treatment.prevention")}</h4>
                            <ul class="prevention-list">
                                {props.llm_result.prevention_tips.iter().map(|tip| {
                                    html! {
                                        <li class="prevention-item" key={tip}>
                                            <span class="prevention-icon">{"🛡️"}</span>
                                            <span class="prevention-text">{tip}</span>
                                        </li>
                                    }
                                }).collect::<Html>()}
                            </ul>
                        </div>
                    }
                } else {
                    html! {}
                }}
                
                {if let Some(organic) = &props.llm_result.treatment_plan.organic_alternative {
                    html! {
                        <div class="organic-alternative">
                            <h4 class="section-title">{i18n.t("treatment.organic")}</h4>
                            <div class="organic-content">
                                <p class="organic-method"><strong>{"วิธีการ: "}</strong>{&organic.method}</p>
                                
                                <div class="organic-ingredients">
                                    <h5>{"ส่วนผสม:"}</h5>
                                    <ul>
                                        {organic.ingredients.iter().map(|ingredient| {
                                            html! {
                                                <li key={ingredient}>{ingredient}</li>
                                            }
                                        }).collect::<Html>()}
                                    </ul>
                                </div>
                                
                                <p class="organic-preparation"><strong>{"การเตรียม: "}</strong>{&organic.preparation}</p>
                                
                                <div class="organic-effectiveness">
                                    <span class="effectiveness-label">{"ประสิทธิภาพ: "}</span>
                                    <span class="effectiveness-value">
                                        {format!("{:.0}% เทียบกับการรักษาด้วยสารเคมี", organic.effectiveness * 100.0)}
                                    </span>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }}
                
                {if !props.llm_result.sources.is_empty() {
                    html! {
                        <div class="sources">
                            <h4 class="section-title">{"แหล่งข้อมูล:"}</h4>
                            <ul class="sources-list">
                                {props.llm_result.sources.iter().map(|source| {
                                    html! {
                                        <li class="source-item" key={source}>{source}</li>
                                    }
                                }).collect::<Html>()}
                            </ul>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}

fn severity_text(severity: &DiseaseSeverity) -> &'static str {
    match severity {
        DiseaseSeverity::Low => "น้อย",
        DiseaseSeverity::Medium => "ปานกลาง",
        DiseaseSeverity::High => "สูง", 
        DiseaseSeverity::Critical => "วิกฤต",
    }
}

fn urgency_text(urgency: &TreatmentUrgency) -> &'static str {
    match urgency {
        TreatmentUrgency::Low => "ไม่เร่งด่วน",
        TreatmentUrgency::Medium => "ปานกลาง",
        TreatmentUrgency::High => "เร่งด่วน",
        TreatmentUrgency::Critical => "วิกฤต",
    }
}