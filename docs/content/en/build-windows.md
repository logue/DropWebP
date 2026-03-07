# Windows Build Environment Setup

This guide walks you through setting up the development environment for building Tauri Vue3 App on Windows.

## Choose Your Build Method

There are three ways to build on Windows:

1. **Docker for Linux Build (Recommended)**: Build Linux packages from Windows
2. **Docker Environment for Windows Build**: Clean environment avoiding dependency conflicts
3. **Native Environment**: Faster but more complex setup

---

## Method 1: Docker for Linux Build (Recommended)

You can build Linux packages (.deb, .rpm) from Windows using Docker.

### Prerequisites

- Windows 10/11 (64-bit)
- Docker Desktop for Windows
- WSL 2 (recommended)
- PowerShell 5.1 or higher
- 8GB+ RAM (16GB recommended)

### Steps

1. **Install Docker Desktop**

   Download and install [Docker Desktop for Windows](https://www.docker.com/products/docker-desktop/).

2. **Enable WSL 2**

   Enable "Use WSL 2 based engine" in Docker Desktop settings (recommended).

3. **Clone the Project**

   ```powershell
   git clone https://github.com/logue/tauri-vuetify-starter.git
   cd tauri-vuetify-starter
   ```

4. **Build Linux Packages**

   ```powershell
   # For x86_64 Linux
   pnpm run build:tauri:linux-x64

   # For ARM64 Linux
   pnpm run build:tauri:linux-arm64

   # Or run the script directly
   pwsh .\scripts\build-linux-docker.ps1 -Target x64
   ```

5. **Check Build Artifacts**

   Upon successful build, packages will be generated at:
   - `backend/target/x86_64-unknown-linux-gnu/release/bundle/deb/`
   - `backend/target/x86_64-unknown-linux-gnu/release/bundle/rpm/`

### Linux Build Benefits

- ✅ Build Linux packages directly from Windows
- ✅ Consistency with CI/CD pipelines
- ✅ Easy cross-platform development
- ✅ Keeps host environment clean

> **More Info**: See [Building for Linux (Using Docker)](./build-linux-docker.md) for details.

---

## Method 2: Docker Environment for Windows Build

### Prerequisites

- Windows 10/11 Pro, Enterprise, or Education (with Hyper-V support)
- Docker Desktop for Windows

### Steps

1. **Install Docker Desktop**

   Download and install [Docker Desktop](https://www.docker.com/products/docker-desktop).

2. **Switch to Windows Container Mode**

   Right-click the Docker Desktop tray icon and select "Switch to Windows containers...".

3. **Clone the Project**

   ```powershell
   git clone https://github.com/logue/tauri-vuetify-starter.git
   cd tauri-vuetify-starter
   ```

4. **Build Docker Image** (first time only, takes 30-60 minutes)

   ```powershell
   docker build -f Dockerfile.windows-x64 -t tauri-vue3-windows-builder .
   ```

5. **Build the Application**

   ```powershell
   docker run --rm -v ${PWD}:C:\workspace tauri-vue3-windows-builder
   ```

6. **Check Build Artifacts**

   Upon successful build, executables and installers will be generated in the `backend/target/release/bundle/` directory.

### Docker Environment Benefits

- ✅ Keeps host environment clean
- ✅ Avoids dependency conflicts
- ✅ Reproducible builds
- ✅ Clean build environment
- ✅ Consistency with CI/CD pipelines

---

## Method 3: Native Environment Build

## 1. Install Chocolatey

1. Install Chocolatey package manager by running the following command in PowerShell as Administrator:

   ```powershell
   Set-ExecutionPolicy Bypass -Scope Process -Force;
   [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;
   iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
   ```

2. After installation, verify the version:

   ```powershell
   choco -v
   ```

## 2. Install Git

1. Install Git using Chocolatey:

   ```powershell
   choco install git -y
   ```

2. After installation, verify the version:

   ```powershell
   git --version
   ```

## 3. Clone the Project

1. Clone the project from GitHub and navigate to the project directory:

   ```powershell
   git clone https://github.com/logue/tauri-vuetify-starter.git
   cd tauri-vuetify-starter
   ```

## 4. Install Visual Studio Community 2022

1. Install Visual Studio Community 2022:

   ```powershell
   choco install visualstudio2022community -y
   ```

2. Install the C++ Desktop Development workload:

   ```powershell
   choco install visualstudio2022-workload-nativedesktop -y
   ```

3. Install Clang/LLVM build tools, which are required for building certain image codec libraries:

   ```powershell
   choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Component.VC.Llvm.Clang --add Microsoft.VisualStudio.Component.VC.Llvm.ClangToolset" -y
   ```

4. Once installation is complete, you can verify the installed components using the Visual Studio Installer.

> **Notice:** The C++ Desktop Development workload includes tools necessary for building Rust native extensions, such as MSVC (Microsoft's compiler), Windows SDK, and CMake.

## 5. Install NASM and Ninja

1. Install NASM and Ninja, which are required for building image codec libraries:

   ```powershell
   choco install nasm ninja -y
   ```

2. After installation, verify the versions:

   ```powershell
   nasm -v
   ninja --version
   ```

3. Add NASM to your system PATH so that Cargo can find it during build time:

   ```powershell
   [System.Environment]::SetEnvironmentVariable('PATH', [System.Environment]::GetEnvironmentVariable('PATH', 'User') + ';C:\Program Files\NASM', 'User')
   ```

4. Restart your terminal or PowerShell session for the PATH changes to take effect.

> **Notice:** NASM is an assembler used for building optimized codec libraries like libavif. Ninja is a fast build system often used in conjunction with CMake.

## 6. Install Node.js and pnpm

1. Install Node.js and pnpm:

   ```powershell
   choco install nodejs pnpm -y
   ```

2. After installation, verify the versions:

   ```powershell
   node -v
   pnpm -v
   ```

## 7. Install Rust (Official Method)

1. Install Rust using the official method by running the following command in PowerShell or Command Prompt:

   ```powershell
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. After installation, verify the version:

   ```powershell
   rustc --version
   ```

> **Warning:** While it's possible to install Rust via Chocolatey, it installs with the MinGW toolchain, which may lead to compatibility issues with libraries.

## 8. Set Up vcpkg

1. Clone the vcpkg repository:

   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
   cd C:\vcpkg
   ```

2. Run the bootstrap script:

   ```powershell
   .\bootstrap-vcpkg.bat
   ```

3. Set environment variables (recommended to add to system environment variables):

   ```powershell
   $env:VCPKG_ROOT = "C:\vcpkg"
   [System.Environment]::SetEnvironmentVariable('VCPKG_ROOT', 'C:\vcpkg', 'User')
   ```

> **Important:** The VCPKG_ROOT environment variable is required for the build system to locate vcpkg libraries.

## 9. Install Dependencies

### Create Release Triplet

vcpkg's default triplet includes debug symbols which cause link errors with Rust release builds. Create a custom triplet:

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE x64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\x64-windows-static-release.cmake
```

### Install Dependencies

> **Note:** `backend/setup-vcpkg.ps1` is a static-link setup template. Edit the install target(s) in the script to link any libraries you need.

Use the automated installation script (recommended):

```powershell
cd tauri-vuetify-starter\backend
.\setup-vcpkg.ps1
```

Or install manually:

```powershell
cd C:\vcpkg

# Example install with x64-windows-static-release triplet (release-only)
.\vcpkg install <package>:x64-windows-static-release
```

Installed libraries depend on what you define in `backend/setup-vcpkg.ps1`.

Verify installation:

```powershell
.\vcpkg list | Select-String "jxl|webp|openjpeg|jpeg|lcms"
```

## 10. Build the Application

1. Navigate to the app directory and install dependencies:

   ```powershell
   cd frontend
   pnpm install
   ```

2. Build and run the application in development mode:

   ```powershell
   pnpm run dev:tauri
   ```

3. For a production build:

   ```powershell
   pnpm run build:tauri
   ```

The application should now build successfully on Windows. If you encounter any issues, ensure all dependencies are properly installed and environment variables are set correctly.

---

## Cross-Building for Arm64 Windows

You can cross-build for Arm64 Windows (Windows on ARM) from an x64 Windows machine.

### Prerequisites

- x64 Windows build environment set up as described above
- vcpkg dependencies for Arm64 target

### 1. Add Rust Toolchain

```powershell
rustup target add aarch64-pc-windows-msvc
```

### 2. Install vcpkg Dependencies for Arm64

Create release triplet for Arm64 (if not already done):

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE arm64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\arm64-windows-static-release.cmake
```

Install dependencies:

```powershell
cd C:\vcpkg

# Example install with arm64-windows-static-release triplet
.\vcpkg install <package>:arm64-windows-static-release
```

### 3. Build for Arm64

```powershell
cd path\to\tauri-vuetify-starter\app
pnpm run build:tauri:windows-arm64
```

Or build manually:

```powershell
cd backend
cargo build --release --target aarch64-pc-windows-msvc
cd ..
pnpm tauri build --target aarch64-pc-windows-msvc
```

### Notes

- Arm64 binaries will only run on Arm64 Windows devices (e.g., Surface Pro X)
- Cross-built binaries cannot be executed on x64 machines
- Build artifacts are generated in `backend/target/aarch64-pc-windows-msvc/release/`
