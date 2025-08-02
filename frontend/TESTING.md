# Frontend Testing Guide

## Mobile Responsiveness Verification

The AI4Thai Crop Guardian frontend has been designed with mobile-first responsive design principles.

### Responsive Breakpoints

1. **Mobile (max-width: 480px)**:
   - Single column layout for crop grid
   - Smaller font sizes for titles
   - Reduced spacing and padding

2. **Tablet (max-width: 768px)**:
   - Two-column crop grid
   - Compressed header (logo text hidden)
   - Adjusted container padding (0.75rem)
   - Reduced header height (3.5rem)

3. **Desktop (min-width: 769px)**:
   - Full chat container with margins
   - Two-column step layout
   - Maximum message width 70%

4. **Large Desktop (min-width: 1024px)**:
   - Maximum message width 60%
   - Full padding and spacing

### Key Mobile Features

- **Viewport Meta Tag**: Properly configured for mobile devices
- **Touch-Friendly**: Button sizes optimized for touch (minimum 44px)
- **Flexible Layout**: CSS Grid and Flexbox for responsive layouts
- **Thai Font Support**: Sarabun font with system fallbacks
- **PWA Ready**: Manifest and service worker included
- **Dark Mode**: Automatic dark mode detection

### Manual Testing Steps

1. **Desktop Testing**:
   ```bash
   trunk serve --address 0.0.0.0 --port 8080
   # Open http://localhost:8080 in browser
   # Test with browser dev tools device emulation
   ```

2. **Mobile Device Testing**:
   - iPhone SE (375x667)
   - iPhone 12 Pro (390x844)
   - Samsung Galaxy S21 (360x800)
   - iPad (768x1024)

3. **Feature Testing**:
   - Chat interface functionality
   - Message sending with Enter key
   - Language toggle (Thai/English)
   - Image upload interface
   - Welcome message display
   - Error handling display

### Automated Tests

Run the unit tests to verify core functionality:

```bash
cargo test
```

All 6 tests should pass:
- App state creation
- Chat message creation
- I18n context functionality
- Image validation utilities
- Text validation functions
- Text sanitization

### Performance Considerations

- **Inlined CSS**: All styles embedded in HTML for faster loading
- **WASM Optimization**: Rust compiled to optimized WebAssembly
- **Minimal Dependencies**: Only essential JavaScript libraries
- **Progressive Enhancement**: Works without JavaScript for basic functionality