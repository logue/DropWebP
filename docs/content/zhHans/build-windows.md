# 设置开发环境（Windows）

在Windows上为Drop Compress Image设置开发环境的指南。

## 选择构建方法

Windows上有两种构建方式：

1. **Docker环境构建（推荐）**：干净的环境避免依赖冲突
2. **原生环境构建**：更快但设置更复杂

---

## 方法1：Docker环境构建（推荐）

### 先决条件

- Windows 10/11 Pro、Enterprise或Education（支持Hyper-V）
- Docker Desktop for Windows

### 步骤

1. **安装Docker Desktop**

   下载并安装[Docker Desktop](https://www.docker.com/products/docker-desktop)。

2. **切换到Windows容器模式**

   右键点击Docker Desktop托盘图标，选择"Switch to Windows containers..."。

3. **克隆项目**

   ```powershell
   git clone https://github.com/logue/DropWebP.git
   cd DropWebP
   ```

4. **构建Docker镜像**（仅首次，需要30-60分钟）

   ```powershell
   docker build -f Dockerfile.windows-x64 -t dropwebp-windows-builder .
   ```

5. **构建应用程序**

   ```powershell
   docker run --rm -v ${PWD}:C:\workspace dropwebp-windows-builder
   ```

6. **检查构建产物**

   构建成功后，可执行文件和安装程序将生成在`app/src-tauri/target/release/bundle/`目录中。

### Docker环境的优势

- ✅ 保持主机环境干净
- ✅ 避免依赖冲突
- ✅ 可重现的构建
- ✅ 干净的构建环境
- ✅ 与CI/CD管道保持一致

---

## 方法2：原生环境构建

## 1. 安装Chocolatey

以管理员身份打开PowerShell并运行以下命令以安装Chocolatey。

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force;
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```

安装后，您可以使用以下命令检查版本。

```powershell
choco -v
```

## 2. 安装Git

使用Chocolatey安装Git。

```powershell
choco install git -y
```

安装后，验证版本。

```powershell
git --version
```

## 3. 克隆项目

从GitHub克隆项目并导航到项目目录。

```powershell
git clone https://github.com/logue/DropWebP.git
cd DropWebP
```

## 4. 安装Visual Studio Community 2022

安装Visual Studio Community 2022。

```powershell
choco install visualstudio2022community -y
```

接下来，安装C++桌面开发工作负载。

```powershell
choco install visualstudio2022-workload-nativedesktop -y
```

安装Clang/LLVM构建工具，这是构建某些图像编解码器库所必需的。

```powershell
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Component.VC.Llvm.Clang --add Microsoft.VisualStudio.Component.VC.Llvm.ClangToolset" -y
```

安装完成后，您可以使用Visual Studio安装程序验证已安装的组件。

> **注意：** C++桌面开发工作负载包括构建Rust本机扩展所需的工具，例如MSVC（Microsoft的编译器）、Windows SDK和CMake。

## 5. 安装NASM和Ninja

安装NASM和Ninja，这些是构建图像编解码器库所必需的。

```powershell
choco install nasm ninja -y
```

安装后，验证版本。

```powershell
nasm -v
ninja --version
```

将NASM添加到系统PATH中，以便Cargo在构建时可以找到它。

```powershell
[System.Environment]::SetEnvironmentVariable('PATH', [System.Environment]::GetEnvironmentVariable('PATH', 'User') + ';C:\Program Files\NASM', 'User')
```

重新启动终端或PowerShell会话以使PATH更改生效。

> **注意：** NASM（Netwide Assembler）是一种汇编器，用于构建优化的编解码器库，如libavif。Ninja是一种快速的构建系统，通常与CMake一起使用。

## 6. 安装Node.js和pnpm

安装Node.js和pnpm。

```powershell
choco install nodejs pnpm -y
```

安装后，验证版本。

```powershell
node -v
pnpm -v
```

## 7. 安装Rust（官方方法）

在PowerShell或命令提示符中运行以下命令，使用官方方法安装Rust。

```powershell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装后，验证版本。

```powershell
rustc --version
```

> **警告：** 虽然可以通过Chocolatey安装Rust，但它会使用MinGW工具链进行安装，这可能会导致与库的兼容性问题。

## 8. 设置vcpkg

1. 克隆vcpkg仓库：

   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
   cd C:\vcpkg
   ```

2. 运行引导脚本：

   ```powershell
   .\bootstrap-vcpkg.bat
   ```

3. 设置环境变量（建议添加到系统环境变量）：

   ```powershell
   $env:VCPKG_ROOT = "C:\vcpkg"
   [System.Environment]::SetEnvironmentVariable('VCPKG_ROOT', 'C:\vcpkg', 'User')
   ```

> **重要：** VCPKG_ROOT环境变量是构建系统定位vcpkg库所必需的。

## 9. 安装依赖项

### 创建发布三元组

vcpkg的默认三元组包含调试符号，会导致Rust发布构建出现链接错误。创建自定义三元组：

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE x64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\x64-windows-static-release.cmake
```

### 安装依赖项

> **注意（2026年2月更新）**：项目现在在Windows上使用`rav1e`（基于Rust的AV1编码器）进行AVIF编码。这样就不再需要`libaom`和`aom`包。`rav1e`避免了NASM的多遍优化要求，提高了Windows上的构建稳定性。

使用自动安装脚本（推荐）：

```powershell
cd DropWebP\app\src-tauri
.\setup-vcpkg.ps1
```

或手动安装：

```powershell
cd C:\vcpkg

# 使用x64-windows-static-release三元组安装（仅发布版）
# 注意：不再需要aom和libavif[aom]（使用rav1e）
.\vcpkg install libjxl:x64-windows-static-release
.\vcpkg install libwebp:x64-windows-static-release
.\vcpkg install openjpeg:x64-windows-static-release
.\vcpkg install libjpeg-turbo:x64-windows-static-release
.\vcpkg install lcms:x64-windows-static-release
```

已安装的库：

- **rav1e**：AV1编码器（基于Rust，用于AVIF编码） - 由Cargo自动构建
- **libjxl**：JPEG XL图像格式
- **libwebp**：WebP图像格式
- **openjpeg**：JPEG 2000图像格式
- **libjpeg-turbo**：JPEG图像处理（用于jpegli）
- **lcms**：Little CMS色彩管理

> **macOS/Linux用户注意**：由于这些平台上的NASM和CMake配置更加稳定，macOS和Linux仍然可以使用`libaom`。

验证安装：

```powershell
.\vcpkg list | Select-String "jxl|webp|openjpeg|jpeg|lcms"
```

## 10. 构建应用程序

1. 导航到 app 目录并安装依赖项：

   ```powershell
   cd app
   pnpm install
   ```

2. 在开发模式下构建并运行应用程序：

   ```powershell
   pnpm run dev:tauri
   ```

3. 对于生产构建：

   ```powershell
   pnpm run build:tauri
   ```

现在应用程序应该可以在 Windows 上成功构建。如果遇到任何问题，请确保所有依赖项都已正确安装，并且环境变量已正确设置。

---

## Arm64 Windows 交叉编译

您可以从 x64 Windows 机器交叉编译 Arm64 Windows（Windows on ARM）。

### 前提条件

- 如上所述设置好的 x64 构建环境
- Arm64 目标的 vcpkg 依赖项

### 1. 添加 Rust 工具链

```powershell
rustup target add aarch64-pc-windows-msvc
```

### 2. 为 Arm64 安装 vcpkg 依赖项

创建 Arm64 的发布三元组（如果尚未完成）：

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE arm64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\arm64-windows-static-release.cmake
```

安装依赖项：

```powershell
cd C:\vcpkg

# 注意：不再需要aom和libavif[aom]（使用rav1e）
.\vcpkg install libjxl:arm64-windows-static-release
.\vcpkg install libwebp:arm64-windows-static-release
.\vcpkg install openjpeg:arm64-windows-static-release
.\vcpkg install libjpeg-turbo:arm64-windows-static-release
.\vcpkg install lcms:arm64-windows-static-release
```

### 3. 为 Arm64 构建

```powershell
cd path\to\DropWebP\app
pnpm run build:tauri:windows-arm64
```

或手动构建：

```powershell
cd app\src-tauri
cargo build --release --target aarch64-pc-windows-msvc
cd ..
pnpm tauri build --target aarch64-pc-windows-msvc
```

### 注意事项

- Arm64 二进制文件仅在 Arm64 Windows 设备（例如 Surface Pro X）上运行
- 交叉编译的二进制文件无法在 x64 机器上执行
- 构建产物在 `app/src-tauri/target/aarch64-pc-windows-msvc/release/` 中生成
