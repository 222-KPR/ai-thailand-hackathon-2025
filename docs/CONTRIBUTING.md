# Contributing to AI4Thai Crop Guardian

Thank you for your interest in contributing to AI4Thai Crop Guardian! This document provides guidelines and information for contributors.

## 🎯 Overview

AI4Thai Crop Guardian is an open-source project aimed at democratizing AI-powered agricultural expertise for Thai farmers. We welcome contributions from developers, designers, agricultural experts, and anyone passionate about improving farming practices through technology.

## 🤝 How to Contribute

### Types of Contributions

We welcome various types of contributions:

- 🐛 **Bug Reports**: Help us identify and fix issues
- 💡 **Feature Requests**: Suggest new features or improvements
- 📝 **Documentation**: Improve or add documentation
- 🔧 **Code Contributions**: Fix bugs or implement new features
- 🎨 **Design**: UI/UX improvements and design assets
- 🌐 **Translations**: Help localize the application
- 🧪 **Testing**: Write tests or help with quality assurance

### Getting Started

1. **Fork the Repository**
   ```bash
   git clone https://github.com/your-username/ai4thai-crop-guardian.git
   cd ai4thai-crop-guardian
   ```

2. **Set Up Development Environment**
   ```bash
   ./scripts/setup-dev.sh
   ```

3. **Create a Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make Your Changes**
   - Follow our coding standards
   - Write tests for new functionality
   - Update documentation as needed

5. **Test Your Changes**
   ```bash
   ./scripts/test-all.sh
   ```

6. **Submit a Pull Request**
   - Push your branch to your fork
   - Create a pull request with a clear description

## 📋 Development Guidelines

### Code Style

#### Rust Code
- Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write comprehensive documentation comments

```rust
/// Detects diseases in crop images using machine learning models.
/// 
/// # Arguments
/// 
/// * `image_data` - The image data as bytes
/// * `crop_type` - The type of crop being analyzed
/// 
/// # Returns
/// 
/// A `Result` containing the disease detection results or an error
/// 
/// # Examples
/// 
/// ```
/// let result = detect_disease(&image_data, CropType::Rice).await?;
/// println!("Detected diseases: {:?}", result.diseases);
/// ```
pub async fn detect_disease(
    image_data: &[u8], 
    crop_type: CropType
) -> Result<DetectionResult, DetectionError> {
    // Implementation
}
```

#### Python Code
- Follow [PEP 8](https://pep8.org/) style guide
- Use `black` for code formatting
- Use `flake8` for linting
- Write docstrings for all functions and classes

```python
def process_image(image_path: str, crop_type: str) -> Dict[str, Any]:
    """
    Process crop image for disease detection.
    
    Args:
        image_path: Path to the image file
        crop_type: Type of crop ('rice', 'cassava', etc.)
        
    Returns:
        Dictionary containing detection results
        
    Raises:
        ValueError: If image_path is invalid
        ProcessingError: If image processing fails
        
    Example:
        >>> result = process_image('crop.jpg', 'rice')
        >>> print(result['diseases'])
    """
    # Implementation
```

#### Frontend Code (Yew/Rust)
- Use consistent component structure
- Implement proper error handling
- Write unit tests for components
- Follow accessibility guidelines

```rust
#[derive(Properties, PartialEq)]
pub struct CropImageProps {
    pub image_url: String,
    pub alt_text: String,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(CropImage)]
pub fn crop_image(props: &CropImageProps) -> Html {
    let onclick = {
        let on_click = props.on_click.clone();
        Callback::from(move |e: MouseEvent| {
            on_click.emit(e);
        })
    };

    html! {
        <img 
            src={props.image_url.clone()}
            alt={props.alt_text.clone()}
            onclick={onclick}
            class="crop-image"
            role="button"
            tabindex="0"
        />
    }
}
```

### Testing Guidelines

#### Unit Tests
- Write tests for all public functions
- Aim for >80% code coverage
- Use descriptive test names
- Follow the Arrange-Act-Assert pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_disease_with_valid_rice_image() {
        // Arrange
        let image_data = load_test_image("rice_blast.jpg");
        let crop_type = CropType::Rice;

        // Act
        let result = detect_disease(&image_data, crop_type).await;

        // Assert
        assert!(result.is_ok());
        let detection = result.unwrap();
        assert_eq!(detection.diseases.len(), 1);
        assert_eq!(detection.diseases[0].name, "Rice Blast");
        assert!(detection.diseases[0].confidence > 0.8);
    }
}
```

#### Integration Tests
- Test API endpoints end-to-end
- Test service interactions
- Use test databases and mock external services

```python
import pytest
from fastapi.testclient import TestClient
from app import app

client = TestClient(app)

def test_disease_detection_endpoint():
    """Test the disease detection API endpoint."""
    with open("test_images/rice_blast.jpg", "rb") as image_file:
        response = client.post(
            "/detect",
            files={"image": ("rice_blast.jpg", image_file, "image/jpeg")},
            data={"crop_type": "rice"}
        )
    
    assert response.status_code == 200
    data = response.json()
    assert "diseases" in data
    assert len(data["diseases"]) > 0
    assert data["diseases"][0]["confidence"] > 0.8
```

### Documentation Standards

#### Code Documentation
- Document all public APIs
- Include examples in documentation
- Keep documentation up-to-date with code changes
- Use clear, concise language

#### API Documentation
- Use OpenAPI/Swagger specifications
- Include request/response examples
- Document error codes and responses
- Provide authentication examples

#### User Documentation
- Write step-by-step guides
- Include screenshots where helpful
- Test documentation with real users
- Keep language simple and accessible

### Git Workflow

#### Branch Naming
- `feature/description` - New features
- `bugfix/description` - Bug fixes
- `hotfix/description` - Critical fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring

#### Commit Messages
Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
type(scope): description

[optional body]

[optional footer]
```

Examples:
```
feat(vision): add rice blast detection model
fix(api): resolve authentication token expiration issue
docs(readme): update installation instructions
test(vision): add unit tests for image preprocessing
```

#### Pull Request Process

1. **Create Descriptive PR Title**
   ```
   feat(vision): Add support for cassava disease detection
   ```

2. **Fill Out PR Template**
   - Description of changes
   - Testing performed
   - Screenshots (if UI changes)
   - Breaking changes (if any)

3. **Ensure CI Passes**
   - All tests pass
   - Code coverage meets requirements
   - Linting passes
   - Documentation builds successfully

4. **Request Review**
   - Tag relevant reviewers
   - Respond to feedback promptly
   - Make requested changes

5. **Merge Requirements**
   - At least one approval from maintainer
   - All CI checks pass
   - No merge conflicts
   - Up-to-date with main branch

## 🐛 Reporting Issues

### Bug Reports

When reporting bugs, please include:

1. **Clear Title**: Descriptive summary of the issue
2. **Environment**: OS, browser, versions
3. **Steps to Reproduce**: Detailed steps
4. **Expected Behavior**: What should happen
5. **Actual Behavior**: What actually happens
6. **Screenshots**: If applicable
7. **Logs**: Relevant error messages

**Bug Report Template:**
```markdown
## Bug Description
Brief description of the bug

## Environment
- OS: macOS 12.6
- Browser: Chrome 108.0
- Version: v1.0.0

## Steps to Reproduce
1. Go to disease detection page
2. Upload rice image
3. Click "Analyze"
4. See error

## Expected Behavior
Should display disease detection results

## Actual Behavior
Shows "Internal Server Error" message

## Screenshots
[Attach screenshots]

## Additional Context
Any other relevant information
```

### Feature Requests

When requesting features, please include:

1. **Problem Statement**: What problem does this solve?
2. **Proposed Solution**: How should it work?
3. **Use Cases**: Who would use this feature?
4. **Alternatives**: Other solutions considered
5. **Additional Context**: Any other relevant information

## 🌐 Internationalization

### Adding Translations

1. **Create Translation Files**
   ```
   frontend/locales/
   ├── en.json
   ├── th.json
   └── [language-code].json
   ```

2. **Translation Format**
   ```json
   {
     "common": {
       "submit": "Submit",
       "cancel": "Cancel",
       "loading": "Loading..."
     },
     "disease_detection": {
       "title": "Disease Detection",
       "upload_image": "Upload Image",
       "analyzing": "Analyzing image..."
     }
   }
   ```

3. **Use Translation Keys**
   ```rust
   use yew_i18n::use_translation;
   
   #[function_component(DetectionPage)]
   pub fn detection_page() -> Html {
       let t = use_translation();
       
       html! {
           <h1>{t.get("disease_detection.title")}</h1>
           <button>{t.get("common.submit")}</button>
       }
   }
   ```

## 🏆 Recognition

### Contributors

We recognize contributors in several ways:

- **Contributors List**: Listed in README.md
- **Release Notes**: Mentioned in release announcements
- **Hall of Fame**: Featured on project website
- **Swag**: Stickers and merchandise for significant contributions

### Contribution Levels

- **First-time Contributor**: Welcome package and mentorship
- **Regular Contributor**: Recognition in release notes
- **Core Contributor**: Commit access and decision-making input
- **Maintainer**: Full project access and responsibilities

## 📞 Getting Help

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and discussions
- **Discord**: Real-time chat and community support
- **Email**: Direct contact for sensitive issues

### Mentorship Program

New contributors can request mentorship:

1. Comment on a "good first issue"
2. Tag `@mentorship-team`
3. Get paired with an experienced contributor
4. Receive guidance and support

## 📄 Code of Conduct

### Our Pledge

We pledge to make participation in our project a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, gender identity and expression, level of experience, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards

**Positive behaviors include:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

**Unacceptable behaviors include:**
- Harassment, trolling, or discriminatory comments
- Publishing others' private information
- Other conduct which could reasonably be considered inappropriate

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported by contacting the project team at conduct@ai4thai.com. All complaints will be reviewed and investigated promptly and fairly.

## 📚 Additional Resources

- [Development Setup Guide](development/setup.md)
- [Architecture Overview](architecture/README.md)
- [API Documentation](api/README.md)
- [Deployment Guide](deployment/README.md)
- [Testing Guide](development/testing.md)

## 🙏 Thank You

Thank you for contributing to AI4Thai Crop Guardian! Your contributions help make agricultural technology more accessible to farmers worldwide.

---

For questions about contributing, please reach out to the maintainers or create a discussion in the repository.
