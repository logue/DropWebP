# Build DropWebP for Linux

This guide walks you through setting up the development environment and building DropWebP on Ubuntu 24.04 LTS (and similar Debian-based distributions).

## Prerequisites

Before you begin, make sure you have:

- Ubuntu 24.04 LTS or similar Debian-based distribution
- Sudo privileges for installing software
- Basic familiarity with terminal commands

## Step 1: Update System Packages

First, update your system packages to ensure you have the latest versions:

```bash
sudo apt update
sudo apt upgrade -y
```

## Step 2: Install Build Dependencies

Install the essential build tools and libraries required for Tauri development:

```bash
# Install build essentials and development libraries
sudo apt install -y \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libwebkit2gtk-4.1-dev \
  patchelf
```

### What These Packages Do

- **build-essential**: Provides GCC, G++, and make
- **libssl-dev**: OpenSSL development libraries
- **libgtk-3-dev**: GTK3 development libraries for UI
- **libayatana-appindicator3-dev**: System tray support
- **librsvg2-dev**: SVG rendering support
- **libwebkit2gtk-4.1-dev**: WebKit for Tauri's webview
- **patchelf**: ELF binary patcher for AppImage

### Verify Installation

```bash
gcc --version
```

You should see output showing GCC version 13.x or higher.

## Step 3: Install Rust

DropWebP is built with Rust, so you'll need to install the Rust toolchain.

### Install Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When prompted, choose option 1 (default installation).

### Configure Your Shell

```bash
source $HOME/.cargo/env
```

To make this permanent, add it to your shell profile:

```bash
echo 'source $HOME/.cargo/env' >> ~/.bashrc
source ~/.bashrc
```

### Verify Rust Installation

```bash
rustc --version
cargo --version
```

You should see version information for both `rustc` and `cargo`.

## Step 4: Install Node.js

The frontend of DropWebP is built with Vue.js and requires Node.js.

### Install Node.js via NodeSource Repository

```bash
# Install Node.js 22.x (LTS)
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt install -y nodejs
```

### Verify Node.js Installation

```bash
node --version
npm --version
```

You should see Node.js version 22.x or higher.

## Step 5: Install pnpm

DropWebP uses pnpm as its package manager for better performance and disk efficiency.

### Install pnpm

```bash
npm install -g pnpm
```

### Verify pnpm Installation

```bash
pnpm --version
```

## Step 6: Install Additional Dependencies

Install additional tools that may be required:

```bash
# Install CMake (for some native dependencies)
sudo apt install -y cmake

# Install pkg-config (for linking libraries)
sudo apt install -y pkg-config
```

## Step 7: Clone and Build DropWebP

Now you're ready to clone and build DropWebP.

### Clone the Repository

```bash
git clone https://github.com/logue/DropWebP.git
cd DropWebP
```

### Install Frontend Dependencies

```bash
# Install all workspace dependencies
pnpm install
```

### Install Tauri CLI v2

```bash
# Install Tauri CLI v2 globally
pnpm add -g @tauri-apps/cli@next
```

### Build the Application

For development:

```bash
# Run in development mode
pnpm dev:tauri
```

For production:

```bash
# Build for production
pnpm build:tauri
```

The built application will be in `app/src-tauri/target/release/`.

## Step 8: Distribution Formats

Tauri on Linux can generate multiple distribution formats:

### AppImage (Recommended)

AppImage is a universal package format that works on most Linux distributions:

```bash
pnpm build:tauri
```

The AppImage will be in `app/src-tauri/target/release/bundle/appimage/`.

### Debian Package (.deb)

For Debian/Ubuntu-based distributions:

```bash
pnpm build:tauri
```

The .deb package will be in `app/src-tauri/target/release/bundle/deb/`.

Install it with:

```bash
sudo dpkg -i app/src-tauri/target/release/bundle/deb/*.deb
```

### RPM Package (.rpm)

For Red Hat/Fedora-based distributions, you'll need to install additional tools:

```bash
sudo apt install -y rpm
pnpm build:tauri
```

The .rpm package will be in `app/src-tauri/target/release/bundle/rpm/`.

## Troubleshooting

### Common Issues

1. **Missing libwebkit2gtk-4.1**

   If you get errors about missing webkit libraries:

   ```bash
   # Try the older webkit version
   sudo apt install -y libwebkit2gtk-4.0-dev
   ```

2. **Permission Denied for npm/pnpm**

   ```bash
   # Fix npm global directory permissions
   mkdir -p ~/.npm-global
   npm config set prefix '~/.npm-global'
   echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **Build Failures with Native Dependencies**

   ```bash
   # Clear build caches
   cargo clean
   pnpm clean

   # Rebuild everything
   pnpm install
   pnpm build:tauri
   ```

4. **AppImage Not Executable**

   ```bash
   # Make AppImage executable
   chmod +x app/src-tauri/target/release/bundle/appimage/*.AppImage
   ```

5. **Missing GLIBC Version**

   If you see errors about GLIBC version, ensure you're on Ubuntu 24.04 LTS or newer:

   ```bash
   ldd --version
   ```

### Graphics Driver Issues

For optimal performance, ensure you have proper graphics drivers installed:

```bash
# For NVIDIA
sudo ubuntu-drivers autoinstall

# For AMD
sudo apt install -y mesa-vulkan-drivers

# For Intel
sudo apt install -y intel-media-va-driver
```

### Getting Help

If you encounter issues not covered here:

1. Check the [DropWebP repository](https://github.com/logue/DropWebP) for known issues
2. Review the [Tauri v2 documentation](https://v2.tauri.app/start/prerequisites/) for Linux-specific guidance
3. Search existing GitHub issues or create a new one

## Next Steps

Once you have DropWebP built successfully:

1. **Run Tests**: Execute `pnpm test` to ensure everything works correctly
2. **Development**: Use `pnpm dev:tauri` for development with hot reloading
3. **Customization**: Explore the codebase and make your modifications
4. **Distribution**: Use `pnpm build:tauri` to create distributable packages

You're now ready to develop and build DropWebP on Linux!
