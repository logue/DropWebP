# 设置开发环境（Windows）

在Windows上为Drop Compress Image设置开发环境的指南。

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

## 8. 设置vcpkg（官方说明）

在所需目录中运行以下命令以克隆vcpkg。

```powershell
git clone https://github.com/microsoft/vcpkg.git
```

导航到vcpkg目录并运行设置命令。

```powershell
cd vcpkg
.\bootstrap-vcpkg.bat
```

将vcpkg.exe的路径（例如`C:\path\to\vcpkg`）添加到环境变量`PATH`中。方法：将vcpkg目录添加到系统环境变量中的"Path"。

此外，添加环境变量`VCPKG_DEFAULT_TRIPLET`并将其设置为`x64-windows-static-md`。这确保默认安装适用于64位Windows的静态库。

安装后，验证路径是否设置，并使用以下命令检查版本。

```powershell
vcpkg version
```

> **警告：** 如果vcpkg的路径包含非字母数字字符，可能无法正常工作。建议将其克隆到驱动器根目录等位置。

## 9. 安装所需库

运行以下命令以安装图像转换所需的库。

```powershell
vcpkg install libavif libjxl libwebp libjpeg-turbo libpng
```

> **注意：** JPEG XL (`libjxl`) 通过 `vendored` 功能静态链接，因此将来可能不需要 vcpkg。安装可能需要一些时间，并且某些库可能会构建失败。在这种情况下，请参阅vcpkg文档。

## 10. 安装依赖项并构建

在项目目录中安装依赖项。

```powershell
pnpm install
```

以开发模式启动应用程序。

```powershell
cd app
pnpm tauri dev
```

> **完成：** 第一次启动可能需要一些时间来编译Rust依赖项。一旦应用程序窗口出现，开发环境设置就完成了。
