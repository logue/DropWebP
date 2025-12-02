# Linux构建（使用Docker）

如何在macOS上使用Docker构建Linux二进制文件

## 📋 前置要求

- 必须安装Docker Desktop for Mac
- 足够的磁盘空间（初次构建需要约5GB）

## 🚀 使用方法

### 构建x86_64 (AMD64)版本

```bash
# 从项目根目录运行
./scripts/build-linux-docker.sh x64

# 或从app目录运行
pnpm run build:tauri:linux-docker-x64
```

### 构建ARM64 (AArch64)版本

```bash
# 从项目根目录运行
./scripts/build-linux-docker.sh arm64

# 或从app目录运行
pnpm run build:tauri:linux-docker-arm64
```

## 📦 构建产物

构建产物将生成在以下目录中：

```text
app/src-tauri/target/
  ├── x86_64-unknown-linux-gnu/release/bundle/
  │   ├── deb/           # Debian/Ubuntu软件包
  │   ├── rpm/           # Red Hat/Fedora软件包
  │   └── appimage/      # AppImage（推荐用于分发）
  │
  └── aarch64-unknown-linux-gnu/release/bundle/
      ├── deb/
      ├── rpm/
      └── appimage/
```

## ⚙️ 工作原理

1. 从`Dockerfile.linux-build`构建Docker镜像
   - 基于Rust 1.83 + Debian Bookworm
   - 安装Tauri依赖项（WebKit2GTK、GTK3等）
   - 安装Node.js 22.x和pnpm

2. 在Docker容器内运行Tauri构建
   - 挂载项目目录
   - 使用指定的目标架构进行构建

3. 将产物输出到macOS目录

## 🔧 故障排除

### 重新构建Docker镜像

```bash
docker build -f Dockerfile.linux-build -t dropwebp-linux-builder --no-cache .
```

### 删除Docker镜像

```bash
docker rmi dropwebp-linux-builder
```

### 清除构建缓存

```bash
rm -rf app/src-tauri/target/x86_64-unknown-linux-gnu
rm -rf app/src-tauri/target/aarch64-unknown-linux-gnu
```

## 📝 注意事项

- 初次构建由于Docker镜像构建和下载需要较长时间（20-30分钟）
- 后续构建会重用Docker镜像，速度较快（10-15分钟）
- ARM64构建可能比x86_64构建需要更长时间

## 🎯 推荐的分发格式

- **AppImage**：推荐用于分发（适用于所有Linux发行版）
- **.deb**：适用于Debian/Ubuntu用户
- **.rpm**：适用于Red Hat/Fedora用户
