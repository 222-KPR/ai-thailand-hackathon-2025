# Frontend Development Guide

This section contains comprehensive documentation for the AI4Thai Crop Guardian frontend development using Yew WebAssembly.

## 📋 Frontend Documentation

### 🎨 [Design System](design-system.md)
Complete design system with dopamine colors, typography, and component library.

### 🏗️ [Component Architecture](component-architecture.md)
Component structure, patterns, and best practices.

### 🧪 [Testing Strategy](testing-strategy.md)
Frontend testing approaches, frameworks, and coverage requirements.

### 📱 [PWA Implementation](pwa-implementation.md)
Progressive Web App features, service workers, and offline functionality.

### 🌐 [Internationalization](internationalization.md)
Thai language support, localization, and cultural considerations.

### ⚡ [Performance Optimization](performance-optimization.md)
WebAssembly optimization, bundle size, and loading performance.

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ with WebAssembly target
- Trunk (WebAssembly build tool)
- Node.js 18+ (for tooling)

### Setup
```bash
# Install WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk wasm-pack

# Navigate to frontend
cd frontend

# Start development server
trunk serve --port 8080 --open
```

## 🏗️ Architecture Overview

### Technology Stack
- **Framework**: Yew (Rust WebAssembly)
- **Build Tool**: Trunk
- **Styling**: SCSS with CSS custom properties
- **State Management**: Yewdux
- **Routing**: Yew Router
- **Testing**: wasm-pack-test

### Project Structure
```
frontend/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Main app component
│   ├── components/          # Reusable components
│   │   ├── layout/          # Layout components
│   │   ├── ui/              # UI components
│   │   ├── chat/            # Chat interface
│   │   └── camera/          # Camera components
│   ├── pages/               # Page components
│   ├── services/            # API services
│   ├── stores/              # State management
│   ├── utils/               # Utility functions
│   └── styles/              # SCSS stylesheets
├── static/                  # Static assets
├── tests/                   # Frontend tests
└── Trunk.toml              # Build configuration
```

## 🎨 Design System

### Dopamine Color Palette
```rust
pub struct ColorPalette {
    // Primary dopamine colors
    pub primary_electric_blue: &'static str = "#0066FF";
    pub primary_vibrant_orange: &'static str = "#FF6B35";
    pub primary_energetic_pink: &'static str = "#FF1B8D";
    
    // Accent colors
    pub accent_lime_green: &'static str = "#32D74B";
    pub accent_purple: &'static str = "#AF52DE";
    pub accent_yellow: &'static str = "#FFD60A";
}
```

### Bento Grid System
- Modular grid layout for flexible content organization
- Responsive design with mobile-first approach
- Support for various card sizes and layouts

## 🧪 Testing Strategy

### Test Coverage Requirements
- **Overall Coverage**: 85% minimum
- **Components**: 90% minimum
- **Design System**: 95% minimum
- **Critical Paths**: 100% required

### Testing Pyramid
- **70% Unit Tests**: Component logic and utilities
- **20% Integration Tests**: Component interactions
- **10% E2E Tests**: User workflows

## 📊 Performance Targets

### Bundle Size
- **Initial Bundle**: < 500KB gzipped
- **Lazy Loaded Chunks**: < 100KB each
- **Total Assets**: < 2MB

### Loading Performance
- **First Contentful Paint**: < 1.5s
- **Largest Contentful Paint**: < 2.5s
- **Time to Interactive**: < 3s
- **Cumulative Layout Shift**: < 0.1

## 🔧 Development Workflow

### Component Development
1. Create component with proper TypeScript-like props
2. Implement responsive design
3. Add micro-interactions and animations
4. Write comprehensive tests
5. Update Storybook documentation

### Testing Workflow
1. Write unit tests for component logic
2. Add integration tests for user interactions
3. Ensure accessibility compliance
4. Validate responsive behavior
5. Performance testing

### Code Quality
- **Linting**: Clippy for Rust code
- **Formatting**: rustfmt with custom configuration
- **Type Safety**: Leverage Rust's type system
- **Documentation**: Comprehensive doc comments

## 🌐 Internationalization

### Thai Language Support
- **Fonts**: Optimized Thai font rendering
- **Typography**: Increased line heights for Thai text
- **Cultural Design**: Colors and patterns that resonate with Thai culture
- **RTL Support**: Future-ready for additional languages

## 📱 PWA Features

### Core Features
- **Offline Functionality**: Service worker with caching strategies
- **App-like Experience**: Home screen installation
- **Push Notifications**: Treatment reminders and updates
- **Background Sync**: Offline data synchronization

### Performance Optimizations
- **Code Splitting**: Lazy loading of routes and components
- **Asset Optimization**: Image compression and WebP support
- **Caching Strategy**: Intelligent caching of API responses
- **Bundle Analysis**: Regular bundle size monitoring

---

For frontend development questions, please refer to the specific documentation sections or create an issue in the repository.
