use std::collections::HashMap;
use yewdux::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use shared::{ChatMessage, Language, CropType, UserProfile, VisionResponse, LLMResponse};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Store)]
pub struct AppState {
    pub language: Language,
    pub user_profile: Option<UserProfile>,
    pub conversation_id: Uuid,
    pub messages: Vec<ChatMessage>,
    pub is_loading: bool,
    pub current_image: Option<ImageState>,
    pub last_diagnosis: Option<DiagnosisState>,
    pub connection_status: ConnectionStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageState {
    pub data: String,        // base64 encoded
    pub crop_type: CropType,
    pub metadata: ImageMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub size_bytes: u64,
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub filename: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosisState {
    pub vision_result: VisionResponse,
    pub llm_result: Option<LLMResponse>,
    pub is_processing_advice: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
    Error(String),
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            language: Language::Thai,
            user_profile: None,
            conversation_id: Uuid::new_v4(),
            messages: Vec::new(),
            is_loading: false,
            current_image: None,
            last_diagnosis: None,
            connection_status: ConnectionStatus::Disconnected,
            error_message: None,
        }
    }
}

// Actions for state management
pub enum AppAction {
    SetLanguage(Language),
    SetUserProfile(UserProfile),
    AddMessage(ChatMessage),
    SetLoading(bool),
    SetCurrentImage(ImageState),
    ClearCurrentImage,
    SetDiagnosis(DiagnosisState),
    SetConnectionStatus(ConnectionStatus),
    SetError(Option<String>),
    ClearMessages,
    StartNewConversation,
}

impl Reducer<AppState> for AppAction {
    fn apply(self, mut state: std::rc::Rc<AppState>) -> std::rc::Rc<AppState> {
        let state = std::rc::Rc::make_mut(&mut state);
        
        match self {
            AppAction::SetLanguage(lang) => {
                state.language = lang;
            }
            AppAction::SetUserProfile(profile) => {
                state.user_profile = Some(profile);
            }
            AppAction::AddMessage(message) => {
                state.messages.push(message);
            }
            AppAction::SetLoading(loading) => {
                state.is_loading = loading;
            }
            AppAction::SetCurrentImage(image) => {
                state.current_image = Some(image);
            }
            AppAction::ClearCurrentImage => {
                state.current_image = None;
            }
            AppAction::SetDiagnosis(diagnosis) => {
                state.last_diagnosis = Some(diagnosis);
            }
            AppAction::SetConnectionStatus(status) => {
                state.connection_status = status;
            }
            AppAction::SetError(error) => {
                state.error_message = error;
            }
            AppAction::ClearMessages => {
                state.messages.clear();
            }
            AppAction::StartNewConversation => {
                state.conversation_id = Uuid::new_v4();
                state.messages.clear();
                state.current_image = None;
                state.last_diagnosis = None;
                state.error_message = None;
            }
        }
        
        state.into()
    }
}