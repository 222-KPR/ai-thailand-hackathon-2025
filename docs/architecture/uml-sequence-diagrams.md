# UML Sequence Diagrams

## User Registration and Authentication Flow

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant G as API Gateway
    participant A as Auth Service
    participant DB as Database
    participant C as Cache

    Note over U,C: User Registration Flow
    U->>F: Fill registration form
    F->>F: Validate input
    F->>G: POST /auth/register
    G->>A: Validate registration data
    A->>DB: Check if email exists
    DB-->>A: Email availability
    A->>A: Hash password
    A->>DB: Create user record
    DB-->>A: User created
    A->>A: Generate JWT tokens
    A->>C: Store refresh token
    A-->>G: Return tokens & user data
    G-->>F: Registration success
    F-->>U: Show welcome message

    Note over U,C: User Login Flow
    U->>F: Enter credentials
    F->>G: POST /auth/login
    G->>A: Authenticate user
    A->>DB: Find user by email
    DB-->>A: User data
    A->>A: Verify password
    A->>A: Generate JWT tokens
    A->>C: Store refresh token
    A-->>G: Return tokens & user data
    G-->>F: Login success
    F->>F: Store tokens
    F-->>U: Redirect to dashboard

    Note over U,C: Token Refresh Flow
    F->>F: Check token expiry
    F->>G: POST /auth/refresh
    G->>A: Validate refresh token
    A->>C: Check refresh token
    C-->>A: Token valid
    A->>A: Generate new access token
    A-->>G: Return new token
    G-->>F: Token refreshed
    F->>F: Update stored token
```

## Disease Detection Flow

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant G as API Gateway
    participant V as Vision Service
    participant M as Model Registry
    participant IP as Image Processor
    participant IE as Inference Engine
    participant L as LLM Service
    participant DB as Database
    participant C as Cache
    participant Q as Queue Worker

    Note over U,Q: Disease Detection Process
    U->>F: Upload crop image
    F->>F: Validate image format/size
    F->>G: POST /api/diagnose (multipart)
    G->>G: Authenticate request
    G->>V: Forward image analysis request
    
    V->>IP: Process uploaded image
    IP->>IP: Validate image quality
    IP->>IP: Resize and normalize
    IP-->>V: Processed image data
    
    V->>M: Get disease detection model
    M-->>V: Model instance
    V->>IE: Run inference
    IE->>IE: Predict diseases
    IE->>IE: Calculate confidence scores
    IE-->>V: Prediction results
    
    V->>V: Format detection results
    V->>DB: Store diagnosis record
    V-->>G: Return detection results
    
    G->>L: Request treatment advice
    L->>L: Generate recommendations
    L-->>G: Treatment advice
    
    G->>G: Combine results
    G->>C: Cache response
    G->>Q: Enqueue notification job
    G-->>F: Complete diagnosis response
    F-->>U: Display results & recommendations

    Note over Q: Background Processing
    Q->>Q: Process notification job
    Q->>DB: Update user statistics
    Q->>DB: Log diagnosis event
```

## Chat Conversation Flow

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant G as API Gateway
    participant L as LLM Service
    participant CE as Chat Engine
    participant CM as Context Manager
    participant RG as Response Generator
    participant M as Model Registry
    participant DB as Database
    participant C as Cache

    Note over U,C: Chat Conversation Flow
    U->>F: Type message
    F->>G: POST /api/chat/conversations/{id}/messages
    G->>G: Authenticate & validate
    G->>L: Forward chat request
    
    L->>CM: Get conversation context
    CM->>C: Retrieve cached context
    C-->>CM: Context data
    CM-->>L: Conversation context
    
    L->>CE: Process message
    CE->>CE: Analyze message intent
    CE->>CE: Determine response type
    
    CE->>RG: Generate response
    RG->>M: Get appropriate model
    M-->>RG: Model instance
    RG->>RG: Generate AI response
    RG-->>CE: Generated response
    
    CE->>CM: Update context
    CM->>C: Cache updated context
    CE-->>L: Complete response
    
    L->>DB: Store message & response
    L-->>G: Return chat response
    G-->>F: Chat response
    F-->>U: Display AI response

    Note over U,C: WebSocket Real-time Flow
    U->>F: Connect WebSocket
    F->>G: WebSocket connection
    G->>G: Authenticate WebSocket
    G-->>F: Connection established
    
    U->>F: Send message via WebSocket
    F->>G: WebSocket message
    G->>L: Process message
    L-->>G: Response ready
    G->>F: WebSocket response
    F-->>U: Real-time response display
```

## Batch Image Analysis Flow

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant G as API Gateway
    participant V as Vision Service
    participant Q as Queue Worker
    participant JP as Job Processor
    participant DB as Database
    participant C as Cache
    participant N as Notification Service

    Note over U,N: Batch Analysis Flow
    U->>F: Select multiple images
    F->>F: Validate batch size & formats
    F->>G: POST /api/diagnose/batch
    G->>G: Authenticate request
    G->>V: Forward batch request
    
    V->>V: Validate batch parameters
    V->>Q: Enqueue batch job
    Q->>Q: Create job record
    Q-->>V: Job ID
    V-->>G: Batch job accepted
    G-->>F: Job ID & status
    F-->>U: Show processing status

    Note over Q,N: Background Batch Processing
    Q->>JP: Process batch job
    JP->>JP: Split into individual tasks
    
    loop For each image
        JP->>V: Process single image
        V->>V: Run disease detection
        V-->>JP: Individual result
        JP->>DB: Store individual diagnosis
    end
    
    JP->>JP: Aggregate results
    JP->>DB: Update batch job status
    JP->>C: Cache batch results
    JP->>N: Send completion notification
    N->>N: Send email/push notification
    N-->>U: Batch complete notification

    Note over U,C: Result Retrieval
    U->>F: Check batch status
    F->>G: GET /api/diagnose/batch/{job_id}
    G->>C: Check cached results
    C-->>G: Batch results
    G-->>F: Complete results
    F-->>U: Display batch analysis
```

## System Health Check Flow

```mermaid
sequenceDiagram
    participant M as Monitoring System
    participant G as API Gateway
    participant V as Vision Service
    participant L as LLM Service
    participant DB as Database
    participant C as Cache
    participant A as Alert Manager

    Note over M,A: Health Check Cycle
    M->>G: GET /health
    G->>G: Check internal health
    G->>V: GET /health
    V->>V: Check model status
    V->>V: Check GPU availability
    V-->>G: Vision service health
    
    G->>L: GET /health
    L->>L: Check model status
    L->>L: Check context manager
    L-->>G: LLM service health
    
    G->>DB: Test connection
    DB-->>G: Database health
    
    G->>C: Test connection
    C-->>G: Cache health
    
    G->>G: Aggregate health status
    G-->>M: Overall system health

    alt System Healthy
        M->>M: Record healthy status
        M->>M: Update metrics
    else System Unhealthy
        M->>A: Trigger alert
        A->>A: Send notifications
        A->>A: Escalate if needed
    end

    Note over M,A: Detailed Health Check
    M->>G: GET /health/detailed
    G->>V: GET /health/detailed
    V-->>G: Detailed vision metrics
    G->>L: GET /health/detailed
    L-->>G: Detailed LLM metrics
    G->>G: Compile detailed report
    G-->>M: Comprehensive health data
    M->>M: Update dashboards
```

## User Profile Update Flow

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant G as API Gateway
    participant A as Auth Service
    participant UR as User Repository
    participant DB as Database
    participant C as Cache
    participant Q as Queue Worker

    Note over U,Q: Profile Update Flow
    U->>F: Edit profile information
    F->>F: Validate form data
    F->>G: PUT /api/profile
    G->>A: Validate JWT token
    A-->>G: Token valid
    
    G->>G: Validate update data
    G->>UR: Update user profile
    UR->>DB: Execute update query
    DB-->>UR: Update successful
    UR-->>G: Updated user data
    
    G->>C: Invalidate cached profile
    G->>Q: Enqueue profile sync job
    G-->>F: Profile updated
    F-->>U: Show success message

    Note over Q: Background Processing
    Q->>Q: Process profile sync
    Q->>DB: Update related records
    Q->>C: Update cached data
    Q->>Q: Log profile change
```

## Error Handling Flow

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant G as API Gateway
    participant S as Service
    participant L as Logging Service
    participant M as Metrics Service
    participant A as Alert Manager

    Note over U,A: Error Handling Flow
    U->>F: Perform action
    F->>G: API request
    G->>S: Forward request
    S->>S: Process request
    S-->>G: Error response
    
    G->>L: Log error details
    G->>M: Record error metric
    G->>G: Format error response
    G-->>F: Standardized error
    
    F->>F: Handle error gracefully
    F-->>U: User-friendly error message

    alt Critical Error
        G->>A: Trigger alert
        A->>A: Send immediate notification
        A->>A: Page on-call engineer
    else Recoverable Error
        G->>G: Log for analysis
        G->>M: Update error rate metrics
    end

    Note over U,A: Retry Logic
    F->>F: Implement retry logic
    F->>G: Retry request
    G->>S: Forward retry
    S-->>G: Success response
    G-->>F: Successful response
    F-->>U: Show success result
```
