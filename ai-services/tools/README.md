# AI4Thai Configuration Tools

This directory contains shared tooling and configuration templates for all AI4Thai Python services within the ai-services directory.

## 🔧 Tools

### `generate-configs.py`

Generates standardized configuration files for AI4Thai services from shared templates.

**Usage:**
```bash
# Navigate to ai-services directory first
cd ai-services

# First time setup with uv
uv sync

# Generate configs for all services
uv run python tools/generate-configs.py

# Generate configs for specific service only
uv run python tools/generate-configs.py --service vision-service

# Validate configuration without generating files
uv run python tools/generate-configs.py --validate

# Preview what would be generated (dry run)
uv run python tools/generate-configs.py --dry-run
```

**Requirements:**
- Python 3.11+
- uv package manager
- Dependencies managed via `uv sync` in the ai-services directory

## 📋 Configuration Standards

### Port Allocation
- **Constraint**: AI services must use ports 2001-2099
- **Vision Service**: Port 2001
- **Queue Worker**: Port 2003
- **Future Services**: Ports 2004-2099

### API Gateway Compliance

All external-facing services must comply with the provider API gateway response format:

**Required Response Format:**
```json
{
  "timestamp": 1704283200.123,
  "detail": {
    "status": "success|error",
    "message": "Description",
    "data": { /* actual response data */ }
  }
}
```

**Implementation:**
Services with `"external_access": true` will automatically get the `api_gateway_response.py` helper file generated.

### Generated Files

For each service, the following files are generated:

1. **`pyproject.toml`** - Python project configuration with dependencies
2. **`Makefile`** - Standardized development commands
3. **`Dockerfile`** - Multi-stage Docker build configuration
4. **`.python-version`** - Python version specification
5. **`api_gateway_response.py`** - API gateway response format helper (external services only)

## 📁 Templates

Located in `ai-services/tools/shared-config/`:

- `pyproject.toml.template` - Python project configuration template
- `Dockerfile.python.template` - Docker configuration for Python services
- `Makefile.template` - Development workflow commands
- `api_gateway_response.py.template` - API gateway compliance helper
- `services.json` - Service definitions and metadata

## 🔍 Configuration Structure

### Service Definition Format

```json
{
  "service-name": {
    "package_name": "ai4thai-service-name",
    "description": "Service description",
    "version": "1.0.0",
    "path": "relative/path/to/service",
    "type": "service-type",
    "language": "python",
    "python_version": "3.11",
    "port": 2001,
    "external_access": true,
    "keywords": ["agriculture", "ai", "thailand"],
    "additional_dependencies": [
      "# Comments for dependency groups",
      "package==version",
      "another-package==version"
    ]
  }
}
```

### Constraints and Validation

The configuration generator validates:
- ✅ All required fields are present
- ✅ Ports are within the allowed range (2001-2099)
- ✅ No port conflicts between services
- ✅ Service directories exist
- ⚠️  Service configuration compliance

## 🚀 Adding New Services

1. Add service definition to `ai-services/tools/shared-config/services.json`
2. Ensure port is in range 2001-2099 and not already used
3. Set `external_access: true` if service needs API gateway compliance
4. Run `uv run python tools/generate-configs.py --service new-service-name` from the ai-services directory
5. Customize generated files as needed

## 📡 API Gateway Integration

External access to AI services goes through a provider API gateway that requires:

- **Response Format**: Must include `timestamp` and `detail` keys
- **Port Range**: Services must use ports 2001-2099
- **Standardization**: Use the generated `api_gateway_response.py` helper

Example usage in FastAPI endpoints:
```python
from api_gateway_response import APIGatewayResponse

@app.post("/detect/pests")
async def detect_pests(image: UploadFile):
    try:
        results = await process_image(image)
        return APIGatewayResponse.success(
            data=results,
            message="Pest detection completed",
            processing_time_ms=123.45
        )
    except Exception as e:
        return APIGatewayResponse.error(
            error_message=str(e),
            error_code="DETECTION_FAILED"
        )
```

## 🔄 Backup and Safety

The configuration generator automatically:
- Creates timestamped backups of existing files before overwriting
- Validates configuration before generation
- Provides dry-run mode for preview
- Maintains backup files with format: `filename.ext.backup_YYYYMMDD_HHMMSS`
