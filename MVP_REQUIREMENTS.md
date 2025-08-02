# AI4Thai Crop Guardian - MVP Requirements

## 🎯 MVP Scope Definition

### Core Value Proposition
Enable Thai farmers to instantly diagnose crop diseases using smartphone photos and receive actionable treatment advice in their native language.

### Success Criteria
- **95%+ disease identification accuracy** for supported crops
- **<3 second response time** for diagnosis
- **Thai/English bilingual** interface
- **Offline-capable** PWA for rural connectivity
- **1000+ farmers** using the platform within 3 months

## 📱 Functional Requirements

### 1. User Authentication & Onboarding
- **Simple Registration**: Phone number or email
- **Language Selection**: Thai/English preference
- **Farm Profile**: Basic farm info (location, crop types)
- **Tutorial**: Interactive guide for first-time users

### 2. Disease Detection Core
```rust
// Core detection flow
struct DiagnosisRequest {
    image: Vec<u8>,          // Raw image data
    crop_type: CropType,     // Rice, Cassava, Durian, etc.
    location: Option<GeoLocation>,
    timestamp: DateTime<Utc>,
}

struct DiagnosisResponse {
    disease: String,          // Disease name in Thai/English
    confidence: f32,          // 0.0 - 1.0
    severity: Severity,       // Low, Medium, High
    treatment_plan: Vec<TreatmentStep>,
    estimated_cost: Option<f32>,
}
```

**Supported Crops (MVP)**:
- Rice (ข้าว)
- Cassava (มันสำปะหลัง) 
- Durian (ทุเรียน)
- Mango (มะม่วง)
- Rubber (ยางพารา)

**Supported Diseases**:
- Rice: Blast, Brown spot, Bacterial blight
- Cassava: Mosaic virus, Root rot
- Durian: Fusarium wilt, Anthracnose
- Mango: Anthracnose, Powdery mildew
- Rubber: Leaf blight, Tapping panel dryness

### 3. Chat Interface
- **Text Input**: Thai/English typing support
- **Image Upload**: Camera capture + gallery selection
- **Voice Input**: ASR for hands-free operation (Post-MVP)
- **Voice Output**: TTS for treatment instructions (Post-MVP)
- **History**: Previous diagnoses and conversations

### 4. AI Advisory System
- **Treatment Recommendations**: Organic and chemical options
- **Timing Guidance**: When to apply treatments
- **Cost Estimation**: Treatment expenses in Thai Baht
- **Prevention Tips**: Future disease prevention advice
- **Weather Awareness**: Rain/humidity impact on treatments

### 5. PWA Features
- **Offline Mode**: Core functionality without internet
- **Home Screen Install**: Native app-like experience
- **Push Notifications**: Treatment reminders
- **Camera Access**: Direct photo capture
- **Fast Loading**: <2 second initial load

## ⚙️ Technical Requirements

### Backend API Specifications

#### Authentication Endpoints
```
POST /api/v1/auth/register
POST /api/v1/auth/login
POST /api/v1/auth/refresh
DELETE /api/v1/auth/logout
```

#### Core Diagnosis API
```
POST /api/v1/diagnose
GET /api/v1/diagnose/{id}
GET /api/v1/diagnose/history
```

#### User Management
```
GET /api/v1/user/profile
PUT /api/v1/user/profile
GET /api/v1/user/stats
```

### Database Schema
```sql
-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR UNIQUE,
    phone VARCHAR,
    language VARCHAR(2) DEFAULT 'th',
    farm_location POINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Diagnoses table
CREATE TABLE diagnoses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    crop_type VARCHAR NOT NULL,
    disease_detected VARCHAR,
    confidence FLOAT,
    severity VARCHAR,
    image_url VARCHAR,
    treatment_plan JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Chat conversations
CREATE TABLE conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    messages JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Performance Requirements
- **API Response Time**: <3 seconds for diagnosis
- **Image Processing**: <1 second preprocessing
- **Database Queries**: <100ms average
- **Frontend Load**: <2 seconds initial load
- **Offline Sync**: <5 seconds when connection restored

### Infrastructure Requirements
- **Availability**: 99.9% uptime
- **Scalability**: Support 10,000 concurrent users
- **Security**: TLS 1.3, JWT authentication
- **Monitoring**: Real-time error tracking and metrics
- **Backup**: Daily database backups with 30-day retention

## 🎨 UI/UX Requirements

### Design Principles
- **Farmer-First**: Intuitive for low-tech users
- **Mobile-Optimized**: Touch-friendly interface
- **Accessible**: High contrast, large fonts
- **Fast**: Minimal taps to complete diagnosis

### Key Screens

#### 1. Home/Chat Screen
```
┌─────────────────────────────┐
│ 🌾 AI4Thai Crop Guardian   │
├─────────────────────────────┤
│                             │
│ 💬 สวัสดีครับ! มีปัญหา      │
│    เรื่องพืชผลอะไรไหม?       │
│                             │
│ 📷 [Take Photo]             │
│ 🎤 [Voice Input]            │
│ ⌨️  [Type Message]          │
│                             │
├─────────────────────────────┤
│ 📝 Recent Diagnoses         │
│ 📊 Farm Dashboard           │
└─────────────────────────────┘
```

#### 2. Camera Interface
```
┌─────────────────────────────┐
│ 📷 Take Photo of Affected  │
│     Plant                   │
├─────────────────────────────┤
│                             │
│    [Camera Viewfinder]      │
│                             │
│ 💡 Tips:                    │
│ • Focus on affected leaves  │
│ • Good lighting             │
│ • Multiple angles           │
│                             │
├─────────────────────────────┤
│ [Capture] [Gallery] [Cancel]│
└─────────────────────────────┘
```

#### 3. Diagnosis Results
```
┌─────────────────────────────┐
│ 🔍 Diagnosis Results        │
├─────────────────────────────┤
│ 🦠 Rice Blast Disease       │
│ 📊 Confidence: 94%          │
│ ⚠️  Severity: Medium         │
│                             │
│ 💊 Treatment Plan:          │
│ 1. Apply fungicide spray    │
│ 2. Remove affected leaves   │
│ 3. Improve ventilation      │
│                             │
│ 💰 Est. Cost: ฿150-250      │
│                             │
├─────────────────────────────┤
│ [Get Details] [Ask Question]│
└─────────────────────────────┘
```

### Responsive Design
- **Mobile First**: Optimized for 375px+ width
- **Tablet Support**: Adaptive layout for larger screens
- **Desktop**: Full-featured web interface
- **Touch Targets**: Minimum 44px tap areas

## 🧪 Testing Requirements

### Unit Testing
- **Backend**: 90%+ code coverage for core logic
- **Frontend**: Component and integration tests
- **API**: Comprehensive endpoint testing
- **Database**: Migration and query testing

### Integration Testing
- **Computer Vision**: Accuracy validation with test dataset
- **API Integration**: External service mocking
- **End-to-End**: User journey automation
- **Performance**: Load testing under various conditions

### Quality Assurance
- **Manual Testing**: Device and browser compatibility
- **User Testing**: Farmer feedback sessions
- **Security Testing**: Penetration testing and vulnerability scans
- **Accessibility**: WCAG 2.1 compliance

## 🚀 Deployment Requirements

### Development Environment
```bash
# Local development stack
docker-compose up -d postgres redis
cargo run --bin backend
trunk serve frontend/
```

### Staging Environment
- **Containerized**: Docker-based deployment
- **Database**: PostgreSQL with test data
- **Monitoring**: Basic logging and metrics
- **Testing**: Automated test execution

### Production Environment
- **Container Orchestration**: Kubernetes cluster
- **Load Balancing**: NGINX ingress controller
- **Database**: Managed PostgreSQL with replication
- **CDN**: Static asset delivery
- **Monitoring**: Prometheus + Grafana stack
- **Logging**: Structured logging with log aggregation

### CI/CD Pipeline
```yaml
# GitHub Actions workflow
- Code Quality: Rust fmt, clippy, tests
- Security: Dependency audit, container scanning
- Build: Multi-stage Docker builds
- Deploy: Automated staging deployment
- Testing: Integration and E2E tests
- Production: Manual approval + deployment
```

## 📊 Success Metrics & KPIs

### Technical Metrics
- **Diagnosis Accuracy**: >95% on test dataset
- **Response Time**: <3 seconds average
- **Uptime**: 99.9% availability
- **Error Rate**: <0.1% API errors

### User Metrics
- **Adoption**: 1000+ registered farmers
- **Engagement**: 3+ diagnoses per user/month
- **Retention**: 70%+ monthly active users
- **Satisfaction**: 4.5+ app store rating

### Business Metrics
- **Conversion**: 10%+ free to premium conversion
- **Impact**: 20% yield improvement for active users
- **Support**: <24 hour response time
- **Growth**: 20% month-over-month user growth

## 🔄 MVP Development Timeline

### Week 1-2: Foundation
- [ ] Project setup and workspace configuration
- [ ] Database schema and migrations
- [ ] Basic API structure with authentication
- [ ] Frontend PWA shell

### Week 3-4: Core Features
- [ ] Computer vision integration
- [ ] Disease detection API
- [ ] Chat interface implementation
- [ ] Image upload and processing

### Week 5-6: Integration & Polish
- [ ] LLM advisory system integration
- [ ] Thai language support
- [ ] Offline functionality
- [ ] Performance optimization

### Week 7-8: Testing & Deployment
- [ ] Comprehensive testing suite
- [ ] User acceptance testing
- [ ] Production deployment
- [ ] Monitoring and observability

### Week 9-10: Launch & Iteration
- [ ] Beta testing with real farmers
- [ ] Feedback collection and analysis
- [ ] Bug fixes and improvements
- [ ] Documentation and onboarding

## 🎯 Definition of Done

### MVP is complete when:
- ✅ All core features are implemented and tested
- ✅ 95%+ disease detection accuracy achieved
- ✅ Thai/English bilingual interface working
- ✅ PWA passes Lighthouse audit (>90 score)
- ✅ Production deployment is stable
- ✅ 100+ farmers successfully use the platform
- ✅ Performance meets all specified requirements
- ✅ Security audit passes without critical issues