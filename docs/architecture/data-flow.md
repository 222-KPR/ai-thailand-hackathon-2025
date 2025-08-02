# Data Flow Diagrams

## System-Level Data Flow Diagram

```mermaid
flowchart TD
    %% External Entities
    User[👤 User/Farmer]
    Admin[👨‍💼 Admin]
    Weather[🌤️ Weather API]
    HF[🤗 HuggingFace Hub]
    
    %% Main System Components
    Frontend[📱 Frontend PWA<br/>Yew WebAssembly]
    Gateway[🔌 API Gateway<br/>Rust/Axum]
    
    %% AI Services
    Vision[👁️ Vision Service<br/>FastAPI + HuggingFace]
    LLM[🧠 LLM Service<br/>FastAPI + HuggingFace]
    
    %% Core Services
    Queue[⚡ Queue Worker<br/>Rust]
    
    %% Data Stores
    DB[(🗄️ PostgreSQL<br/>Primary Database)]
    Cache[(⚡ Redis<br/>Cache & Queue)]
    Files[(📁 File Storage<br/>Images & Models)]
    
    %% Monitoring
    Metrics[📊 Metrics<br/>Prometheus]
    Logs[📝 Logs<br/>ELK Stack]
    
    %% Data Flows
    User -.->|"1. User Input<br/>(Images, Text, Voice)"| Frontend
    Frontend -->|"2. API Requests<br/>(HTTP/WebSocket)"| Gateway
    
    Gateway -->|"3a. Image Analysis<br/>(Multipart Form)"| Vision
    Gateway -->|"3b. Chat Messages<br/>(JSON)"| LLM
    Gateway -->|"3c. Background Jobs<br/>(Queue Messages)"| Queue
    
    Vision -->|"4a. Disease Detection<br/>(JSON Response)"| Gateway
    LLM -->|"4b. AI Responses<br/>(JSON Response)"| Gateway
    
    Gateway -->|"5. Processed Data<br/>(JSON Response)"| Frontend
    Frontend -.->|"6. Results Display<br/>(UI Updates)"| User
    
    %% Data Storage Flows
    Vision -->|"Store Diagnoses"| DB
    LLM -->|"Store Conversations"| DB
    Gateway -->|"Store User Data"| DB
    Queue -->|"Update Records"| DB
    
    Vision -->|"Cache Results"| Cache
    LLM -->|"Cache Contexts"| Cache
    Gateway -->|"Session Data"| Cache
    Queue -->|"Job Queue"| Cache
    
    Vision -->|"Store Images"| Files
    Vision -->|"Load Models"| Files
    LLM -->|"Load Models"| Files
    
    %% External Data Flows
    Gateway -->|"Weather Requests"| Weather
    Weather -->|"Weather Data"| Gateway
    
    Vision -->|"Download Models"| HF
    LLM -->|"Download Models"| HF
    
    %% Monitoring Flows
    Gateway -->|"Metrics"| Metrics
    Vision -->|"Metrics"| Metrics
    LLM -->|"Metrics"| Metrics
    Queue -->|"Metrics"| Metrics
    
    Gateway -->|"Logs"| Logs
    Vision -->|"Logs"| Logs
    LLM -->|"Logs"| Logs
    Queue -->|"Logs"| Logs
    
    %% Admin Flows
    Admin -.->|"Admin Interface"| Frontend
    Admin -.->|"Direct Access"| Metrics
    Admin -.->|"Direct Access"| Logs
    
    %% Styling
    classDef userEntity fill:#e1f5fe
    classDef systemComponent fill:#f3e5f5
    classDef aiService fill:#e8f5e8
    classDef dataStore fill:#fce4ec
    classDef monitoring fill:#fff3e0
    
    class User,Admin userEntity
    class Frontend,Gateway,Queue systemComponent
    class Vision,LLM aiService
    class DB,Cache,Files dataStore
    class Metrics,Logs,Weather,HF monitoring
```

## Disease Detection Data Flow

```mermaid
flowchart TD
    %% Input Stage
    User[👤 User]
    Camera[📷 Camera/Gallery]
    ImageFile[🖼️ Image File<br/>JPG/PNG/WebP<br/>Max 10MB]
    
    %% Frontend Processing
    Frontend[📱 Frontend]
    Validation[✅ Client Validation<br/>• Format check<br/>• Size check<br/>• Quality check]
    
    %% API Gateway
    Gateway[🔌 API Gateway]
    Auth[🔐 Authentication<br/>JWT Validation]
    RateLimit[⏱️ Rate Limiting<br/>50 req/hour]
    
    %% Vision Service Processing
    Vision[👁️ Vision Service]
    ImageProcessor[🔄 Image Processor<br/>• Resize to 256x256<br/>• Normalize pixels<br/>• Quality assessment]
    
    ModelRegistry[🤖 Model Registry<br/>• Crop Classifier<br/>• Disease Detector]
    
    InferenceEngine[⚙️ Inference Engine<br/>• Load models<br/>• Run predictions<br/>• Calculate confidence]
    
    %% Models
    CropModel[🌾 Crop Classification<br/>ViT-Base-Patch16-224<br/>Input: 224x224<br/>Output: Crop type + confidence]
    
    DiseaseModel[🦠 Disease Detection<br/>ResNet-50 Fine-tuned<br/>Input: 256x256<br/>Output: Disease + confidence]
    
    %% Result Processing
    ResultFormatter[📋 Result Formatter<br/>• Format predictions<br/>• Add metadata<br/>• Generate recommendations]
    
    %% LLM Service for Advice
    LLM[🧠 LLM Service]
    AdviceGenerator[💡 Advice Generator<br/>• Treatment recommendations<br/>• Prevention tips<br/>• Cost estimates]
    
    %% Data Storage
    DB[(🗄️ Database<br/>• Diagnosis records<br/>• User history<br/>• Statistics)]
    
    Cache[(⚡ Cache<br/>• Model predictions<br/>• User sessions<br/>• Frequent queries)]
    
    FileStorage[(📁 File Storage<br/>• Original images<br/>• Processed images<br/>• Model files)]
    
    %% Output
    Results[📊 Diagnosis Results<br/>• Disease identification<br/>• Confidence scores<br/>• Treatment advice<br/>• Prevention tips]
    
    %% Data Flow
    User --> Camera
    Camera --> ImageFile
    ImageFile --> Frontend
    Frontend --> Validation
    Validation --> Gateway
    
    Gateway --> Auth
    Auth --> RateLimit
    RateLimit --> Vision
    
    Vision --> ImageProcessor
    ImageProcessor --> ModelRegistry
    ModelRegistry --> InferenceEngine
    
    InferenceEngine --> CropModel
    InferenceEngine --> DiseaseModel
    
    CropModel --> ResultFormatter
    DiseaseModel --> ResultFormatter
    
    ResultFormatter --> LLM
    LLM --> AdviceGenerator
    AdviceGenerator --> ResultFormatter
    
    ResultFormatter --> DB
    ResultFormatter --> Cache
    ImageProcessor --> FileStorage
    
    ResultFormatter --> Gateway
    Gateway --> Frontend
    Frontend --> Results
    Results --> User
    
    %% Data Annotations
    ImageFile -.->|"Raw Image Data<br/>Binary format"| Frontend
    Frontend -.->|"Multipart Form<br/>image + metadata"| Gateway
    Gateway -.->|"HTTP Request<br/>+ Authentication"| Vision
    
    Vision -.->|"Processed Image<br/>Tensor format"| InferenceEngine
    InferenceEngine -.->|"Prediction Vectors<br/>Probabilities"| ResultFormatter
    
    ResultFormatter -.->|"Structured Data<br/>JSON format"| Gateway
    Gateway -.->|"API Response<br/>JSON + HTTP headers"| Frontend
    Frontend -.->|"UI Components<br/>Rendered results"| User
    
    %% Storage Annotations
    DB -.->|"SQL Queries<br/>CRUD operations"| Vision
    Cache -.->|"Key-Value pairs<br/>TTL-based"| Vision
    FileStorage -.->|"Binary files<br/>S3-compatible"| Vision
    
    %% Styling
    classDef input fill:#e3f2fd
    classDef processing fill:#f3e5f5
    classDef aiModel fill:#e8f5e8
    classDef storage fill:#fce4ec
    classDef output fill:#fff3e0
    
    class User,Camera,ImageFile input
    class Frontend,Gateway,Vision,ImageProcessor,ModelRegistry,InferenceEngine,ResultFormatter,LLM,AdviceGenerator processing
    class CropModel,DiseaseModel aiModel
    class DB,Cache,FileStorage storage
    class Results output
```

## Chat Conversation Data Flow

```mermaid
flowchart TD
    %% Input Sources
    User[👤 User]
    TextInput[💬 Text Input<br/>Thai/English<br/>Max 1000 chars]
    VoiceInput[🎤 Voice Input<br/>Speech-to-Text<br/>Thai language]
    ImageInput[🖼️ Image Context<br/>Crop photos<br/>Disease images]
    
    %% Frontend Processing
    Frontend[📱 Frontend]
    InputProcessor[🔄 Input Processor<br/>• Text validation<br/>• Voice transcription<br/>• Image preprocessing]
    
    %% Real-time Communication
    WebSocket[🔌 WebSocket<br/>Real-time messaging<br/>Bidirectional]
    
    %% API Gateway
    Gateway[🔌 API Gateway]
    MessageRouter[📮 Message Router<br/>• Route to LLM service<br/>• Handle WebSocket<br/>• Manage sessions]
    
    %% LLM Service Components
    LLM[🧠 LLM Service]
    
    ContextManager[🧠 Context Manager<br/>• Conversation history<br/>• User preferences<br/>• Agricultural context]
    
    ChatEngine[💭 Chat Engine<br/>• Intent recognition<br/>• Response planning<br/>• Context integration]
    
    ResponseGenerator[✍️ Response Generator<br/>• Text generation<br/>• Thai language processing<br/>• Agricultural advice]
    
    %% AI Models
    AgriAdvisor[🌾 Agricultural Advisor<br/>DialoGPT-Large<br/>Fine-tuned for farming<br/>Context: 512 tokens]
    
    ThaiLM[🇹🇭 Thai Language Model<br/>WangchanBERTa<br/>Thai understanding<br/>Agricultural terminology]
    
    Translator[🔄 Translation Model<br/>Thai ↔ English<br/>Bidirectional<br/>Agricultural context]
    
    %% Knowledge Base
    KnowledgeBase[📚 Knowledge Base<br/>• Crop diseases<br/>• Treatment methods<br/>• Best practices<br/>• Local conditions]
    
    %% Data Storage
    ConversationDB[(💬 Conversation Store<br/>• Message history<br/>• User contexts<br/>• Conversation metadata)]
    
    ContextCache[(🧠 Context Cache<br/>• Active conversations<br/>• User preferences<br/>• Recent interactions)]
    
    %% External Services
    Translation[🌐 Translation API<br/>Google Translate<br/>Fallback service]
    
    %% Output Processing
    ResponseFormatter[📝 Response Formatter<br/>• Format for display<br/>• Add metadata<br/>• Include sources]
    
    %% Final Output
    ChatResponse[💬 Chat Response<br/>• AI-generated text<br/>• Confidence scores<br/>• Source references<br/>• Suggested actions]
    
    %% Data Flow - Input Processing
    User --> TextInput
    User --> VoiceInput
    User --> ImageInput
    
    TextInput --> Frontend
    VoiceInput --> Frontend
    ImageInput --> Frontend
    
    Frontend --> InputProcessor
    InputProcessor --> WebSocket
    WebSocket --> Gateway
    Gateway --> MessageRouter
    MessageRouter --> LLM
    
    %% Data Flow - LLM Processing
    LLM --> ContextManager
    ContextManager --> ContextCache
    ContextCache --> ContextManager
    
    ContextManager --> ChatEngine
    ChatEngine --> ResponseGenerator
    
    ResponseGenerator --> AgriAdvisor
    ResponseGenerator --> ThaiLM
    ResponseGenerator --> Translator
    
    AgriAdvisor --> ResponseGenerator
    ThaiLM --> ResponseGenerator
    Translator --> ResponseGenerator
    
    ResponseGenerator --> KnowledgeBase
    KnowledgeBase --> ResponseGenerator
    
    %% External Translation (Fallback)
    ResponseGenerator --> Translation
    Translation --> ResponseGenerator
    
    %% Data Flow - Output Processing
    ResponseGenerator --> ResponseFormatter
    ResponseFormatter --> ConversationDB
    ResponseFormatter --> ContextCache
    
    ResponseFormatter --> Gateway
    Gateway --> WebSocket
    WebSocket --> Frontend
    Frontend --> ChatResponse
    ChatResponse --> User
    
    %% Data Annotations
    TextInput -.->|"Plain text<br/>UTF-8 encoding"| Frontend
    VoiceInput -.->|"Audio data<br/>WAV/MP3 format"| Frontend
    ImageInput -.->|"Image context<br/>JPEG/PNG"| Frontend
    
    Frontend -.->|"WebSocket message<br/>JSON format"| Gateway
    Gateway -.->|"Structured request<br/>+ Authentication"| LLM
    
    LLM -.->|"Context data<br/>Conversation state"| ContextManager
    ContextManager -.->|"Enriched context<br/>+ History"| ChatEngine
    
    ChatEngine -.->|"Generation request<br/>+ Intent"| ResponseGenerator
    ResponseGenerator -.->|"Model input<br/>Tokenized text"| AgriAdvisor
    
    AgriAdvisor -.->|"Generated tokens<br/>Probability scores"| ResponseGenerator
    ResponseGenerator -.->|"Formatted response<br/>+ Metadata"| Gateway
    
    Gateway -.->|"WebSocket response<br/>JSON format"| Frontend
    Frontend -.->|"Rendered message<br/>UI components"| User
    
    %% Storage Annotations
    ConversationDB -.->|"Message persistence<br/>SQL operations"| LLM
    ContextCache -.->|"Fast retrieval<br/>Redis operations"| LLM
    KnowledgeBase -.->|"Knowledge queries<br/>Vector search"| ResponseGenerator
    
    %% Styling
    classDef input fill:#e3f2fd
    classDef frontend fill:#f3e5f5
    classDef gateway fill:#e1f5fe
    classDef llm fill:#e8f5e8
    classDef model fill:#c8e6c9
    classDef storage fill:#fce4ec
    classDef external fill:#fff3e0
    classDef output fill:#f9fbe7
    
    class User,TextInput,VoiceInput,ImageInput input
    class Frontend,InputProcessor,WebSocket frontend
    class Gateway,MessageRouter gateway
    class LLM,ContextManager,ChatEngine,ResponseGenerator,ResponseFormatter llm
    class AgriAdvisor,ThaiLM,Translator model
    class ConversationDB,ContextCache,KnowledgeBase storage
    class Translation external
    class ChatResponse output
```

## User Data Flow

```mermaid
flowchart TD
    %% User Interactions
    User[👤 User/Farmer]
    Registration[📝 Registration<br/>• Personal info<br/>• Farm details<br/>• Preferences]
    Login[🔐 Login<br/>• Email/password<br/>• JWT tokens<br/>• Session data]
    ProfileUpdate[👤 Profile Update<br/>• Personal info<br/>• Farm info<br/>• Preferences]
    
    %% Frontend Layer
    Frontend[📱 Frontend PWA]
    LocalStorage[💾 Local Storage<br/>• JWT tokens<br/>• User preferences<br/>• Offline data]
    
    %% API Gateway
    Gateway[🔌 API Gateway]
    AuthMiddleware[🔐 Auth Middleware<br/>• Token validation<br/>• User identification<br/>• Permission checks]
    
    %% User Service
    UserService[👤 User Service<br/>• Profile management<br/>• Authentication<br/>• Preferences]
    
    %% Authentication Components
    JWTHandler[🎫 JWT Handler<br/>• Token generation<br/>• Token validation<br/>• Refresh logic]
    
    PasswordManager[🔒 Password Manager<br/>• Hashing (bcrypt)<br/>• Validation<br/>• Security policies]
    
    %% Data Storage
    UserDB[(👥 User Database<br/>• User profiles<br/>• Authentication data<br/>• Farm information)]
    
    SessionCache[(🔄 Session Cache<br/>• Active sessions<br/>• Refresh tokens<br/>• User contexts)]
    
    %% Analytics and Tracking
    Analytics[📊 Analytics Service<br/>• User behavior<br/>• Feature usage<br/>• Performance metrics]
    
    AuditLog[(📝 Audit Log<br/>• Login attempts<br/>• Profile changes<br/>• Security events)]
    
    %% Notification System
    NotificationService[📢 Notification Service<br/>• Email notifications<br/>• Push notifications<br/>• SMS alerts]
    
    %% External Services
    EmailProvider[📧 Email Provider<br/>SMTP service<br/>Verification emails]
    
    %% Data Flows - Registration
    User --> Registration
    Registration --> Frontend
    Frontend --> Gateway
    Gateway --> UserService
    
    UserService --> PasswordManager
    PasswordManager --> UserService
    UserService --> UserDB
    UserService --> JWTHandler
    JWTHandler --> SessionCache
    
    UserService --> NotificationService
    NotificationService --> EmailProvider
    
    UserService --> Analytics
    UserService --> AuditLog
    
    %% Data Flows - Login
    User --> Login
    Login --> Frontend
    Frontend --> LocalStorage
    Frontend --> Gateway
    Gateway --> AuthMiddleware
    AuthMiddleware --> UserService
    
    UserService --> UserDB
    UserService --> PasswordManager
    UserService --> JWTHandler
    JWTHandler --> SessionCache
    
    %% Data Flows - Profile Update
    User --> ProfileUpdate
    ProfileUpdate --> Frontend
    Frontend --> Gateway
    Gateway --> AuthMiddleware
    AuthMiddleware --> UserService
    
    UserService --> UserDB
    UserService --> Analytics
    UserService --> AuditLog
    
    %% Response Flows
    UserService --> Gateway
    Gateway --> Frontend
    Frontend --> LocalStorage
    Frontend --> User
    
    %% Background Processes
    Analytics --> UserDB
    AuditLog --> UserDB
    
    %% Data Annotations
    Registration -.->|"User form data<br/>JSON format"| Frontend
    Login -.->|"Credentials<br/>Email + password"| Frontend
    ProfileUpdate -.->|"Updated fields<br/>Partial data"| Frontend
    
    Frontend -.->|"HTTP requests<br/>+ CSRF tokens"| Gateway
    Gateway -.->|"Authenticated requests<br/>+ User context"| UserService
    
    UserService -.->|"Hashed passwords<br/>Secure storage"| UserDB
    UserService -.->|"JWT tokens<br/>Signed & encrypted"| SessionCache
    
    UserService -.->|"User events<br/>Structured logs"| Analytics
    UserService -.->|"Security events<br/>Audit trail"| AuditLog
    
    NotificationService -.->|"Email templates<br/>HTML/text"| EmailProvider
    
    %% Cache and Performance
    SessionCache -.->|"Fast token lookup<br/>Redis operations"| AuthMiddleware
    LocalStorage -.->|"Client-side cache<br/>Browser storage"| Frontend
    
    %% Styling
    classDef user fill:#e3f2fd
    classDef frontend fill:#f3e5f5
    classDef gateway fill:#e1f5fe
    classDef service fill:#e8f5e8
    classDef auth fill:#fff3e0
    classDef storage fill:#fce4ec
    classDef external fill:#ffecb3
    classDef analytics fill:#f1f8e9
    
    class User,Registration,Login,ProfileUpdate user
    class Frontend,LocalStorage frontend
    class Gateway,AuthMiddleware gateway
    class UserService service
    class JWTHandler,PasswordManager auth
    class UserDB,SessionCache storage
    class EmailProvider external
    class Analytics,AuditLog,NotificationService analytics
```

## System Integration Data Flow

```mermaid
flowchart TD
    %% External Data Sources
    WeatherAPI[🌤️ Weather API<br/>OpenWeatherMap<br/>Real-time data]
    
    HuggingFace[🤗 HuggingFace Hub<br/>Model repository<br/>Pre-trained models]
    
    ExternalAPIs[🌐 External APIs<br/>• Translation services<br/>• SMS providers<br/>• Email services]
    
    %% Core System
    APIGateway[🔌 API Gateway<br/>Central orchestrator]
    
    %% Services
    VisionService[👁️ Vision Service<br/>Image analysis]
    LLMService[🧠 LLM Service<br/>Text generation]
    QueueWorker[⚡ Queue Worker<br/>Background processing]
    
    %% Data Processing Pipeline
    DataPipeline[🔄 Data Pipeline<br/>• ETL processes<br/>• Data validation<br/>• Transformation]
    
    %% Storage Systems
    PrimaryDB[(🗄️ Primary Database<br/>PostgreSQL<br/>Transactional data)]
    
    AnalyticsDB[(📊 Analytics Database<br/>Time-series data<br/>Metrics & logs)]
    
    CacheLayer[(⚡ Cache Layer<br/>Redis<br/>Fast access data)]
    
    FileStorage[(📁 File Storage<br/>S3-compatible<br/>Images & models)]
    
    %% Monitoring and Observability
    MetricsCollector[📊 Metrics Collector<br/>Prometheus<br/>System metrics]
    
    LogAggregator[📝 Log Aggregator<br/>ELK Stack<br/>Centralized logging]
    
    AlertManager[🚨 Alert Manager<br/>Notification system<br/>Incident response]
    
    %% Data Flows - External Integration
    WeatherAPI -->|"Weather data<br/>JSON/REST"| APIGateway
    HuggingFace -->|"Model downloads<br/>Binary files"| VisionService
    HuggingFace -->|"Model downloads<br/>Binary files"| LLMService
    ExternalAPIs -->|"Service responses<br/>Various formats"| APIGateway
    
    %% Data Flows - Internal Processing
    APIGateway -->|"Image requests<br/>Multipart form"| VisionService
    APIGateway -->|"Text requests<br/>JSON"| LLMService
    APIGateway -->|"Background jobs<br/>Queue messages"| QueueWorker
    
    VisionService -->|"Analysis results<br/>JSON"| APIGateway
    LLMService -->|"Generated responses<br/>JSON"| APIGateway
    QueueWorker -->|"Job results<br/>Status updates"| APIGateway
    
    %% Data Flows - Storage
    VisionService -->|"Diagnosis data<br/>SQL inserts"| PrimaryDB
    LLMService -->|"Conversation data<br/>SQL inserts"| PrimaryDB
    QueueWorker -->|"Job data<br/>SQL updates"| PrimaryDB
    
    VisionService -->|"Cached results<br/>Key-value pairs"| CacheLayer
    LLMService -->|"Context data<br/>Key-value pairs"| CacheLayer
    APIGateway -->|"Session data<br/>Key-value pairs"| CacheLayer
    
    VisionService -->|"Images & models<br/>Binary files"| FileStorage
    LLMService -->|"Model files<br/>Binary files"| FileStorage
    
    %% Data Flows - Analytics Pipeline
    APIGateway -->|"Request logs<br/>Structured data"| DataPipeline
    VisionService -->|"Analysis metrics<br/>Structured data"| DataPipeline
    LLMService -->|"Generation metrics<br/>Structured data"| DataPipeline
    
    DataPipeline -->|"Processed metrics<br/>Time-series data"| AnalyticsDB
    DataPipeline -->|"Aggregated data<br/>Summary statistics"| PrimaryDB
    
    %% Data Flows - Monitoring
    APIGateway -->|"System metrics<br/>Prometheus format"| MetricsCollector
    VisionService -->|"Service metrics<br/>Prometheus format"| MetricsCollector
    LLMService -->|"Service metrics<br/>Prometheus format"| MetricsCollector
    QueueWorker -->|"Worker metrics<br/>Prometheus format"| MetricsCollector
    
    APIGateway -->|"Application logs<br/>JSON format"| LogAggregator
    VisionService -->|"Service logs<br/>JSON format"| LogAggregator
    LLMService -->|"Service logs<br/>JSON format"| LogAggregator
    QueueWorker -->|"Worker logs<br/>JSON format"| LogAggregator
    
    MetricsCollector -->|"Alert conditions<br/>Threshold breaches"| AlertManager
    LogAggregator -->|"Error patterns<br/>Log analysis"| AlertManager
    
    %% Data Flows - Feedback Loop
    AnalyticsDB -->|"Performance insights<br/>Query results"| DataPipeline
    MetricsCollector -->|"System health<br/>Metric queries"| APIGateway
    AlertManager -->|"System status<br/>Health indicators"| APIGateway
    
    %% Data Annotations
    WeatherAPI -.->|"REST API calls<br/>HTTP/JSON"| APIGateway
    HuggingFace -.->|"Model downloads<br/>HTTPS/Binary"| VisionService
    
    APIGateway -.->|"Service calls<br/>HTTP/JSON"| VisionService
    VisionService -.->|"Database writes<br/>SQL/ACID"| PrimaryDB
    
    DataPipeline -.->|"ETL operations<br/>Batch/Stream"| AnalyticsDB
    MetricsCollector -.->|"Metrics scraping<br/>Pull model"| VisionService
    
    %% Styling
    classDef external fill:#ffecb3
    classDef gateway fill:#e1f5fe
    classDef service fill:#e8f5e8
    classDef storage fill:#fce4ec
    classDef monitoring fill:#fff3e0
    classDef pipeline fill:#f3e5f5
    
    class WeatherAPI,HuggingFace,ExternalAPIs external
    class APIGateway gateway
    class VisionService,LLMService,QueueWorker service
    class PrimaryDB,AnalyticsDB,CacheLayer,FileStorage storage
    class MetricsCollector,LogAggregator,AlertManager monitoring
    class DataPipeline pipeline
```
