# Swimlane Diagrams

## Disease Detection Process Swimlane

```mermaid
flowchart TD
    subgraph "👤 User"
        A1[Take crop photo]
        A2[Open app]
        A3[Upload image]
        A4[Select crop type]
        A5[Submit for analysis]
        A6[View results]
        A7[Read recommendations]
    end
    
    subgraph "📱 Frontend"
        B1[Load camera interface]
        B2[Validate image format]
        B3[Check image size]
        B4[Show upload progress]
        B5[Display loading spinner]
        B6[Render results UI]
        B7[Show treatment advice]
    end
    
    subgraph "🔌 API Gateway"
        C1[Receive upload request]
        C2[Authenticate user]
        C3[Validate request data]
        C4[Forward to Vision Service]
        C5[Receive analysis results]
        C6[Request treatment advice]
        C7[Combine responses]
        C8[Return to frontend]
    end
    
    subgraph "👁️ Vision Service"
        D1[Receive image data]
        D2[Validate image quality]
        D3[Preprocess image]
        D4[Load AI models]
        D5[Run crop classification]
        D6[Run disease detection]
        D7[Calculate confidence scores]
        D8[Format results]
        D9[Store diagnosis]
    end
    
    subgraph "🧠 LLM Service"
        E1[Receive advice request]
        E2[Load agricultural model]
        E3[Generate treatment plan]
        E4[Add prevention tips]
        E5[Format advice response]
    end
    
    subgraph "🗄️ Database"
        F1[Store user data]
        F2[Store diagnosis record]
        F3[Update user statistics]
        F4[Log analysis event]
    end
    
    subgraph "⚡ Queue Worker"
        G1[Process notification job]
        G2[Send success notification]
        G3[Update analytics]
    end
    
    %% Flow connections
    A1 --> A2
    A2 --> B1
    A3 --> B2
    B2 --> B3
    A4 --> A5
    A5 --> C1
    
    C1 --> C2
    C2 --> C3
    C3 --> C4
    C4 --> D1
    
    D1 --> D2
    D2 --> D3
    D3 --> D4
    D4 --> D5
    D5 --> D6
    D6 --> D7
    D7 --> D8
    D8 --> D9
    D9 --> F2
    
    D8 --> C5
    C5 --> C6
    C6 --> E1
    E1 --> E2
    E2 --> E3
    E3 --> E4
    E4 --> E5
    E5 --> C7
    
    C7 --> C8
    C8 --> B6
    B6 --> A6
    A6 --> B7
    B7 --> A7
    
    %% Background processes
    C8 --> G1
    G1 --> G2
    G2 --> G3
    F2 --> F3
    F3 --> F4
    
    %% Timing annotations
    A1 -.->|"0s"| A2
    A5 -.->|"2s"| C1
    D9 -.->|"1.8s"| C5
    E5 -.->|"2.1s"| C7
    C8 -.->|"4.2s"| B6
    
    %% Styling
    classDef user fill:#e3f2fd
    classDef frontend fill:#f3e5f5
    classDef gateway fill:#e1f5fe
    classDef vision fill:#e8f5e8
    classDef llm fill:#c8e6c9
    classDef database fill:#fce4ec
    classDef queue fill:#fff3e0
    
    class A1,A2,A3,A4,A5,A6,A7 user
    class B1,B2,B3,B4,B5,B6,B7 frontend
    class C1,C2,C3,C4,C5,C6,C7,C8 gateway
    class D1,D2,D3,D4,D5,D6,D7,D8,D9 vision
    class E1,E2,E3,E4,E5 llm
    class F1,F2,F3,F4 database
    class G1,G2,G3 queue
```

## Chat Conversation Process Swimlane

```mermaid
flowchart TD
    subgraph "👤 User"
        A1[Open chat interface]
        A2[Type question about crops]
        A3[Send message]
        A4[Wait for response]
        A5[Read AI response]
        A6[Ask follow-up question]
        A7[Continue conversation]
    end
    
    subgraph "📱 Frontend"
        B1[Load chat UI]
        B2[Establish WebSocket]
        B3[Validate message input]
        B4[Send via WebSocket]
        B5[Show typing indicator]
        B6[Receive response]
        B7[Render message bubble]
        B8[Update conversation history]
    end
    
    subgraph "🔌 API Gateway"
        C1[Handle WebSocket connection]
        C2[Authenticate WebSocket]
        C3[Receive chat message]
        C4[Route to LLM Service]
        C5[Receive AI response]
        C6[Send via WebSocket]
        C7[Log conversation event]
    end
    
    subgraph "🧠 LLM Service"
        D1[Receive message request]
        D2[Load conversation context]
        D3[Analyze message intent]
        D4[Determine response type]
        D5[Load appropriate model]
        D6[Generate AI response]
        D7[Post-process response]
        D8[Update conversation context]
    end
    
    subgraph "🧠 Context Manager"
        E1[Retrieve conversation history]
        E2[Load user preferences]
        E3[Get agricultural context]
        E4[Update context with new message]
        E5[Cache updated context]
    end
    
    subgraph "🤖 AI Models"
        F1[Agricultural Advisor Model]
        F2[Thai Language Model]
        F3[Translation Model]
        F4[Generate contextual response]
    end
    
    subgraph "💬 Conversation Store"
        G1[Store user message]
        G2[Store AI response]
        G3[Update conversation metadata]
        G4[Index for search]
    end
    
    subgraph "⚡ Cache"
        H1[Cache conversation context]
        H2[Cache user session]
        H3[Cache frequent responses]
    end
    
    %% Main flow
    A1 --> B1
    B1 --> B2
    A2 --> A3
    A3 --> B3
    B3 --> B4
    B4 --> C3
    
    C1 --> C2
    C3 --> C4
    C4 --> D1
    
    D1 --> D2
    D2 --> E1
    E1 --> E2
    E2 --> E3
    E3 --> D3
    
    D3 --> D4
    D4 --> D5
    D5 --> F1
    F1 --> F2
    F2 --> F4
    F4 --> D6
    
    D6 --> D7
    D7 --> D8
    D8 --> E4
    E4 --> E5
    E5 --> H1
    
    D7 --> C5
    C5 --> C6
    C6 --> B6
    B6 --> B7
    B7 --> A5
    
    %% Storage operations
    D1 --> G1
    D7 --> G2
    G2 --> G3
    G3 --> G4
    
    E5 --> H2
    D7 --> H3
    
    %% Follow-up flow
    A5 --> A6
    A6 --> A7
    A7 --> B3
    
    %% Background processes
    C6 --> C7
    
    %% Timing annotations
    A3 -.->|"0s"| B4
    C3 -.->|"0.1s"| D1
    D2 -.->|"0.2s"| E3
    F4 -.->|"2.0s"| D6
    C6 -.->|"2.3s"| B6
    
    %% Real-time indicators
    B4 -.->|"WebSocket"| C3
    C6 -.->|"WebSocket"| B6
    B5 -.->|"Real-time"| A4
    
    %% Styling
    classDef user fill:#e3f2fd
    classDef frontend fill:#f3e5f5
    classDef gateway fill:#e1f5fe
    classDef llm fill:#e8f5e8
    classDef context fill:#c8e6c9
    classDef models fill:#a5d6a7
    classDef storage fill:#fce4ec
    classDef cache fill:#fff3e0
    
    class A1,A2,A3,A4,A5,A6,A7 user
    class B1,B2,B3,B4,B5,B6,B7,B8 frontend
    class C1,C2,C3,C4,C5,C6,C7 gateway
    class D1,D2,D3,D4,D5,D6,D7,D8 llm
    class E1,E2,E3,E4,E5 context
    class F1,F2,F3,F4 models
    class G1,G2,G3,G4 storage
    class H1,H2,H3 cache
```

## User Registration and Onboarding Swimlane

```mermaid
flowchart TD
    subgraph "👤 New User"
        A1[Download app]
        A2[Open registration]
        A3[Fill personal info]
        A4[Add farm details]
        A5[Set preferences]
        A6[Submit registration]
        A7[Check email]
        A8[Verify account]
        A9[Complete onboarding]
    end
    
    subgraph "📱 Frontend"
        B1[Show welcome screen]
        B2[Display registration form]
        B3[Validate form fields]
        B4[Show progress indicator]
        B5[Submit registration data]
        B6[Show verification message]
        B7[Handle email verification]
        B8[Show onboarding tutorial]
    end
    
    subgraph "🔌 API Gateway"
        C1[Receive registration request]
        C2[Validate request data]
        C3[Check rate limits]
        C4[Forward to User Service]
        C5[Receive registration response]
        C6[Trigger verification email]
        C7[Handle verification callback]
        C8[Return success response]
    end
    
    subgraph "👤 User Service"
        D1[Validate user data]
        D2[Check email uniqueness]
        D3[Hash password]
        D4[Create user record]
        D5[Generate verification token]
        D6[Store user preferences]
        D7[Verify email token]
        D8[Activate user account]
    end
    
    subgraph "🗄️ Database"
        E1[Check existing users]
        E2[Insert new user record]
        E3[Store farm information]
        E4[Store preferences]
        E5[Update verification status]
        E6[Log registration event]
    end
    
    subgraph "📧 Email Service"
        F1[Generate verification email]
        F2[Send welcome email]
        F3[Queue email delivery]
        F4[Track email status]
    end
    
    subgraph "📊 Analytics"
        G1[Track registration start]
        G2[Track form completion]
        G3[Track verification success]
        G4[Update user metrics]
    end
    
    subgraph "🔐 Security"
        H1[Generate secure token]
        H2[Set token expiration]
        H3[Validate token on verification]
        H4[Log security events]
    end
    
    %% Main registration flow
    A1 --> B1
    B1 --> A2
    A2 --> B2
    A3 --> A4
    A4 --> A5
    A5 --> B3
    B3 --> B4
    A6 --> B5
    B5 --> C1
    
    C1 --> C2
    C2 --> C3
    C3 --> C4
    C4 --> D1
    
    D1 --> D2
    D2 --> E1
    E1 --> D3
    D3 --> D4
    D4 --> E2
    E2 --> E3
    E3 --> D5
    
    D5 --> H1
    H1 --> H2
    H2 --> D6
    D6 --> E4
    E4 --> C5
    
    C5 --> C6
    C6 --> F1
    F1 --> F2
    F2 --> F3
    F3 --> B6
    B6 --> A7
    
    %% Email verification flow
    A7 --> A8
    A8 --> B7
    B7 --> C7
    C7 --> D7
    D7 --> H3
    H3 --> D8
    D8 --> E5
    E5 --> C8
    C8 --> B8
    B8 --> A9
    
    %% Analytics tracking
    A2 --> G1
    A6 --> G2
    A8 --> G3
    A9 --> G4
    
    %% Security logging
    D4 --> H4
    D8 --> H4
    
    %% Background processes
    E2 --> E6
    F3 --> F4
    
    %% Error handling paths
    D2 -.->|"Email exists"| C5
    H3 -.->|"Invalid token"| C7
    F4 -.->|"Email failed"| F2
    
    %% Timing annotations
    A6 -.->|"0s"| B5
    C1 -.->|"0.1s"| D1
    D4 -.->|"0.5s"| E2
    F2 -.->|"1.0s"| B6
    A8 -.->|"varies"| B7
    
    %% Styling
    classDef user fill:#e3f2fd
    classDef frontend fill:#f3e5f5
    classDef gateway fill:#e1f5fe
    classDef service fill:#e8f5e8
    classDef database fill:#fce4ec
    classDef email fill:#fff3e0
    classDef analytics fill:#f1f8e9
    classDef security fill:#ffebee
    
    class A1,A2,A3,A4,A5,A6,A7,A8,A9 user
    class B1,B2,B3,B4,B5,B6,B7,B8 frontend
    class C1,C2,C3,C4,C5,C6,C7,C8 gateway
    class D1,D2,D3,D4,D5,D6,D7,D8 service
    class E1,E2,E3,E4,E5,E6 database
    class F1,F2,F3,F4 email
    class G1,G2,G3,G4 analytics
    class H1,H2,H3,H4 security
```

## System Deployment and Scaling Swimlane

```mermaid
flowchart TD
    subgraph "👨‍💼 DevOps Engineer"
        A1[Review deployment request]
        A2[Check system resources]
        A3[Plan deployment strategy]
        A4[Execute deployment script]
        A5[Monitor deployment progress]
        A6[Verify system health]
        A7[Update documentation]
    end
    
    subgraph "🔧 CI/CD Pipeline"
        B1[Trigger deployment pipeline]
        B2[Run automated tests]
        B3[Build Docker images]
        B4[Push to registry]
        B5[Deploy to staging]
        B6[Run integration tests]
        B7[Deploy to production]
        B8[Update service discovery]
    end
    
    subgraph "☸️ Kubernetes Cluster"
        C1[Receive deployment manifest]
        C2[Validate resource requirements]
        C3[Schedule pods]
        C4[Pull container images]
        C5[Start new pods]
        C6[Update load balancer]
        C7[Terminate old pods]
        C8[Update ingress rules]
    end
    
    subgraph "🤖 AI Services"
        D1[Download HuggingFace models]
        D2[Initialize model registry]
        D3[Load AI models into memory]
        D4[Start health check endpoints]
        D5[Register with service mesh]
        D6[Begin processing requests]
    end
    
    subgraph "🗄️ Database"
        E1[Run database migrations]
        E2[Update schema]
        E3[Reindex tables]
        E4[Update statistics]
        E5[Backup current state]
        E6[Verify data integrity]
    end
    
    subgraph "📊 Monitoring"
        F1[Deploy monitoring agents]
        F2[Update Prometheus config]
        F3[Create Grafana dashboards]
        F4[Set up alerting rules]
        F5[Test alert channels]
        F6[Monitor deployment metrics]
    end
    
    subgraph "🔍 Health Checks"
        G1[Check service endpoints]
        G2[Verify database connections]
        G3[Test AI model loading]
        G4[Validate external APIs]
        G5[Run smoke tests]
        G6[Generate health report]
    end
    
    subgraph "⚖️ Load Balancer"
        H1[Update backend pools]
        H2[Configure health checks]
        H3[Adjust traffic weights]
        H4[Enable new services]
        H5[Drain old services]
        H6[Update SSL certificates]
    end
    
    %% Main deployment flow
    A1 --> A2
    A2 --> A3
    A3 --> B1
    A4 --> B1
    
    B1 --> B2
    B2 --> B3
    B3 --> B4
    B4 --> B5
    B5 --> B6
    B6 --> B7
    B7 --> C1
    
    C1 --> C2
    C2 --> C3
    C3 --> C4
    C4 --> C5
    C5 --> D1
    
    D1 --> D2
    D2 --> D3
    D3 --> D4
    D4 --> D5
    D5 --> D6
    
    %% Database operations
    B7 --> E1
    E1 --> E2
    E2 --> E3
    E3 --> E4
    E4 --> E5
    E5 --> E6
    
    %% Monitoring setup
    C5 --> F1
    F1 --> F2
    F2 --> F3
    F3 --> F4
    F4 --> F5
    F5 --> F6
    
    %% Health verification
    D6 --> G1
    G1 --> G2
    G2 --> G3
    G3 --> G4
    G4 --> G5
    G5 --> G6
    
    %% Load balancer updates
    C5 --> H1
    H1 --> H2
    H2 --> H3
    H3 --> H4
    H4 --> C6
    C6 --> C7
    C7 --> H5
    
    %% Final verification
    G6 --> A5
    F6 --> A5
    A5 --> A6
    A6 --> A7
    
    %% Service discovery updates
    D5 --> B8
    H4 --> B8
    B8 --> C8
    
    %% SSL certificate updates
    H4 --> H6
    
    %% Timing annotations
    A4 -.->|"0s"| B1
    B7 -.->|"5min"| C1
    D3 -.->|"2min"| D4
    G5 -.->|"1min"| G6
    A6 -.->|"15min total"| A7
    
    %% Rollback paths (dotted)
    G6 -.->|"Health check fails"| C7
    F6 -.->|"Metrics anomaly"| H5
    
    %% Styling
    classDef devops fill:#e3f2fd
    classDef pipeline fill:#f3e5f5
    classDef kubernetes fill:#e1f5fe
    classDef aiservice fill:#e8f5e8
    classDef database fill:#fce4ec
    classDef monitoring fill:#fff3e0
    classDef health fill:#f1f8e9
    classDef loadbalancer fill:#ffebee
    
    class A1,A2,A3,A4,A5,A6,A7 devops
    class B1,B2,B3,B4,B5,B6,B7,B8 pipeline
    class C1,C2,C3,C4,C5,C6,C7,C8 kubernetes
    class D1,D2,D3,D4,D5,D6 aiservice
    class E1,E2,E3,E4,E5,E6 database
    class F1,F2,F3,F4,F5,F6 monitoring
    class G1,G2,G3,G4,G5,G6 health
    class H1,H2,H3,H4,H5,H6 loadbalancer
```

## Error Handling and Recovery Swimlane

```mermaid
flowchart TD
    subgraph "🚨 Error Detection"
        A1[Monitor system metrics]
        A2[Detect anomaly]
        A3[Classify error severity]
        A4[Trigger alert]
        A5[Log error details]
    end
    
    subgraph "📊 Monitoring System"
        B1[Collect metrics]
        B2[Analyze patterns]
        B3[Check thresholds]
        B4[Generate alerts]
        B5[Update dashboards]
        B6[Send notifications]
    end
    
    subgraph "👨‍💼 On-Call Engineer"
        C1[Receive alert]
        C2[Assess severity]
        C3[Check runbooks]
        C4[Execute recovery steps]
        C5[Monitor recovery]
        C6[Update incident log]
        C7[Conduct post-mortem]
    end
    
    subgraph "🔧 Automated Recovery"
        D1[Detect service failure]
        D2[Attempt auto-restart]
        D3[Check health status]
        D4[Scale resources]
        D5[Failover to backup]
        D6[Update load balancer]
        D7[Log recovery actions]
    end
    
    subgraph "🏥 Health Check System"
        E1[Run health checks]
        E2[Test service endpoints]
        E3[Verify database connections]
        E4[Check AI model status]
        E5[Validate external APIs]
        E6[Report health status]
    end
    
    subgraph "👤 User Experience"
        F1[Experience service degradation]
        F2[See error message]
        F3[Retry operation]
        F4[Contact support]
        F5[Receive service update]
        F6[Resume normal usage]
    end
    
    subgraph "📞 Support Team"
        G1[Receive user reports]
        G2[Triage issues]
        G3[Escalate to engineering]
        G4[Communicate with users]
        G5[Update status page]
        G6[Follow up with users]
    end
    
    subgraph "🔄 Recovery Actions"
        H1[Restart failed services]
        H2[Clear cache corruption]
        H3[Restore from backup]
        H4[Update configurations]
        H5[Apply hotfixes]
        H6[Verify system stability]
    end
    
    %% Error detection flow
    A1 --> B1
    B1 --> B2
    B2 --> B3
    B3 --> A2
    A2 --> A3
    A3 --> A4
    A4 --> B4
    B4 --> B6
    
    %% Alert handling
    B6 --> C1
    C1 --> C2
    C2 --> C3
    C3 --> C4
    
    %% Automated recovery
    A2 --> D1
    D1 --> D2
    D2 --> D3
    D3 --> E1
    E1 --> E2
    E2 --> E3
    E3 --> E4
    E4 --> E5
    E5 --> E6
    
    %% Recovery decision
    E6 --> D4
    D4 --> D5
    D5 --> D6
    D6 --> D7
    
    %% Manual intervention
    C4 --> H1
    H1 --> H2
    H2 --> H3
    H3 --> H4
    H4 --> H5
    H5 --> H6
    
    %% User impact
    A2 --> F1
    F1 --> F2
    F2 --> F3
    F3 --> F4
    F4 --> G1
    
    %% Support handling
    G1 --> G2
    G2 --> G3
    G3 --> C1
    G2 --> G4
    G4 --> G5
    
    %% Recovery verification
    H6 --> C5
    D7 --> C5
    C5 --> C6
    C6 --> G6
    G6 --> F5
    F5 --> F6
    
    %% Post-incident
    C6 --> C7
    C7 --> A5
    
    %% Continuous monitoring
    H6 --> A1
    F6 --> A1
    
    %% Timing annotations
    A2 -.->|"0s"| A4
    B6 -.->|"30s"| C1
    D2 -.->|"1min"| D3
    C4 -.->|"5min"| H1
    H6 -.->|"15min"| C5
    
    %% Severity paths
    A3 -.->|"Critical"| D1
    A3 -.->|"High"| C1
    A3 -.->|"Medium"| G2
    
    %% Styling
    classDef detection fill:#ffebee
    classDef monitoring fill:#fff3e0
    classDef oncall fill:#e3f2fd
    classDef automated fill:#e8f5e8
    classDef health fill:#f1f8e9
    classDef user fill:#fce4ec
    classDef support fill:#f3e5f5
    classDef recovery fill:#e1f5fe
    
    class A1,A2,A3,A4,A5 detection
    class B1,B2,B3,B4,B5,B6 monitoring
    class C1,C2,C3,C4,C5,C6,C7 oncall
    class D1,D2,D3,D4,D5,D6,D7 automated
    class E1,E2,E3,E4,E5,E6 health
    class F1,F2,F3,F4,F5,F6 user
    class G1,G2,G3,G4,G5,G6 support
    class H1,H2,H3,H4,H5,H6 recovery
```
