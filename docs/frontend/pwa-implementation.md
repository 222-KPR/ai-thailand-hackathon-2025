# PWA Implementation Guide

This document outlines the Progressive Web App (PWA) implementation for AI4Thai Crop Guardian, focusing on offline functionality, performance optimization, and native app-like experience.

## 🎯 PWA Objectives

### Core Goals
- **Offline Functionality**: Core features work without internet connection
- **App-like Experience**: Native app feel with smooth interactions
- **Fast Loading**: Sub-2 second initial load times
- **Reliable Performance**: Consistent experience across devices
- **Installable**: Home screen installation capability

### Target Metrics
- **Lighthouse PWA Score**: >90
- **Performance Score**: >90
- **Accessibility Score**: >90
- **Best Practices Score**: >90
- **SEO Score**: >90

## 🏗️ PWA Architecture

### Service Worker Strategy
```javascript
// sw.js - Service Worker implementation
const CACHE_NAME = 'ai4thai-v1.0.0';
const STATIC_CACHE = 'ai4thai-static-v1.0.0';
const DYNAMIC_CACHE = 'ai4thai-dynamic-v1.0.0';
const API_CACHE = 'ai4thai-api-v1.0.0';

// Static assets to cache immediately
const STATIC_ASSETS = [
    '/',
    '/index.html',
    '/manifest.json',
    '/static/css/main.css',
    '/static/js/main.js',
    '/static/fonts/poppins-regular.woff2',
    '/static/fonts/sarabun-regular.woff2',
    '/static/icons/icon-192.png',
    '/static/icons/icon-512.png',
    '/offline.html'
];

// API endpoints to cache
const API_ENDPOINTS = [
    '/api/profile',
    '/api/diagnoses',
    '/api/chat/conversations'
];

// Install event - cache static assets
self.addEventListener('install', event => {
    event.waitUntil(
        caches.open(STATIC_CACHE)
            .then(cache => cache.addAll(STATIC_ASSETS))
            .then(() => self.skipWaiting())
    );
});

// Activate event - clean up old caches
self.addEventListener('activate', event => {
    event.waitUntil(
        caches.keys()
            .then(cacheNames => {
                return Promise.all(
                    cacheNames
                        .filter(cacheName => 
                            cacheName !== STATIC_CACHE && 
                            cacheName !== DYNAMIC_CACHE &&
                            cacheName !== API_CACHE
                        )
                        .map(cacheName => caches.delete(cacheName))
                );
            })
            .then(() => self.clients.claim())
    );
});

// Fetch event - implement caching strategies
self.addEventListener('fetch', event => {
    const { request } = event;
    const url = new URL(request.url);

    // Handle API requests
    if (url.pathname.startsWith('/api/')) {
        event.respondWith(handleApiRequest(request));
    }
    // Handle static assets
    else if (STATIC_ASSETS.includes(url.pathname)) {
        event.respondWith(handleStaticAsset(request));
    }
    // Handle dynamic content
    else {
        event.respondWith(handleDynamicContent(request));
    }
});

// API request handler - Network First with Cache Fallback
async function handleApiRequest(request) {
    try {
        const networkResponse = await fetch(request);
        
        if (networkResponse.ok) {
            const cache = await caches.open(API_CACHE);
            cache.put(request, networkResponse.clone());
        }
        
        return networkResponse;
    } catch (error) {
        const cachedResponse = await caches.match(request);
        if (cachedResponse) {
            return cachedResponse;
        }
        
        // Return offline fallback for critical endpoints
        if (request.url.includes('/api/profile')) {
            return new Response(JSON.stringify({
                offline: true,
                message: 'Profile data unavailable offline'
            }), {
                headers: { 'Content-Type': 'application/json' }
            });
        }
        
        throw error;
    }
}

// Static asset handler - Cache First
async function handleStaticAsset(request) {
    const cachedResponse = await caches.match(request);
    if (cachedResponse) {
        return cachedResponse;
    }
    
    const networkResponse = await fetch(request);
    const cache = await caches.open(STATIC_CACHE);
    cache.put(request, networkResponse.clone());
    
    return networkResponse;
}

// Dynamic content handler - Stale While Revalidate
async function handleDynamicContent(request) {
    const cache = await caches.open(DYNAMIC_CACHE);
    const cachedResponse = await cache.match(request);
    
    const fetchPromise = fetch(request).then(networkResponse => {
        cache.put(request, networkResponse.clone());
        return networkResponse;
    });
    
    return cachedResponse || fetchPromise;
}
```

### Web App Manifest
```json
{
  "name": "AI4Thai Crop Guardian",
  "short_name": "AI4Thai",
  "description": "AI-powered crop disease detection for Thai farmers",
  "start_url": "/",
  "display": "standalone",
  "orientation": "portrait-primary",
  "theme_color": "#0066FF",
  "background_color": "#FFFFFF",
  "categories": ["agriculture", "productivity", "utilities"],
  "lang": "th",
  "dir": "ltr",
  "icons": [
    {
      "src": "/static/icons/icon-72.png",
      "sizes": "72x72",
      "type": "image/png",
      "purpose": "maskable"
    },
    {
      "src": "/static/icons/icon-96.png",
      "sizes": "96x96",
      "type": "image/png",
      "purpose": "maskable"
    },
    {
      "src": "/static/icons/icon-128.png",
      "sizes": "128x128",
      "type": "image/png",
      "purpose": "maskable"
    },
    {
      "src": "/static/icons/icon-144.png",
      "sizes": "144x144",
      "type": "image/png",
      "purpose": "maskable"
    },
    {
      "src": "/static/icons/icon-152.png",
      "sizes": "152x152",
      "type": "image/png",
      "purpose": "maskable"
    },
    {
      "src": "/static/icons/icon-192.png",
      "sizes": "192x192",
      "type": "image/png",
      "purpose": "any maskable"
    },
    {
      "src": "/static/icons/icon-384.png",
      "sizes": "384x384",
      "type": "image/png",
      "purpose": "any maskable"
    },
    {
      "src": "/static/icons/icon-512.png",
      "sizes": "512x512",
      "type": "image/png",
      "purpose": "any maskable"
    }
  ],
  "screenshots": [
    {
      "src": "/static/screenshots/desktop-1.png",
      "sizes": "1280x720",
      "type": "image/png",
      "form_factor": "wide"
    },
    {
      "src": "/static/screenshots/mobile-1.png",
      "sizes": "375x667",
      "type": "image/png",
      "form_factor": "narrow"
    }
  ],
  "shortcuts": [
    {
      "name": "Take Photo",
      "short_name": "Camera",
      "description": "Capture crop image for diagnosis",
      "url": "/camera",
      "icons": [
        {
          "src": "/static/icons/camera-96.png",
          "sizes": "96x96"
        }
      ]
    },
    {
      "name": "Chat",
      "short_name": "Chat",
      "description": "Ask agricultural questions",
      "url": "/chat",
      "icons": [
        {
          "src": "/static/icons/chat-96.png",
          "sizes": "96x96"
        }
      ]
    }
  ]
}
```

## 🔧 Rust/Yew Integration

### Service Worker Registration
```rust
// src/utils/pwa.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ServiceWorkerContainer, ServiceWorkerRegistration};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = navigator)]
    static serviceWorker: ServiceWorkerContainer;
}

pub async fn register_service_worker() -> Result<(), JsValue> {
    if !is_service_worker_supported() {
        log::warn!("Service Worker not supported");
        return Ok(());
    }

    let registration_promise = serviceWorker.register("/sw.js");
    let registration = JsFuture::from(registration_promise).await?;
    
    log::info!("Service Worker registered successfully");
    
    // Listen for updates
    setup_update_listener(registration.into())?;
    
    Ok(())
}

fn is_service_worker_supported() -> bool {
    js_sys::Reflect::has(&web_sys::window().unwrap().navigator(), &"serviceWorker".into())
        .unwrap_or(false)
}

fn setup_update_listener(registration: ServiceWorkerRegistration) -> Result<(), JsValue> {
    let callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        // Show update available notification
        show_update_notification();
    }) as Box<dyn FnMut(_)>);
    
    registration.add_event_listener_with_callback("updatefound", callback.as_ref().unchecked_ref())?;
    callback.forget();
    
    Ok(())
}

fn show_update_notification() {
    // Implement update notification UI
    log::info!("App update available");
}
```

### Offline State Management
```rust
// src/stores/offline_store.rs
use yewdux::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Store)]
pub struct OfflineStore {
    pub is_online: bool,
    pub pending_uploads: Vec<PendingUpload>,
    pub cached_diagnoses: Vec<CachedDiagnosis>,
    pub sync_queue: Vec<SyncItem>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingUpload {
    pub id: String,
    pub image_data: Vec<u8>,
    pub crop_type: String,
    pub timestamp: f64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CachedDiagnosis {
    pub id: String,
    pub result: DiagnosisResult,
    pub cached_at: f64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncItem {
    pub id: String,
    pub action: SyncAction,
    pub data: serde_json::Value,
    pub timestamp: f64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum SyncAction {
    UploadDiagnosis,
    SendMessage,
    UpdateProfile,
}

pub enum OfflineAction {
    SetOnlineStatus(bool),
    AddPendingUpload(PendingUpload),
    RemovePendingUpload(String),
    CacheDiagnosis(CachedDiagnosis),
    AddSyncItem(SyncItem),
    ProcessSyncQueue,
}

impl Reducer<OfflineStore> for OfflineAction {
    fn apply(self, mut state: Rc<OfflineStore>) -> Rc<OfflineStore> {
        let state = Rc::make_mut(&mut state);
        
        match self {
            OfflineAction::SetOnlineStatus(is_online) => {
                state.is_online = is_online;
                if is_online {
                    // Trigger sync when coming back online
                    spawn_local(sync_pending_data());
                }
            }
            OfflineAction::AddPendingUpload(upload) => {
                state.pending_uploads.push(upload);
            }
            OfflineAction::RemovePendingUpload(id) => {
                state.pending_uploads.retain(|upload| upload.id != id);
            }
            OfflineAction::CacheDiagnosis(diagnosis) => {
                state.cached_diagnoses.push(diagnosis);
            }
            OfflineAction::AddSyncItem(item) => {
                state.sync_queue.push(item);
            }
            OfflineAction::ProcessSyncQueue => {
                // Process sync queue
            }
        }
        
        state.into()
    }
}

async fn sync_pending_data() {
    // Implement background sync logic
}
```

### Offline-First Components
```rust
// src/components/offline/offline_indicator.rs
use yew::prelude::*;
use yewdux::prelude::*;
use crate::stores::offline_store::OfflineStore;

#[function_component(OfflineIndicator)]
pub fn offline_indicator() -> Html {
    let offline_state = use_store_value::<OfflineStore>();
    
    if offline_state.is_online {
        return html! {};
    }
    
    html! {
        <div class="offline-indicator">
            <div class="offline-indicator__content">
                <i class="icon-wifi-off"></i>
                <span>{ "Working offline" }</span>
                if !offline_state.pending_uploads.is_empty() {
                    <span class="pending-count">
                        { format!("{} pending", offline_state.pending_uploads.len()) }
                    </span>
                }
            </div>
        </div>
    }
}

// src/components/camera/offline_camera.rs
use yew::prelude::*;
use yewdux::prelude::*;
use crate::stores::offline_store::{OfflineStore, OfflineAction, PendingUpload};

#[function_component(OfflineCamera)]
pub fn offline_camera() -> Html {
    let offline_dispatch = use_dispatch::<OfflineStore>();
    let offline_state = use_store_value::<OfflineStore>();
    
    let on_capture = {
        let dispatch = offline_dispatch.clone();
        let is_online = offline_state.is_online;
        
        Callback::from(move |image_data: Vec<u8>| {
            if is_online {
                // Process immediately
                spawn_local(process_image_online(image_data));
            } else {
                // Queue for later processing
                let pending_upload = PendingUpload {
                    id: generate_id(),
                    image_data,
                    crop_type: "rice".to_string(),
                    timestamp: js_sys::Date::now(),
                };
                
                dispatch.apply(OfflineAction::AddPendingUpload(pending_upload));
                show_offline_message();
            }
        })
    };
    
    html! {
        <div class="offline-camera">
            <CameraCapture on_capture={on_capture} />
            
            if !offline_state.pending_uploads.is_empty() {
                <div class="pending-uploads">
                    <h3>{ "Pending Uploads" }</h3>
                    { for offline_state.pending_uploads.iter().map(|upload| {
                        html! {
                            <div key={upload.id.clone()} class="pending-upload">
                                <span>{ format!("Image captured at {}", format_timestamp(upload.timestamp)) }</span>
                                <span class="crop-type">{ &upload.crop_type }</span>
                            </div>
                        }
                    })}
                </div>
            }
        </div>
    }
}

async fn process_image_online(image_data: Vec<u8>) {
    // Process image when online
}

fn show_offline_message() {
    // Show user feedback for offline capture
}

fn generate_id() -> String {
    // Generate unique ID
    format!("upload_{}", js_sys::Date::now())
}

fn format_timestamp(timestamp: f64) -> String {
    // Format timestamp for display
    let date = js_sys::Date::new(&timestamp.into());
    date.to_locale_string("th-TH", &js_sys::Object::new()).into()
}
```

## 📱 Installation and App-like Experience

### Installation Prompt
```rust
// src/components/pwa/install_prompt.rs
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type BeforeInstallPromptEvent;
    
    #[wasm_bindgen(method)]
    fn prompt(this: &BeforeInstallPromptEvent) -> js_sys::Promise;
    
    #[wasm_bindgen(method, getter)]
    fn userChoice(this: &BeforeInstallPromptEvent) -> js_sys::Promise;
}

#[function_component(InstallPrompt)]
pub fn install_prompt() -> Html {
    let show_prompt = use_state(|| false);
    let install_event = use_state(|| None::<BeforeInstallPromptEvent>);
    
    // Listen for beforeinstallprompt event
    {
        let show_prompt = show_prompt.clone();
        let install_event = install_event.clone();
        
        use_effect_with((), move |_| {
            let callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
                event.prevent_default();
                install_event.set(Some(event.unchecked_into()));
                show_prompt.set(true);
            }) as Box<dyn FnMut(_)>);
            
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("beforeinstallprompt", callback.as_ref().unchecked_ref())
                .unwrap();
            
            move || {
                callback.forget();
            }
        });
    }
    
    let on_install = {
        let install_event = install_event.clone();
        let show_prompt = show_prompt.clone();
        
        Callback::from(move |_| {
            if let Some(event) = (*install_event).as_ref() {
                let promise = event.prompt();
                show_prompt.set(false);
                
                wasm_bindgen_futures::spawn_local(async move {
                    let _result = wasm_bindgen_futures::JsFuture::from(promise).await;
                    // Handle installation result
                });
            }
        })
    };
    
    let on_dismiss = {
        let show_prompt = show_prompt.clone();
        Callback::from(move |_| {
            show_prompt.set(false);
        })
    };
    
    if !*show_prompt {
        return html! {};
    }
    
    html! {
        <div class="install-prompt">
            <div class="install-prompt__content">
                <div class="install-prompt__icon">
                    <img src="/static/icons/icon-96.png" alt="AI4Thai Icon" />
                </div>
                <div class="install-prompt__text">
                    <h3>{ "Install AI4Thai" }</h3>
                    <p>{ "Get the full app experience with offline access and faster loading." }</p>
                </div>
                <div class="install-prompt__actions">
                    <button class="btn btn--primary" onclick={on_install}>
                        { "Install" }
                    </button>
                    <button class="btn btn--ghost" onclick={on_dismiss}>
                        { "Not now" }
                    </button>
                </div>
            </div>
        </div>
    }
}
```

## 🔔 Push Notifications

### Notification Service
```rust
// src/services/notification_service.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Notification, NotificationPermission, PushManager, ServiceWorkerRegistration};

pub struct NotificationService {
    registration: Option<ServiceWorkerRegistration>,
}

impl NotificationService {
    pub fn new() -> Self {
        Self {
            registration: None,
        }
    }
    
    pub async fn request_permission(&self) -> Result<NotificationPermission, JsValue> {
        let permission_promise = Notification::request_permission();
        let permission = JsFuture::from(permission_promise).await?;
        Ok(permission.into())
    }
    
    pub async fn subscribe_to_push(&mut self) -> Result<(), JsValue> {
        let registration = self.get_registration().await?;
        let push_manager = registration.push_manager()?;
        
        let subscription_options = js_sys::Object::new();
        js_sys::Reflect::set(
            &subscription_options,
            &"userVisibleOnly".into(),
            &true.into(),
        )?;
        
        let vapid_key = "YOUR_VAPID_PUBLIC_KEY"; // Replace with actual VAPID key
        let application_server_key = base64_to_uint8_array(vapid_key)?;
        js_sys::Reflect::set(
            &subscription_options,
            &"applicationServerKey".into(),
            &application_server_key,
        )?;
        
        let subscription_promise = push_manager.subscribe_with_options(&subscription_options)?;
        let _subscription = JsFuture::from(subscription_promise).await?;
        
        // Send subscription to server
        self.send_subscription_to_server().await?;
        
        Ok(())
    }
    
    pub fn show_local_notification(&self, title: &str, body: &str, icon: Option<&str>) -> Result<(), JsValue> {
        let options = web_sys::NotificationOptions::new();
        options.set_body(body);
        options.set_icon(icon.unwrap_or("/static/icons/icon-192.png"));
        options.set_badge("/static/icons/badge-72.png");
        
        let _notification = Notification::new_with_options(title, &options)?;
        Ok(())
    }
    
    async fn get_registration(&mut self) -> Result<ServiceWorkerRegistration, JsValue> {
        if let Some(ref registration) = self.registration {
            return Ok(registration.clone());
        }
        
        let navigator = web_sys::window().unwrap().navigator();
        let service_worker = navigator.service_worker();
        let registration_promise = service_worker.ready();
        let registration = JsFuture::from(registration_promise).await?;
        
        let registration: ServiceWorkerRegistration = registration.into();
        self.registration = Some(registration.clone());
        
        Ok(registration)
    }
    
    async fn send_subscription_to_server(&self) -> Result<(), JsValue> {
        // Implement server communication
        Ok(())
    }
}

fn base64_to_uint8_array(base64_string: &str) -> Result<js_sys::Uint8Array, JsValue> {
    let window = web_sys::window().unwrap();
    let decoded = window.atob(base64_string)?;
    let array = js_sys::Uint8Array::new_with_length(decoded.len() as u32);
    
    for (i, byte) in decoded.chars().enumerate() {
        array.set_index(i as u32, byte as u32 as u8);
    }
    
    Ok(array)
}
```

## 📊 Performance Optimization

### Bundle Splitting
```rust
// Trunk.toml configuration
[build]
target = "index.html"
dist = "dist"
public_url = "/"

[[hooks]]
stage = "pre_build"
command = "wasm-opt"
command_arguments = ["--enable-bulk-memory", "-Oz"]

[[hooks]]
stage = "post_build"
command = "workbox"
command_arguments = ["generateSW", "workbox-config.js"]

[watch]
watch = ["src", "static"]
ignore = ["dist"]

[serve]
addresses = ["127.0.0.1", "[::1]"]
port = 8080
open = false
```

### Lazy Loading
```rust
// src/utils/lazy_loading.rs
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/camera")]
    Camera,
    #[at("/chat")]
    Chat,
    #[at("/profile")]
    Profile,
    #[at("/history")]
    History,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Camera => html! { <Suspense fallback={loading_component()}><CameraPage /></Suspense> },
        Route::Chat => html! { <Suspense fallback={loading_component()}><ChatPage /></Suspense> },
        Route::Profile => html! { <Suspense fallback={loading_component()}><ProfilePage /></Suspense> },
        Route::History => html! { <Suspense fallback={loading_component()}><HistoryPage /></Suspense> },
    }
}

fn loading_component() -> Html {
    html! {
        <div class="loading-container">
            <div class="loading-spinner"></div>
            <p>{ "Loading..." }</p>
        </div>
    }
}
```

## 🧪 PWA Testing

### Lighthouse Testing
```bash
# Install Lighthouse CLI
npm install -g lighthouse

# Run PWA audit
lighthouse http://localhost:8080 --only-categories=pwa --chrome-flags="--headless"

# Generate full report
lighthouse http://localhost:8080 --output=html --output-path=./lighthouse-report.html
```

### PWA Testing Checklist
- [ ] **Installable**: App can be installed from browser
- [ ] **Offline Functionality**: Core features work offline
- [ ] **Responsive Design**: Works on all screen sizes
- [ ] **Fast Loading**: Loads in under 3 seconds
- [ ] **Secure Context**: Served over HTTPS
- [ ] **Service Worker**: Properly registered and functioning
- [ ] **Web App Manifest**: Valid and complete
- [ ] **Push Notifications**: Working when supported
- [ ] **Background Sync**: Syncs data when online
- [ ] **App Shell**: Cached for instant loading

This PWA implementation ensures that AI4Thai Crop Guardian provides a native app-like experience with robust offline functionality, making it accessible to Thai farmers even in areas with limited internet connectivity.
