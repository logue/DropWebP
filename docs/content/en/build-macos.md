# Build Drop Compress Image for macOS

This guide walks you through setting up the development environment and building Drop Compress Image on macOS systems.

## Prerequisites

Before you begin, make sure you have:

- macOS 10.15 (Catalina) or later
- Administrator privileges for installing software
- Basic familiarity with Terminal commands

## Step 1: Install Xcode Command Line Tools

First, install the Xcode Command Line Tools which provide essential development tools including `clang` and `make`:

```bash
xcode-select --install
```

This will open a dialog asking if you want to install the command line developer tools. Click **Install** and wait for the installation to complete.

### Verify Installation

Check that the tools are installed correctly:

```bash
clang --version
```

You should see output similar to:

```
Apple clang version 15.0.0 (clang-1500.0.40.1)
Target: arm64-apple-darwin23.0.0
Thread model: posix
```

## Step 2: Install Homebrew

Homebrew is a package manager for macOS that makes it easy to install development tools and libraries.

### Install Homebrew

Open Terminal and run:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### Add Homebrew to PATH

For Apple Silicon Macs (M1/M2/M3), add Homebrew to your PATH:

```bash
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zshrc
source ~/.zshrc
```

For Intel Macs, Homebrew is installed to `/usr/local` and should already be in your PATH.

### Verify Homebrew Installation

```bash
brew --version
```

## Step 3: Install Rust

Drop Compress Image is built with Rust, so you'll need to install the Rust toolchain.

### Install Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When prompted, choose option 1 (default installation).

### Configure Your Shell

```bash
source ~/.cargo/env
```

### Verify Rust Installation

```bash
rustc --version
cargo --version
```

You should see version information for both `rustc` and `cargo`.

## Step 4: Install Node.js

The frontend of Drop Compress Image is built with Vue.js and requires Node.js.

### Install Node.js via Homebrew

```bash
brew install node
```

### Verify Node.js Installation

```bash
node --version
npm --version
```

## Step 5: Install pnpm

Drop Compress Image uses pnpm as its package manager for better performance and disk efficiency.

### Install pnpm

```bash
npm install -g pnpm
```

### Verify pnpm Installation

```bash
pnpm --version
```

## Step 6: Set Up vcpkg and Install Dependencies

This project uses vcpkg to manage C/C++ image processing libraries (libaom, libavif, libjxl, etc.).

### Install vcpkg

```bash
# Clone vcpkg
git clone https://github.com/Microsoft/vcpkg.git ~/Developer/vcpkg

# Bootstrap vcpkg
cd ~/Developer/vcpkg
./bootstrap-vcpkg.sh

# Set environment variables (add to ~/.zshrc)
echo 'export VCPKG_ROOT="$HOME/Developer/vcpkg"' >> ~/.zshrc
echo 'export PATH="$VCPKG_ROOT:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Install Dependencies

Use the automated installation script (recommended):

```bash
cd ~/path/to/DropWebP/app/src-tauri
./setup-vcpkg.sh
```

Or install manually:

```bash
cd ~/Developer/vcpkg

# For Apple Silicon (M1/M2/M3)
./vcpkg install aom:arm64-osx
./vcpkg install libavif[aom]:arm64-osx
./vcpkg install libjxl:arm64-osx
./vcpkg install libwebp:arm64-osx
./vcpkg install openjpeg:arm64-osx
./vcpkg install libjpeg-turbo:arm64-osx
./vcpkg install lcms:arm64-osx

# For Intel Mac
./vcpkg install aom:x64-osx
./vcpkg install libavif[aom]:x64-osx
./vcpkg install libjxl:x64-osx
./vcpkg install libwebp:x64-osx
./vcpkg install openjpeg:x64-osx
./vcpkg install libjpeg-turbo:x64-osx
./vcpkg install lcms:x64-osx
```

Installed libraries:

- **libaom**: AV1 encoder (for AVIF format, **required**)
- **libavif**: AVIF image format
- **libjxl**: JPEG XL image format
- **libwebp**: WebP image format
- **openjpeg**: JPEG 2000 image format
- **libjpeg-turbo**: JPEG image processing (for jpegli)
- **lcms**: Little CMS color management

### Verify Installation

```bash
./vcpkg list | grep -E "aom|avif|jxl|webp|openjpeg|jpeg|lcms"
```

## Step 7: Clone and Build Drop Compress Image

Now you're ready to clone and build Drop Compress Image.

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

## Step 8: Platform-Specific Considerations

### Apple Silicon (M1/M2/M3) Macs

If you're using an Apple Silicon Mac, some dependencies might need to be compiled specifically for the `arm64` architecture. Most modern packages handle this automatically, but if you encounter issues:

```bash
# Check your architecture
uname -m
# Should output: arm64

# If needed, you can force Rust to build for the correct target
rustup target add aarch64-apple-darwin
```

### Intel Macs

For Intel Macs, the default `x86_64` target should work without issues:

```bash
# Check your architecture
uname -m
# Should output: x86_64

# Ensure the correct Rust target is installed
rustup target add x86_64-apple-darwin
```

### Code Signing (Optional)

If you want to distribute your built application, you'll need to sign it with an Apple Developer certificate:

```bash
# Check available signing identities
security find-identity -v -p codesigning

# If you have a developer certificate, Tauri can sign automatically
# Add this to your tauri.conf.json:
{
  "bundle": {
    "macOS": {
      "signing": {
        "identity": "Developer ID Application: Your Name (TEAM_ID)"
      }
    }
  }
}
```

## Troubleshooting

### Common Issues

1. **Permission Denied Errors**

   ```bash
   # Fix permissions for Homebrew
   sudo chown -R $(whoami) /opt/homebrew
   ```

2. **Command Not Found After Installation**

   ```bash
   # Reload your shell profile
   source ~/.zshrc
   # Or restart your terminal
   ```

3. **Build Failures with Native Dependencies**

   ```bash
   # Clear build caches
   cargo clean
   pnpm clean

   # Rebuild everything
   pnpm install
   pnpm tauri build
   ```

4. **Rust Target Issues**

   ```bash
   # List installed targets
   rustup target list --installed

   # Add the correct target for your system
   rustup target add aarch64-apple-darwin  # Apple Silicon
   rustup target add x86_64-apple-darwin   # Intel
   ```

### Getting Help

If you encounter issues not covered here:

1. Check the [Drop Compress Image repository](https://github.com/logue/DropWebP) for known issues
2. Review the [Tauri v2 documentation](https://v2.tauri.app/start/prerequisites/) for macOS-specific guidance
3. Search existing GitHub issues or create a new one

## Next Steps

Once you have Drop Compress Image built successfully:

1. **Run Tests**: Execute `pnpm test` to ensure everything works correctly
2. **Development**: Use `pnpm dev:tauri` for development with hot reloading
3. **Customization**: Explore the codebase and make your modifications
4. **Distribution**: Use `pnpm build:tauri` to create distributable packages

You're now ready to develop and build Drop Compress Image on macOS!

## Building for Intel Mac

If you want to cross-compile for Intel Mac (x86_64) from an Apple Silicon Mac (M1/M2/M3), follow these steps.

### Method 1: Universal Binary (Recommended)

Create a single binary that works on both Intel and Apple Silicon Macs:

```bash
cd app
pnpm run build:tauri:mac-universal
```

**Advantages:**

- No additional library installation required
- Single binary supports both architectures
- Users don't need to worry about their Mac's architecture

**Disadvantages:**

- File size is approximately doubled (contains code for both architectures)

### Method 2: Intel-only Build

If you want to create an Intel Mac-specific binary, you'll need x86_64 libraries.

#### Step 1: Install x86_64 Homebrew

```bash
arch -x86_64 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

#### Step 2: Install x86_64 Libraries

```bash
arch -x86_64 /usr/local/bin/brew install libavif jpeg-xl
```

Or use the setup script:

```bash
./scripts/setup-x86-libs.sh
```

#### Step 3: Build

```bash
cd app
pnpm run build:tauri:mac-x64
```

### Build Target Overview

```bash
# Apple Silicon only
pnpm run build:tauri:mac-arm64

# Intel Mac only
pnpm run build:tauri:mac-x64

# Universal Binary (both)
pnpm run build:tauri:mac-universal
```

### Build Artifacts Location

```text
app/src-tauri/target/
  ├── aarch64-apple-darwin/release/bundle/      # ARM64 only
  ├── x86_64-apple-darwin/release/bundle/       # x86_64 only
  └── universal-apple-darwin/release/bundle/    # Universal (both)
```
