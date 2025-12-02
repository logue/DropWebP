# Building for Linux (Using Docker)

How to build Linux binaries from macOS using Docker

## 📋 Prerequisites

- Docker Desktop for Mac must be installed
- Sufficient disk space (approximately 5GB required for initial build)

## 🚀 Usage

### Building for x86_64 (AMD64)

```bash
# Run from project root
./scripts/build-linux-docker.sh x64

# Or from app directory
pnpm run build:tauri:linux-docker-x64
```

### Building for ARM64 (AArch64)

```bash
# Run from project root
./scripts/build-linux-docker.sh arm64

# Or from app directory
pnpm run build:tauri:linux-docker-arm64
```

## 📦 Build Artifacts

Build artifacts are generated in the following directories:

```text
app/src-tauri/target/
  ├── x86_64-unknown-linux-gnu/release/bundle/
  │   ├── deb/           # Debian/Ubuntu packages
  │   ├── rpm/           # Red Hat/Fedora packages
  │   └── appimage/      # AppImage (recommended for distribution)
  │
  └── aarch64-unknown-linux-gnu/release/bundle/
      ├── deb/
      ├── rpm/
      └── appimage/
```

## ⚙️ How It Works

1. Build Docker image from `Dockerfile.linux-build`
   - Based on Rust 1.83 + Debian Bookworm
   - Installs Tauri dependencies (WebKit2GTK, GTK3, etc.)
   - Installs Node.js 22.x and pnpm

2. Run Tauri build inside Docker container
   - Mount project directory
   - Build with specified target architecture

3. Output artifacts to macOS directory

## 🔧 Troubleshooting

### Rebuild Docker Image

```bash
docker build -f Dockerfile.linux-build -t dropwebp-linux-builder --no-cache .
```

### Remove Docker Image

```bash
docker rmi dropwebp-linux-builder
```

### Clear Build Cache

```bash
rm -rf app/src-tauri/target/x86_64-unknown-linux-gnu
rm -rf app/src-tauri/target/aarch64-unknown-linux-gnu
```

## 📝 Notes

- Initial build takes longer due to Docker image building and downloads (20-30 minutes)
- Subsequent builds are faster as Docker image is reused (10-15 minutes)
- ARM64 builds may take longer than x86_64 builds

## 🎯 Recommended Distribution Format

- **AppImage**: Recommended for distribution (works on all Linux distributions)
- **.deb**: For Debian/Ubuntu users
- **.rpm**: For Red Hat/Fedora users
