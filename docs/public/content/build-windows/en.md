# Windows Build Environment Setup

This guide walks you through setting up the development environment for building DropWebP on Windows.

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
   git clone https://github.com/logue/DropWebP.git
   cd DropWebP
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

## 8. Set Up vcpkg (Official Instructions)

1. Clone the vcpkg repository:

   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git
   cd vcpkg
   ```

2. Run the bootstrap script:

   ```powershell
   .\bootstrap-vcpkg.bat
   ```

3. Integrate vcpkg with Visual Studio (this step sets environment variables like VCPKG_ROOT):

   ```powershell
   .\vcpkg integrate install
   ```

> **Warning:** You can also set VCPKG_ROOT environment variable to the vcpkg installation directory (e.g., C:\vcpkg) and add vcpkg to your PATH if needed.

## 9. Install Required Libraries

1. Install the necessary libraries for image processing:

   ```powershell
   .\vcpkg install libavif libjxl libheif --triplet x64-windows-static-md
   ```

2. After installation, you can verify that the libraries are installed correctly:

   ```powershell
   .\vcpkg list
   ```

> **Notice:** The x64-windows-static-md triplet ensures compatibility with Rust's default MSVC runtime.

## 10. Build the Application

1. Navigate to the app directory and install dependencies:

   ```powershell
   cd app
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
