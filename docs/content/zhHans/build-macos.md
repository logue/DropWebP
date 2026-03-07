# 为 macOS 构建 Tauri Vue3 App

本指南将引导您在 macOS 系统上设置开发环境并构建 Tauri Vue3 App。

## 前提条件

开始之前，请确保您有：

- macOS 10.15 (Catalina) 或更高版本
- 安装软件的管理员权限
- 对终端命令的基本了解

## 步骤 1：安装 Xcode Command Line Tools

首先，安装 Xcode Command Line Tools，它提供包括 `clang` 和 `make` 在内的基本开发工具：

```bash
xcode-select --install
```

这将打开一个对话框，询问您是否要安装命令行开发工具。点击 **安装** 并等待安装完成。

### 验证安装

检查工具是否正确安装：

```bash
clang --version
```

您应该看到类似的输出：

```text
Apple clang version 15.0.0 (clang-1500.0.40.1)
Target: arm64-apple-darwin23.0.0
Thread model: posix
```

## 步骤 2：安装 Homebrew

Homebrew 是 macOS 的包管理器，使开发工具和库的安装变得容易。

### 安装 Homebrew

打开终端并运行：

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 将 Homebrew 添加到 PATH

对于 Apple Silicon Mac (M1/M2/M3)，将 Homebrew 添加到您的 PATH：

```bash
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zshrc
source ~/.zshrc
```

对于 Intel Mac，Homebrew 安装在 `/usr/local` 并且应该已经在您的 PATH 中。

### 验证 Homebrew 安装

```bash
brew --version
```

## 步骤 3：安装 Rust

Tauri Vue3 App 使用 Rust 构建，因此您需要安装 Rust 工具链。

### 通过 rustup 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

当提示时，选择选项 1（默认安装）。

### 配置您的 Shell

```bash
source ~/.cargo/env
```

### 验证 Rust 安装

```bash
rustc --version
cargo --version
```

您应该看到 `rustc` 和 `cargo` 的版本信息。

## 步骤 4：安装 Node.js

Tauri Vue3 App 的前端使用 Vue.js 构建，需要 Node.js。

### 通过 Homebrew 安装 Node.js

```bash
brew install node
```

### 验证 Node.js 安装

```bash
node --version
npm --version
```

## 步骤 5：安装 pnpm

Tauri Vue3 App 使用 pnpm 作为包管理器，以获得更好的性能和磁盘效率。

### 安装 pnpm

```bash
brew install pnpm
```

### 验证 pnpm 安装

```bash
pnpm --version
```

## 步骤 6：设置 vcpkg 并安装依赖项

此项目使用 vcpkg 对 C/C++ 库进行静态链接。请编辑 `backend/setup-vcpkg.sh` 来定义你需要的任意库。

### 安装 vcpkg

```bash
# 克隆 vcpkg
git clone https://github.com/Microsoft/vcpkg.git ~/Developer/vcpkg

# 引导 vcpkg
cd ~/Developer/vcpkg
./bootstrap-vcpkg.sh

# 设置环境变量（添加到 ~/.zshrc）
echo 'export VCPKG_ROOT="$HOME/Developer/vcpkg"' >> ~/.zshrc
echo 'export PATH="$VCPKG_ROOT:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### 安装依赖项

使用自动安装脚本（推荐）：

```bash
cd ~/path/to/tauri-vuetify-starter/backend
./setup-vcpkg.sh
```

或手动安装：

```bash
cd ~/Developer/vcpkg

# Apple Silicon (M1/M2/M3) 示例
./vcpkg install <package>:arm64-osx

# Intel Mac 示例
./vcpkg install <package>:x64-osx
```

安装哪些库取决于你在 `backend/setup-vcpkg.sh` 中的定义。

### 验证安装

```bash
./vcpkg list
```

## 步骤 7：克隆和构建 Tauri Vue3 App

现在您已准备好克隆和构建 Tauri Vue3 App。

### 克隆存储库

```bash
git clone https://github.com/logue/tauri-vuetify-starter.git
cd tauri-vuetify-starter
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

## 步骤 8：平台特定注意事项

### Apple Silicon (M1/M2/M3) Mac

如果您使用 Apple Silicon Mac，某些依赖项可能需要专门为 `arm64` 架构编译。大多数现代包都会自动处理这个问题，但如果遇到问题：

```bash
# 检查您的架构
uname -m
# 应该输出：arm64

# 如果需要，您可以强制 Rust 为正确的目标构建
rustup target add aarch64-apple-darwin
```

### Intel Mac

对于 Intel Mac，默认的 `x86_64` 目标应该可以正常工作：

```bash
# 检查您的架构
uname -m
# 应该输出：x86_64

# 确保安装了正确的 Rust 目标
rustup target add x86_64-apple-darwin
```

### 代码签名（可选）

如果您想分发构建的应用程序，您需要使用 Apple Developer 证书进行签名：

```bash
# 检查可用的签名身份
security find-identity -v -p codesigning

# 如果您有开发者证书，Tauri 可以自动签名
# 将此添加到您的 tauri.conf.json：
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

## 故障排除

### 常见问题

1. **权限被拒绝错误**

   ```bash
   # 修复 Homebrew 权限
   sudo chown -R $(whoami) /opt/homebrew
   ```

2. **安装后找不到命令**

   ```bash
   # 重新加载您的 shell 配置文件
   source ~/.zshrc
   # 或重新启动您的终端
   ```

3. **原生依赖项构建失败**

   ```bash
   # 清理构建缓存
   cargo clean
   pnpm clean

   # 重新构建所有内容
   pnpm install
   pnpm tauri build
   ```

4. **Rust 目标问题**

   ```bash
   # 列出已安装的目标
   rustup target list --installed

   # 为您的系统添加正确的目标
   rustup target add aarch64-apple-darwin  # Apple Silicon
   rustup target add x86_64-apple-darwin   # Intel
   ```

### 为 Intel Mac 构建

您可以在 Apple Silicon Mac 上为 Intel Mac 构建二进制文件，或直接在 Intel Mac 上构建。

#### 方法 1：Universal Binary（推荐）

最简单的方法是构建同时包含 ARM64 和 x86_64 二进制文件的 Universal Binary：

```bash
pnpm run build:tauri:mac-universal
```

此方法不需要安装额外的库，并生成可在所有 Mac 上运行的单个二进制文件。

#### 方法 2：仅 Intel 构建

如果您只需要 Intel 专用二进制文件：

**在 Apple Silicon Mac 上交叉编译：**

1. 安装 x86_64 Homebrew 和所需库：

   ```bash
   # 如果尚未安装 x86_64 Homebrew
   arch -x86_64 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

   # 安装 x86_64 库
   arch -x86_64 /usr/local/bin/brew install libavif jpeg-xl
   ```

   或使用提供的脚本：

   ```bash
   bash scripts/setup-x86-libs.sh
   ```

2. 为 x86_64 目标构建：

   ```bash
   pnpm run build:tauri:mac-x64
   ```

**在 Intel Mac 上构建：**

在 Intel Mac 上可以直接构建：

```bash
pnpm run build:tauri:mac-x64
```

#### 构建目标概览

| 命令                        | 架构                       | 平台          |
| --------------------------- | -------------------------- | ------------- |
| `build:tauri:mac-arm64`     | ARM64                      | Apple Silicon |
| `build:tauri:mac-x64`       | x86_64                     | Intel Mac     |
| `build:tauri:mac-universal` | Universal (ARM64 + x86_64) | 所有 Mac      |

#### 构建产物位置

构建产物根据目标生成在以下位置：

```
backend/target/
├── aarch64-apple-darwin/release/   # ARM64 构建
│   └── bundle/
├── x86_64-apple-darwin/release/    # Intel 构建
│   └── bundle/
└── universal-apple-darwin/release/ # Universal 构建
    └── bundle/
```

### 获取帮助

如果您遇到此处未涵盖的问题：

1. 检查 [Tauri Vue3 App 存储库](https://github.com/logue/tauri-vuetify-starter) 的已知问题
2. 查看 [Tauri v2 文档](https://v2.tauri.app/start/prerequisites/) 以获取 macOS 特定指导
3. 搜索现有的 GitHub 问题或创建新问题

## 下一步

成功构建 Tauri Vue3 App 后：

1. **运行测试**：执行 `pnpm test` 确保一切正常工作
2. **开发**：使用 `pnpm tauri dev` 进行热重载开发
3. **自定义**：探索代码库并进行修改
4. **分发**：使用 `pnpm tauri build` 创建可分发的包

您现在已准备好在 macOS 上开发和构建 Tauri Vue3 App！
