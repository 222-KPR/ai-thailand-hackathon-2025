use yew::prelude::*;
use crate::types::Language;

use crate::app::AppState;
use crate::i18n::I18nContext;

#[function_component(Header)]
pub fn header() -> Html {
    let state = use_context::<UseStateHandle<AppState>>().expect("AppState not found");
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");

    let toggle_language = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            let new_lang = match state.language {
                Language::Thai => Language::English,
                Language::English => Language::Thai,
            };
            let mut new_state = (*state).clone();
            new_state.language = new_lang;
            state.set(new_state);
        })
    };

    let start_new_conversation = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut new_state = (*state).clone();
            new_state.conversation_id = uuid::Uuid::new_v4();
            new_state.messages.clear();
            new_state.error_message = None;
            state.set(new_state);
        })
    };

    html! {
        <header class="header">
            <div class="header-content">
                <div class="header-left">
                    <div class="logo">
                        <span class="logo-icon">{"🌾"}</span>
                        <div class="logo-text">
                            <h1 class="logo-title">{i18n.t("app.title")}</h1>
                            <p class="logo-subtitle">{i18n.t("app.subtitle")}</p>
                        </div>
                    </div>
                </div>

                <div class="header-center">
                    <div class="connection-status status-connected">
                        <span class="status-indicator"></span>
                        <span class="status-text">{i18n.t("status.connected")}</span>
                    </div>
                </div>

                <div class="header-right">
                    <button
                        class="btn btn-secondary btn-sm"
                        onclick={start_new_conversation}
                        title={i18n.t("chat.clear")}
                    >
                        <span class="icon">{"🔄"}</span>
                        <span class="btn-text">{i18n.t("chat.clear")}</span>
                    </button>

                    <button
                        class="btn btn-outline btn-sm language-toggle"
                        onclick={toggle_language}
                        title={i18n.t("language.switch")}
                    >
                        <span class="icon">{"🌐"}</span>
                        <span class="btn-text">
                            {match state.language {
                                Language::Thai => i18n.t("language.english"),
                                Language::English => i18n.t("language.thai"),
                            }}
                        </span>
                    </button>
                </div>
            </div>
        </header>
    }
}
