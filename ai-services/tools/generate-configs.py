#!/usr/bin/env python3
"""
AI4Thai Configuration Generator
Generates service-specific configurations from shared templates
"""

import argparse
import json
import os
import shutil
import sys
from datetime import datetime
from pathlib import Path
from typing import Any, Dict

try:
    from jinja2 import Environment, FileSystemLoader, select_autoescape
except ImportError:
    print("Error: jinja2 is required. Install with: uv add jinja2")
    sys.exit(1)


class ConfigGenerator:
    """Generates configurations for AI4Thai services from templates."""

    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.tools_dir = project_root / "ai-services" / "tools" / "shared-config"
        self.services_config_path = self.tools_dir / "services.json"

        # Initialize Jinja2 environment
        self.env = Environment(
            loader=FileSystemLoader(str(self.tools_dir)),
            autoescape=select_autoescape(),
            trim_blocks=True,
            lstrip_blocks=True,
        )

    def load_services_config(self) -> dict[str, Any]:
        """Load services configuration from JSON file."""
        try:
            with open(self.services_config_path) as f:
                return json.load(f)
        except FileNotFoundError:
            print(
                f"Error: Services configuration not found at {self.services_config_path}"
            )
            sys.exit(1)
        except json.JSONDecodeError as e:
            print(f"Error: Invalid JSON in services configuration: {e}")
            sys.exit(1)

    def generate_service_config(
        self, service_name: str, config: dict[str, Any]
    ) -> None:
        """Generate configuration files for a specific service."""
        service_dir = self.project_root / config["path"]

        if not service_dir.exists():
            print(f"Warning: Service directory {service_dir} does not exist")
            return

        print(f"Generating configurations for {service_name}...")

        # Generate pyproject.toml
        self._generate_pyproject_toml(service_dir, service_name, config)

        # Generate Makefile
        self._generate_makefile(service_dir, service_name, config)

        # Generate Dockerfile
        self._generate_dockerfile(service_dir, service_name, config)

        # Generate .python-version
        self._generate_python_version(service_dir, config)

        # Generate API Gateway response format (for external-facing services)
        if config.get("external_access", True):
            self._generate_api_gateway_response(service_dir, service_name, config)

        print(f"✅ Configuration generated for {service_name}")

    def _backup_existing_file(self, file_path: Path) -> None:
        """Create a backup of existing file if it exists."""
        if file_path.exists():
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            backup_path = file_path.with_suffix(
                f"{file_path.suffix}.backup_{timestamp}"
            )
            shutil.copy2(file_path, backup_path)
            print(f"    📦 Backed up existing file to {backup_path.name}")

    def _write_generated_file(
        self, output_path: Path, content: str, description: str
    ) -> None:
        """Write generated content to file with backup."""
        self._backup_existing_file(output_path)

        with open(output_path, "w") as f:
            f.write(content)
        print(f"    ✅ Generated {description}")

    def _generate_pyproject_toml(
        self, service_dir: Path, service_name: str, config: dict[str, Any]
    ) -> None:
        """Generate pyproject.toml from template."""
        template = self.env.get_template("pyproject.toml.template")

        # Prepare template variables
        variables = {
            "SERVICE_NAME": config["package_name"],
            "SERVICE_VERSION": config.get("version", "1.0.0"),
            "SERVICE_DESCRIPTION": config["description"],
            "README_PATH": config.get("readme_path", "README.md"),
            "KEYWORDS": json.dumps(
                config.get("keywords", ["agriculture", "ai", "thailand"])
            ),
            "ADDITIONAL_DEPENDENCIES": self._format_dependencies(
                config.get("additional_dependencies", [])
            ),
        }

        output = template.render(**variables)
        output_path = service_dir / "pyproject.toml"

        self._write_generated_file(output_path, output, "pyproject.toml")

    def _generate_makefile(
        self, service_dir: Path, service_name: str, config: dict[str, Any]
    ) -> None:
        """Generate Makefile from template."""
        template = self.env.get_template("Makefile.template")

        variables = {
            "SERVICE_NAME": service_name,
            "SERVICE_PORT": config["port"],
            "SERVICE_TYPE": config["type"],
        }

        output = template.render(**variables)
        output_path = service_dir / "Makefile"

        self._write_generated_file(output_path, output, "Makefile")

    def _generate_dockerfile(
        self, service_dir: Path, service_name: str, config: dict[str, Any]
    ) -> None:
        """Generate Dockerfile from template."""
        if config.get("language") != "python":
            return  # Skip non-Python services

        template = self.env.get_template("Dockerfile.python.template")

        variables = {
            "SERVICE_NAME": service_name,
            "SERVICE_PORT": config["port"],
            "SERVICE_TYPE": config["type"],
        }

        output = template.render(**variables)
        output_path = service_dir / "Dockerfile"

        self._write_generated_file(output_path, output, "Dockerfile")

    def _generate_python_version(
        self, service_dir: Path, config: dict[str, Any]
    ) -> None:
        """Generate .python-version file."""
        python_version = config.get("python_version", "3.11")
        output_path = service_dir / ".python-version"

        self._write_generated_file(
            output_path, f"{python_version}\n", ".python-version"
        )

    def _generate_api_gateway_response(
        self, service_dir: Path, service_name: str, config: dict[str, Any]
    ) -> None:
        """Generate API Gateway response format helper file."""
        template = self.env.get_template("api_gateway_response.py.template")

        # Copy template content as-is (no variables needed for this template)
        output = template.render()
        output_path = service_dir / "api_gateway_response.py"

        self._write_generated_file(output_path, output, "api_gateway_response.py")

    def _format_dependencies(self, deps: list) -> str:
        """Format additional dependencies for template."""
        if not deps:
            return ""

        formatted = []
        for dep in deps:
            if isinstance(dep, str):
                # Handle comments and empty lines
                if dep.startswith("#") or dep.strip() == "":
                    formatted.append(f"    {dep}")
                else:
                    formatted.append(f'    "{dep}",')
            elif isinstance(dep, dict):
                formatted.append(f'    "{dep["name"]}=={dep["version"]}",')

        return "\n" + "\n".join(formatted) if formatted else ""

    def generate_all_configs(self) -> None:
        """Generate configurations for all services."""
        services_config = self.load_services_config()

        for service_name, config in services_config["services"].items():
            self.generate_service_config(service_name, config)

        print("\n✅ All configurations generated successfully!")
        print(
            "📝 To apply changes, run the generated Makefiles in each service directory"
        )

    def validate_config(self) -> bool:
        """Validate the services configuration."""
        try:
            services_config = self.load_services_config()

            print("🔍 Validating services configuration...")

            # Get port constraints from metadata
            constraints = services_config.get("metadata", {}).get("constraints", {})
            port_range = constraints.get("port_range", {})
            min_port = port_range.get("min", 2001)
            max_port = port_range.get("max", 2099)

            required_service_fields = [
                "package_name",
                "description",
                "path",
                "type",
                "language",
                "port",
            ]

            for service_name, config in services_config["services"].items():
                print(f"  Checking {service_name}...")

                # Check required fields
                for field in required_service_fields:
                    if field not in config:
                        print(f"    ❌ Missing required field: {field}")
                        return False

                # Check if service directory exists
                service_dir = self.project_root / config["path"]
                if not service_dir.exists():
                    print(f"    ⚠️  Service directory does not exist: {service_dir}")

                # Check port constraints
                port = config["port"]
                if not (min_port <= port <= max_port):
                    print(
                        f"    ❌ Port {port} is outside allowed range {min_port}-{max_port}"
                    )
                    return False

                # Check port uniqueness
                for other_name, other_config in services_config["services"].items():
                    if other_name != service_name and other_config.get("port") == port:
                        print(
                            f"    ❌ Port conflict: {port} used by both {service_name} and {other_name}"
                        )
                        return False

                print(f"    ✅ {service_name} configuration valid")

            print("✅ Configuration validation passed!")
            print("📡 Note: External access will go through provider API gateway")
            print(f"🔌 Services using ports: {min_port}-{max_port}")
            return True

        except Exception as e:
            print(f"❌ Configuration validation failed: {e}")
            return False


def main():
    parser = argparse.ArgumentParser(
        description="Generate AI4Thai service configurations from templates"
    )
    parser.add_argument(
        "--service",
        help="Generate config for specific service only",
        metavar="SERVICE_NAME",
    )
    parser.add_argument(
        "--project-root",
        type=Path,
        default=Path(__file__).parent.parent.parent,
        help="Project root directory (default: auto-detect)",
    )
    parser.add_argument(
        "--validate",
        action="store_true",
        help="Validate services configuration without generating files",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Show what would be generated without creating files",
    )

    args = parser.parse_args()

    generator = ConfigGenerator(args.project_root)

    if args.validate:
        success = generator.validate_config()
        sys.exit(0 if success else 1)

    if args.dry_run:
        print("🔍 Dry run mode - showing what would be generated:")
        services_config = generator.load_services_config()
        for service_name, config in services_config["services"].items():
            if args.service and service_name != args.service:
                continue
            service_dir = generator.project_root / config["path"]
            print(f"\n📦 {service_name}:")
            print(f"  📁 Directory: {service_dir}")
            print("  📄 Files: pyproject.toml, Makefile, Dockerfile, .python-version")
            print(f"  🔌 Port: {config['port']}")
            print(f"  🐍 Python: {config.get('python_version', '3.11')}")
        return

    if args.service:
        services_config = generator.load_services_config()
        if args.service not in services_config["services"]:
            print(f"Error: Service '{args.service}' not found in configuration")
            available = ", ".join(services_config["services"].keys())
            print(f"Available services: {available}")
            sys.exit(1)
        generator.generate_service_config(
            args.service, services_config["services"][args.service]
        )
    else:
        generator.generate_all_configs()


if __name__ == "__main__":
    main()
