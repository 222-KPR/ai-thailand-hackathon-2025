# Architecture Diagrams and API Specifications Index

This document provides a comprehensive index of all architectural diagrams, API specifications, and technical documentation for the AI4Thai Crop Guardian system.

## 📋 Document Overview

### API Specifications
Complete API documentation for all services with request/response examples, error codes, and usage guidelines.

### UML Diagrams
Unified Modeling Language diagrams showing system structure, relationships, and interactions.

### Data Flow Diagrams
Visual representation of how data moves through the system components.

### Swimlane Diagrams
Process flow diagrams showing responsibilities across different system actors.

## 🔌 API Specifications

### [API Gateway Specification](../api/gateway.md)
**Purpose**: Central API gateway endpoints for client applications  
**Coverage**:
- Authentication endpoints (register, login, refresh)
- Disease detection endpoints
- Chat conversation endpoints
- User profile management
- Weather integration
- System health checks
- WebSocket API documentation

**Key Features**:
- JWT authentication flow
- Rate limiting specifications
- Error response formats
- Request/response examples
- Performance benchmarks

### [Vision Service API](../api/vision-service.md)
**Purpose**: Computer vision service for crop disease detection  
**Coverage**:
- Disease detection endpoints
- Crop classification endpoints
- Batch processing capabilities
- Model information endpoints
- Health check endpoints
- Metrics collection

**Key Features**:
- HuggingFace model integration
- Image processing requirements
- Confidence scoring
- Batch analysis support
- Performance characteristics

### [LLM Service API](../api/llm-service.md)
**Purpose**: Language model service for agricultural advisory  
**Coverage**:
- Chat interface endpoints
- Agricultural advice generation
- Translation services
- Text summarization
- Conversation management
- Model information

**Key Features**:
- Thai language support
- Context management
- Multi-model integration
- Conversation persistence
- Real-time response generation

## 🏗️ UML Diagrams

### [Class Diagrams](uml-class-diagram.md)
**Purpose**: System structure and relationships  
**Coverage**:
- Core domain models (User, Diagnosis, Disease, Treatment)
- Chat domain models (Conversation, Message, Context)
- AI service components (VisionService, LLMService, ModelRegistry)
- Infrastructure components (APIGateway, Database, Cache)
- Service interaction patterns

**Key Features**:
- Complete class hierarchies
- Relationship mappings
- Method signatures
- Component interactions

### [Sequence Diagrams](uml-sequence-diagrams.md)
**Purpose**: Process flows and interactions over time  
**Coverage**:
- User registration and authentication flow
- Disease detection process
- Chat conversation flow
- Batch image analysis
- System health checks
- Error handling flows

**Key Features**:
- Timing annotations
- Actor interactions
- Message flows
- Error scenarios
- Background processes

## 📊 Data Flow Diagrams

### [System Data Flow](data-flow.md)
**Purpose**: Data movement through system components  
**Coverage**:
- System-level data flow
- Disease detection data flow
- Chat conversation data flow
- User data flow
- System integration data flow

**Key Features**:
- External data sources
- Internal data processing
- Storage interactions
- Monitoring flows
- Real-time data streams

**Diagrams Included**:
1. **System-Level Flow**: Overall data movement between major components
2. **Disease Detection Flow**: Image processing pipeline from upload to results
3. **Chat Conversation Flow**: Message processing and AI response generation
4. **User Data Flow**: User management and profile data handling
5. **Integration Flow**: External service integration and monitoring

## 🏊‍♂️ Swimlane Diagrams

### [Process Swimlanes](swimlane-diagrams.md)
**Purpose**: Process flows with actor responsibilities  
**Coverage**:
- Disease detection process
- Chat conversation process
- User registration and onboarding
- System deployment and scaling
- Error handling and recovery

**Key Features**:
- Actor responsibilities
- Process timing
- Cross-functional workflows
- Error handling paths
- Background processes

**Diagrams Included**:
1. **Disease Detection Swimlane**: Complete image analysis workflow
2. **Chat Conversation Swimlane**: Real-time messaging process
3. **User Registration Swimlane**: Account creation and verification
4. **Deployment Swimlane**: System deployment and scaling process
5. **Error Recovery Swimlane**: Incident response and recovery

## 📈 Diagram Usage Guide

### For Developers
- **Start with**: [Class Diagrams](uml-class-diagram.md) for system structure
- **Process Understanding**: [Sequence Diagrams](uml-sequence-diagrams.md) for interaction flows
- **API Integration**: [API Specifications](../api/) for implementation details

### For System Architects
- **System Design**: [Data Flow Diagrams](data-flow.md) for architecture overview
- **Process Design**: [Swimlane Diagrams](swimlane-diagrams.md) for workflow planning
- **Component Design**: [Class Diagrams](uml-class-diagram.md) for structure planning

### For DevOps Engineers
- **Deployment**: [Deployment Swimlane](swimlane-diagrams.md#system-deployment-and-scaling-swimlane)
- **Monitoring**: [System Integration Flow](data-flow.md#system-integration-data-flow)
- **Error Handling**: [Error Recovery Swimlane](swimlane-diagrams.md#error-handling-and-recovery-swimlane)

### For Product Managers
- **User Flows**: [User Registration Swimlane](swimlane-diagrams.md#user-registration-and-onboarding-swimlane)
- **Feature Flows**: [Disease Detection Swimlane](swimlane-diagrams.md#disease-detection-process-swimlane)
- **API Capabilities**: [API Gateway Specification](../api/gateway.md)

## 🔍 Quick Reference

### Key System Components
| Component | Purpose | API Spec | Diagrams |
|-----------|---------|----------|----------|
| API Gateway | Central entry point | [Gateway API](../api/gateway.md) | All sequence diagrams |
| Vision Service | Image analysis | [Vision API](../api/vision-service.md) | Disease detection flows |
| LLM Service | Text generation | [LLM API](../api/llm-service.md) | Chat conversation flows |
| Frontend | User interface | - | User interaction flows |
| Database | Data persistence | - | Data flow diagrams |

### Key Processes
| Process | Swimlane | Sequence | Data Flow |
|---------|----------|----------|-----------|
| Disease Detection | ✅ | ✅ | ✅ |
| Chat Conversation | ✅ | ✅ | ✅ |
| User Registration | ✅ | ✅ | ✅ |
| System Deployment | ✅ | - | - |
| Error Recovery | ✅ | ✅ | - |

### Performance Metrics
| Service | Response Time | Throughput | Availability |
|---------|---------------|------------|--------------|
| API Gateway | < 100ms | 1000 req/s | 99.9% |
| Vision Service | < 2s | 40 images/min | 99.5% |
| LLM Service | < 3s | 20 responses/min | 99.5% |
| Database | < 50ms | 5000 queries/s | 99.9% |

## 📚 Additional Resources

### Related Documentation
- [System Architecture Overview](README.md)
- [Microservices Architecture](microservices.md)
- [AI Service Deployment](ai-service-deployment.md)
- [Security Architecture](security.md)
- [Performance Architecture](performance.md)

### External References
- [Mermaid Diagram Syntax](https://mermaid-js.github.io/mermaid/)
- [OpenAPI Specification](https://swagger.io/specification/)
- [UML Notation Guide](https://www.uml-diagrams.org/)
- [Data Flow Diagram Standards](https://en.wikipedia.org/wiki/Data-flow_diagram)

## 🔄 Maintenance

### Document Updates
- **API Specs**: Updated with each service release
- **UML Diagrams**: Updated with architectural changes
- **Data Flow**: Updated with new integrations
- **Swimlanes**: Updated with process changes

### Review Schedule
- **Monthly**: API specification accuracy
- **Quarterly**: Diagram alignment with implementation
- **Bi-annually**: Complete documentation review
- **As-needed**: Emergency updates for critical changes

### Contribution Guidelines
1. Follow existing diagram conventions
2. Use consistent styling and colors
3. Include timing annotations where relevant
4. Update related diagrams when making changes
5. Validate diagram syntax before committing

---

This comprehensive documentation provides a complete view of the AI4Thai Crop Guardian system architecture, enabling effective development, deployment, and maintenance of the platform.
