use yew::prelude::*;
use yewdux::prelude::*;
use shared::Language;

use crate::store::{AppState, AppAction};
use crate::i18n::I18nContext;

#[function_component(Header)]
pub fn header() -> Html {
    let (state, dispatch) = use_store::<AppState>();
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");
    
    let toggle_language = {
        let dispatch = dispatch.clone();
        Callback::from(move |_: MouseEvent| {
            let new_lang = match state.language {
                Language::Thai => Language::English,
                Language::English => Language::Thai,
            };
            dispatch.apply(AppAction::SetLanguage(new_lang));
        })
    };
    
    let start_new_conversation = {
        let dispatch = dispatch.clone();
        Callback::from(move |_: MouseEvent| {
            dispatch.apply(AppAction::StartNewConversation);
        })
    };
    
    let connection_status_class = match &state.connection_status {
        crate::store::ConnectionStatus::Connected => "status-connected",
        crate::store::ConnectionStatus::Connecting => "status-connecting",
        crate::store::ConnectionStatus::Disconnected => "status-disconnected",
        crate::store::ConnectionStatus::Error(_) => "status-error",
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
                    <div class={classes!("connection-status", connection_status_class)}>
                        <span class="status-indicator"></span>
                        <span class="status-text">
                            {match &state.connection_status {
                                crate::store::ConnectionStatus::Connected => i18n.t("status.connected"),
                                crate::store::ConnectionStatus::Connecting => i18n.t("status.connecting"),
                                crate::store::ConnectionStatus::Disconnected => i18n.t("status.disconnected"),
                                crate::store::ConnectionStatus::Error(_) => i18n.t("status.error"),
                            }}
                        </span>
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