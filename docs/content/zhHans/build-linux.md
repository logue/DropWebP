# 为 Linux 构建 Drop Compress Image

本指南将引导您在 Ubuntu 24.04 LTS（和类似的基于 Debian 的发行版）上设置开发环境并构建 Drop Compress Image。

## 前提条件

开始之前，请确保您有：

- Ubuntu 24.04 LTS 或类似的基于 Debian 的发行版
- 安装软件的 sudo 权限
- 对终端命令的基本了解

## 步骤 1：更新系统包

首先，更新系统包以确保您有最新版本：

```bash
sudo apt update
sudo apt upgrade -y
```

## 步骤 2：安装构建依赖项

安装 Tauri 开发所需的基本构建工具和库：

```bash
# 安装构建必需品和开发库
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

### 这些包的作用

- **build-essential**: 提供 GCC、G++ 和 make
- **libssl-dev**: OpenSSL 开发库
- **libgtk-3-dev**: UI 的 GTK3 开发库
- **libayatana-appindicator3-dev**: 系统托盘支持
- **librsvg2-dev**: SVG 渲染支持
- **libwebkit2gtk-4.1-dev**: Tauri 的 webview 用 WebKit
- **patchelf**: AppImage 的 ELF 二进制补丁程序

### 验证安装

```bash
gcc --version
```

您应该看到显示 GCC 版本 13.x 或更高版本的输出。

## 步骤 3：安装 Rust

Drop Compress Image 使用 Rust 构建，因此您需要安装 Rust 工具链。

### 通过 rustup 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

当提示时，选择选项 1（默认安装）。

### 配置您的 Shell

```bash
source $HOME/.cargo/env
```

要使其永久化，请将其添加到您的 shell 配置文件：

```bash
echo 'source $HOME/.cargo/env' >> ~/.bashrc
source ~/.bashrc
```

### 验证 Rust 安装

```bash
rustc --version
cargo --version
```

您应该看到 `rustc` 和 `cargo` 的版本信息。

## 步骤 4：安装 Node.js

Drop Compress Image 的前端使用 Vue.js 构建，需要 Node.js。

### 通过 NodeSource 存储库安装 Node.js

```bash
# 安装 Node.js 22.x (LTS)
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt install -y nodejs
```

### 验证 Node.js 安装

```bash
node --version
npm --version
```

您应该看到 Node.js 版本 22.x 或更高版本。

## 步骤 5：安装 pnpm

Drop Compress Image 使用 pnpm 作为包管理器，以获得更好的性能和磁盘效率。

### 安装 pnpm

```bash
npm install -g pnpm
```

### 验证 pnpm 安装

```bash
pnpm --version
```

## 步骤 6：设置 vcpkg 并安装依赖项

此项目使用 vcpkg 管理 C/C++ 图像处理库（libaom、libavif、libjxl 等）。

### 安装 vcpkg 前提条件

```bash
# 安装 vcpkg 所需的工具
sudo apt install -y curl zip unzip tar cmake pkg-config
```

### 安装 vcpkg

```bash
# 克隆 vcpkg
git clone https://github.com/Microsoft/vcpkg.git ~/vcpkg

# 引导 vcpkg
cd ~/vcpkg
./bootstrap-vcpkg.sh

# 设置环境变量（添加到 ~/.bashrc）
echo 'export VCPKG_ROOT="$HOME/vcpkg"' >> ~/.bashrc
echo 'export PATH="$VCPKG_ROOT:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 安装依赖项

使用自动安装脚本（推荐）：

```bash
cd ~/path/to/DropWebP/app/src-tauri
./setup-vcpkg.sh
```

或手动安装：

```bash
cd ~/vcpkg

# x64 Linux 的情况
./vcpkg install aom:x64-linux
./vcpkg install libavif[aom]:x64-linux
./vcpkg install libjxl:x64-linux
./vcpkg install libwebp:x64-linux
./vcpkg install openjpeg:x64-linux
./vcpkg install libjpeg-turbo:x64-linux
./vcpkg install lcms:x64-linux

# ARM64 Linux 的情况
./vcpkg install aom:arm64-linux
./vcpkg install libavif[aom]:arm64-linux
./vcpkg install libjxl:arm64-linux
./vcpkg install libwebp:arm64-linux
./vcpkg install openjpeg:arm64-linux
./vcpkg install libjpeg-turbo:arm64-linux
./vcpkg install lcms:arm64-linux
```

安装的库：

- **libaom**：AV1 编码器（用于 AVIF 格式，**必需**）
- **libavif**：AVIF 图像格式
- **libjxl**：JPEG XL 图像格式
- **libwebp**：WebP 图像格式
- **openjpeg**：JPEG 2000 图像格式
- **libjpeg-turbo**：JPEG 图像处理（用于 jpegli）
- **lcms**：Little CMS 色彩管理

### 验证安装

```bash
./vcpkg list | grep -E "aom|avif|jxl|webp|openjpeg|jpeg|lcms"
```

## 步骤 7：克隆和构建 Drop Compress Image

现在您已准备好克隆和构建 Drop Compress Image。

### 克隆存储库

```bash
git clone https://github.com/logue/DropWebP.git
cd DropWebP
```

### 安装前端依赖项

```bash
# 安装所有工作区依赖项
pnpm install
```

### 安装 Tauri CLI v2

```bash
# 全局安装 Tauri CLI v2
pnpm add -g @tauri-apps/cli@next
```

### 构建应用程序

开发模式：

```bash
# 在开发模式下运行
pnpm dev:tauri
```

生产模式：

```bash
# 为生产构建
pnpm build:tauri
```

构建的应用程序将在 `app/src-tauri/target/release/` 中。

## 步骤 8：分发格式

Linux 上的 Tauri 可以生成多种分发格式：

### AppImage（推荐）

AppImage 是一种通用包格式，可在大多数 Linux 发行版上运行：

```bash
pnpm build:tauri
```

AppImage 将在 `app/src-tauri/target/release/bundle/appimage/` 中。

### Debian 包 (.deb)

对于基于 Debian/Ubuntu 的发行版：

```bash
pnpm build:tauri
```

.deb 包将在 `app/src-tauri/target/release/bundle/deb/` 中。

安装它：

```bash
sudo dpkg -i app/src-tauri/target/release/bundle/deb/*.deb
```

### RPM 包 (.rpm)

对于基于 Red Hat/Fedora 的发行版，您需要安装额外的工具：

```bash
sudo apt install -y rpm
pnpm build:tauri
```

.rpm 包将在 `app/src-tauri/target/release/bundle/rpm/` 中。

## 故障排除

### 常见问题

1. **缺少 libwebkit2gtk-4.1**

   如果您收到有关缺少 webkit 库的错误：

   ```bash
   # 尝试旧版 webkit
   sudo apt install -y libwebkit2gtk-4.0-dev
   ```

2. **npm/pnpm 权限被拒绝**

   ```bash
   # 修复 npm 全局目录权限
   mkdir -p ~/.npm-global
   npm config set prefix '~/.npm-global'
   echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **原生依赖项构建失败**

   ```bash
   # 清理构建缓存
   cargo clean
   pnpm clean

   # 重新构建所有内容
   pnpm install
   pnpm build:tauri
   ```

4. **AppImage 不可执行**

   ```bash
   # 使 AppImage 可执行
   chmod +x app/src-tauri/target/release/bundle/appimage/*.AppImage
   ```

5. **缺少 GLIBC 版本**

   如果您看到有关 GLIBC 版本的错误，请确保您使用的是 Ubuntu 24.04 LTS 或更新版本：

   ```bash
   ldd --version
   ```

### 图形驱动程序问题

为了获得最佳性能，请确保已安装适当的图形驱动程序：

```bash
# 对于 NVIDIA
sudo ubuntu-drivers autoinstall

# 对于 AMD
sudo apt install -y mesa-vulkan-drivers

# 对于 Intel
sudo apt install -y intel-media-va-driver
```

### 获取帮助

如果您遇到此处未涵盖的问题：

1. 检查 [Drop Compress Image 存储库](https://github.com/logue/DropWebP) 的已知问题
2. 查看 [Tauri v2 文档](https://v2.tauri.app/start/prerequisites/) 以获取 Linux 特定指导
3. 搜索现有的 GitHub 问题或创建新问题

## 下一步

成功构建 Drop Compress Image 后：

1. **运行测试**：执行 `pnpm test` 确保一切正常工作
2. **开发**：使用 `pnpm dev:tauri` 进行热重载开发
3. **自定义**：探索代码库并进行修改
4. **分发**：使用 `pnpm build:tauri` 创建可分发的包

您现在已准备好在 Linux 上开发和构建 Drop Compress Image！
