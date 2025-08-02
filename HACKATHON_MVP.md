# AI4Thai Crop Guardian - 40-Hour Hackathon MVP

## ⏰ Time Constraints & Resource Limits

**Total Development Time**: 40 hours
- **Backend**: 24 hours
- **Frontend**: 18 hours  
- **Model Constraint**: <16GB memory
- **Deployment**: Docker Compose only

## 🎯 Simplified MVP Scope

### Core Demo Features (Minimum Viable)
1. **Single Disease Detection**: Focus on Rice Blast only
2. **Basic Chat Interface**: Text + image upload
3. **Simple AI Response**: Disease identification + basic advice
4. **Mock External APIs**: Simulated LLM/TTS responses

### Success Criteria
- **Working demo** with real image classification
- **End-to-end flow** from photo to diagnosis
- **Thai language** basic support
- **Deployable** via Docker Compose

## 🏗️ Simplified Technical Stack

### Backend (24 hours)
- **Framework**: Axum (Rust) - minimal setup
- **Database**: SQLite (no PostgreSQL setup time)
- **Vision Model**: Local lightweight model OR external API
- **Queue**: In-memory (no Redis for MVP)
- **External APIs**: Mock responses with real API structure

### Frontend (18 hours)  
- **Framework**: Vanilla HTML/CSS/JS (no Rust WASM complexity)
- **PWA**: Basic service worker for demo
- **UI**: Simple Bootstrap-based responsive design
- **Camera**: HTML5 camera API

### Deployment
- **Container**: Single Docker Compose with 2 services
- **No Kubernetes**: Simple docker-compose.yml
- **No CI/CD**: Manual deployment only

## 📱 Hackathon Feature Set

### 1. Image Upload & Classification (Core)
```
Input: Rice plant image
Process: Disease detection model
Output: Disease name + confidence score
```

### 2. Basic Chat Interface
```
┌─────────────────────────────┐
│ 🌾 AI4Thai Crop Guardian   │
├─────────────────────────────┤
│ Bot: สวัสดี! อัปโหลดรูปใบข้าว │
│      ที่เป็นโรค               │
│                             │
│ [📷 Upload Image]           │
│                             │
│ User: [Image uploaded]      │
│                             │
│ Bot: โรคใบไหม้ข้าว (95%)     │
│      ใช้ฟังไซด์พ่นใส่        │
│                             │
├─────────────────────────────┤
│ [Type message...] [Send]    │
└─────────────────────────────┘
```

### 3. Minimal AI Advisory
- **Detection**: "Rice Blast Disease detected"
- **Confidence**: "95% confidence"
- **Treatment**: "Apply fungicide spray, remove affected leaves"
- **Thai Translation**: Basic pre-translated responses

## ⚙️ Backend Requirements (24 hours)

### Hour Breakdown
- **Hours 1-4**: Project setup + basic Axum server
- **Hours 5-12**: Image upload + classification integration  
- **Hours 13-18**: Chat API + SQLite database
- **Hours 19-24**: Mock external APIs + Docker setup

### Simplified API Endpoints
```rust
// Minimal API surface
POST /api/upload          // Image upload + classification
GET  /api/chat/{id}       // Get chat history  
POST /api/chat/{id}/message // Send message
POST /api/diagnose        // Disease diagnosis
```

### Database Schema (SQLite)
```sql
-- Minimal tables
CREATE TABLE chats (
    id INTEGER PRIMARY KEY,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE messages (
    id INTEGER PRIMARY KEY,
    chat_id INTEGER REFERENCES chats(id),
    content TEXT,
    is_user BOOLEAN,
    image_path TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE diagnoses (
    id INTEGER PRIMARY KEY,
    image_path TEXT,
    disease_name TEXT,
    confidence REAL,
    advice TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### Vision Model Options
**Option A: External API (Recommended)**
```rust
// Use existing vision API with <16GB constraint
async fn classify_image(image: Vec<u8>) -> Result<DiagnosisResult> {
    // Call external lightweight vision API
    // Or use HuggingFace Inference API
}
```

**Option B: Local Model**
```rust
// Use ONNX Runtime with small model (<1GB)
// Models: MobileNet, EfficientNet-B0
```

### Mock External Services
```rust
// Mock LLM responses
fn get_treatment_advice(disease: &str) -> String {
    match disease {
        "rice_blast" => "ใช้ฟังไซด์พ่น เอาใบที่เป็นโรคออก",
        _ => "ปรึกษาผู้เชี่ยวชาญ"
    }
}

// Mock TTS (return audio file URL)
fn text_to_speech(text: &str) -> String {
    format!("/audio/mock_{}.mp3", text.len())
}
```

## 📱 Frontend Requirements (18 hours)

### Hour Breakdown
- **Hours 1-6**: HTML/CSS basic chat interface
- **Hours 7-12**: JavaScript camera integration + API calls
- **Hours 13-15**: Thai language UI + responsive design
- **Hours 16-18**: PWA setup + demo polish

### Technology Choice: Vanilla JS (Not Rust WASM)
**Reasoning**: 
- Rust WASM needs 8+ hours setup for hackathon timeline
- Vanilla JS allows rapid prototyping
- Focus on functionality over tech complexity

### Simplified Frontend Structure
```
frontend/
├── index.html              # Main chat interface
├── style.css              # Bootstrap + custom styles
├── app.js                 # Core functionality
├── camera.js              # Image capture logic
├── api.js                 # Backend communication
├── manifest.json          # PWA manifest
└── sw.js                  # Basic service worker
```

### Core JavaScript Features
```javascript
// Image capture and upload
async function captureImage() {
    const stream = await navigator.mediaDevices.getUserMedia({video: true});
    // Capture, compress, upload
}

// Chat functionality
function sendMessage(content, image = null) {
    // Send to backend, update UI
}

// Disease diagnosis display
function showDiagnosis(result) {
    // Display disease name, confidence, advice
}
```

### UI Framework: Bootstrap 5
- **Fast setup**: CDN links, no build process
- **Responsive**: Mobile-first design
- **Components**: Cards, buttons, forms
- **Thai fonts**: Google Fonts integration

## 🚀 Deployment Strategy

### Docker Compose Setup
```yaml
# docker-compose.yml
version: '3.8'

services:
  backend:
    build: ./backend
    ports:
      - "3000:3000"
    volumes:
      - ./data:/app/data
      - ./uploads:/app/uploads
    environment:
      - DATABASE_URL=sqlite:///app/data/app.db

  frontend:
    build: ./frontend
    ports:
      - "8080:80"
    depends_on:
      - backend

  # Optional: nginx for file serving
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./uploads:/usr/share/nginx/html/uploads
```

### Build Process
```bash
# Simple build commands
cd backend && cargo build --release
cd frontend && cp -r . ../dist/
docker-compose up --build
```

## 📊 Hackathon Demo Script

### Demo Flow (5 minutes)
1. **Introduction** (30s): Product vision for Thai farmers
2. **Live Demo** (3m):
   - Open web app on mobile
   - Take photo of rice plant (prepared sample)
   - Show instant disease detection
   - Display Thai treatment advice
3. **Technical Highlights** (1m): Rust backend, Docker deployment
4. **Q&A** (30s): Future roadmap discussion

### Demo Assets Needed
- **Sample rice disease images**: 5-10 high-quality photos
- **Pre-trained responses**: Thai translations ready
- **Mobile device**: For live camera demo
- **Backup slides**: In case demo fails

## 🎯 Success Metrics for Hackathon

### Technical Achievement
- ✅ **Working image classification**: Real model inference
- ✅ **End-to-end flow**: Upload → Diagnosis → Advice  
- ✅ **Thai language**: Basic UI + responses
- ✅ **Mobile responsive**: Works on smartphone
- ✅ **Docker deployment**: One-command setup

### Demo Impact
- ✅ **Clear problem**: Agricultural pain point addressed
- ✅ **Technical innovation**: AI + Rust + Thai language
- ✅ **Market potential**: Scalable solution
- ✅ **Social impact**: Farmer empowerment story

## ⚠️ Risk Mitigation

### Technical Risks
- **Model integration fails**: Fallback to mock responses
- **Camera doesn't work**: File upload alternative  
- **Thai fonts issue**: English fallback
- **Docker problems**: Local development demo

### Time Management
- **Backend behind schedule**: Cut optional features first
- **Frontend delayed**: Focus on mobile-only UI
- **Integration issues**: Mock APIs until working
- **Demo prep time**: Reserve last 2 hours for demo practice

## 🔧 Minimum Viable Demo

### Absolute Must-Haves
1. Image upload works
2. Some disease classification (even if basic)
3. Thai text displays correctly
4. Runs on mobile browser
5. Docker deployment works

### Nice-to-Haves
- Voice input/output
- Multiple crop types
- Advanced AI advice
- Offline functionality
- User authentication

### Emergency Fallback
If technical issues arise:
- **Static demo**: Pre-recorded interactions
- **Slides**: Technical architecture explanation  
- **Code walkthrough**: Show Rust implementation
- **Vision explanation**: Future roadmap presentation

## 📅 Development Timeline

### Day 1 (20 hours)
- **Hours 1-8**: Backend core + basic API
- **Hours 9-16**: Frontend basic UI + camera
- **Hours 17-20**: Integration testing

### Day 2 (20 hours) 
- **Hours 21-28**: Vision model integration
- **Hours 29-36**: Thai language + UI polish
- **Hours 37-40**: Demo preparation + deployment

### Checkpoints
- **Hour 12**: Basic chat UI working
- **Hour 24**: Image upload + mock response
- **Hour 36**: Full integration complete
- **Hour 40**: Demo ready + deployed