#!/usr/bin/env bash
# Update version across package.json, tauri.conf.json, and Cargo.toml from .env

set -e

ENV_PATH="${1:-.env}"

echo "📖 Reading version from .env..."

# Read version from .env
if [[ ! -f "$ENV_PATH" ]]; then
    echo "❌ Error: .env file not found at $ENV_PATH"
    exit 1
fi

VERSION=$(grep "^VERSION=" "$ENV_PATH" | cut -d'=' -f2 | tr -d ' \r\n')
APP_IDENTIFIER=$(grep "^APP_IDENTIFIER=" "$ENV_PATH" | cut -d'=' -f2- | sed -E 's/^[[:space:]]+|[[:space:]]+$//g' | sed -E 's/^"(.*)"$/\1/' | sed -E "s/^'(.*)'$/\1/")

if [[ -z "$VERSION" ]]; then
    echo "❌ Error: VERSION not found in .env"
    exit 1
fi

if [[ -z "$APP_IDENTIFIER" ]]; then
    echo "❌ Error: APP_IDENTIFIER not found in .env"
    exit 1
fi

echo "✅ Found version: $VERSION"
echo "✅ Found identifier: $APP_IDENTIFIER"

# Update frontend/package.json
echo "📝 Updating frontend/package.json..."
PACKAGE_JSON_PATH="frontend/package.json"
if [[ -f "$PACKAGE_JSON_PATH" ]]; then
    # Use jq if available, otherwise use sed
    if command -v jq &> /dev/null; then
        jq --arg version "$VERSION" '.version = $version' "$PACKAGE_JSON_PATH" > "${PACKAGE_JSON_PATH}.tmp"
        mv "${PACKAGE_JSON_PATH}.tmp" "$PACKAGE_JSON_PATH"
    else
        sed -i.bak -E "s/\"version\": \"[^\"]+\"/\"version\": \"$VERSION\"/" "$PACKAGE_JSON_PATH"
        rm -f "${PACKAGE_JSON_PATH}.bak"
    fi
    echo "✓ Updated frontend/package.json to version $VERSION"
else
    echo "⚠️  Warning: $PACKAGE_JSON_PATH not found"
fi

# Update backend/tauri.conf.json
echo "📝 Updating backend/tauri.conf.json..."
TAURI_CONF_PATH="backend/tauri.conf.json"
if [[ -f "$TAURI_CONF_PATH" ]]; then
    # Use jq if available, otherwise use sed
    if command -v jq &> /dev/null; then
        jq --arg version "$VERSION" --arg identifier "$APP_IDENTIFIER" '.version = $version | .identifier = $identifier' "$TAURI_CONF_PATH" > "${TAURI_CONF_PATH}.tmp"
        mv "${TAURI_CONF_PATH}.tmp" "$TAURI_CONF_PATH"
    else
        sed -i.bak -E "s/\"version\": \"[^\"]+\"/\"version\": \"$VERSION\"/" "$TAURI_CONF_PATH"
        sed -i.bak -E "s/\"identifier\": \"[^\"]+\"/\"identifier\": \"$APP_IDENTIFIER\"/" "$TAURI_CONF_PATH"
        rm -f "${TAURI_CONF_PATH}.bak"
    fi
    echo "✓ Updated backend/tauri.conf.json to version $VERSION and identifier $APP_IDENTIFIER"
else
    echo "⚠️  Warning: $TAURI_CONF_PATH not found"
fi

# Update backend/Cargo.toml
echo "📝 Updating backend/Cargo.toml..."
CARGO_TOML_PATH="backend/Cargo.toml"
if [[ -f "$CARGO_TOML_PATH" ]]; then
    sed -i.bak -E "s/^version = \"[^\"]+\"/version = \"$VERSION\"/" "$CARGO_TOML_PATH"
    rm -f "${CARGO_TOML_PATH}.bak"
    echo "✓ Updated backend/Cargo.toml to version $VERSION"
else
    echo "⚠️  Warning: $CARGO_TOML_PATH not found"
fi

echo ""
echo "🎉 Version update complete! All files now use version $VERSION"
