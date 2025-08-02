use yew::prelude::*;

use crate::i18n::I18nContext;

#[function_component(WelcomeMessage)]
pub fn welcome_message() -> Html {
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");
    
    html! {
        <div class="welcome-message">
            <div class="welcome-hero">
                <div class="welcome-icon">{"🌾🤖"}</div>
                <h2 class="welcome-title">{i18n.t("welcome.title")}</h2>
                <p class="welcome-subtitle">{i18n.t("welcome.subtitle")}</p>
            </div>
            
            <div class="welcome-content">
                <div class="how-to-use">
                    <h3 class="section-title">{i18n.t("welcome.how_to_use")}</h3>
                    <div class="steps">
                        <div class="step">
                            <div class="step-icon">{"📷"}</div>
                            <p class="step-text">{i18n.t("welcome.step1")}</p>
                        </div>
                        <div class="step">
                            <div class="step-icon">{"🌿"}</div>
                            <p class="step-text">{i18n.t("welcome.step2")}</p>
                        </div>
                        <div class="step">
                            <div class="step-icon">{"🔬"}</div>
                            <p class="step-text">{i18n.t("welcome.step3")}</p>
                        </div>
                        <div class="step">
                            <div class="step-icon">{"💬"}</div>
                            <p class="step-text">{i18n.t("welcome.step4")}</p>
                        </div>
                    </div>
                </div>
                
                <div class="supported-crops">
                    <h3 class="section-title">{"พืชที่รองรับ / Supported Crops"}</h3>
                    <div class="crops-grid">
                        <div class="crop-item">
                            <span class="crop-icon">{"🌾"}</span>
                            <span class="crop-name">{i18n.t("image.crop.rice")}</span>
                        </div>
                        <div class="crop-item">
                            <span class="crop-icon">{"🥔"}</span>
                            <span class="crop-name">{i18n.t("image.crop.cassava")}</span>
                        </div>
                        <div class="crop-item">
                            <span class="crop-icon">{"🥭"}</span>
                            <span class="crop-name">{i18n.t("image.crop.durian")}</span>
                        </div>
                        <div class="crop-item">
                            <span class="crop-icon">{"🥭"}</span>
                            <span class="crop-name">{i18n.t("image.crop.mango")}</span>
                        </div>
                        <div class="crop-item">
                            <span class="crop-icon">{"🌳"}</span>
                            <span class="crop-name">{i18n.t("image.crop.rubber")}</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}