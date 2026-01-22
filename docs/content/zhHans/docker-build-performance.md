# Docker构建性能分析

本文档说明不同平台上Docker构建的性能特征，以及为什么交叉编译构建会很慢。

## 性能比较

### 实际构建时间

| 主机环境 | 目标 | 构建时间 | QEMU模拟 |
|---------|------|---------|---------|
| macOS (x64) | x64 Linux | 8-12分钟 | 不需要 ✅ |
| macOS (Apple Silicon) | x64 Linux | 10-15分钟 | 使用Rosetta 2 |
| Windows (x64) | x64 Linux | 10-15分钟 | 不需要 ✅ |
| Windows (x64) | ARM64 Linux | **30-60分钟** | 需要 ❌ |

### 关键观察

**为什么Windows x64 → ARM64 Linux构建如此缓慢？**

虽然macOS x64 → x64 Linux构建很快，但Windows x64 → ARM64 Linux构建极其缓慢，主要是由于**架构差异**。

## 技术原因

### 1. QEMU模拟开销

#### 相同架构（快速）

```bash
# macOS x64 → x64 Linux
主机：x86_64
目标：x86_64-unknown-linux-gnu
处理：原生指令执行
速度：接近原生速度
```

#### 交叉架构（缓慢）

```bash
# Windows x64 → ARM64 Linux
主机：x86_64
目标：aarch64-unknown-linux-gnu
处理：通过QEMU模拟ARM指令
速度：慢10-50倍 ❌
```

### 2. 文件系统开销

#### Windows的I/O路径

```
C:\Users\...\DropWebP (NTFS)
  ↓ 9P网络协议
/mnt/c/Users/.../DropWebP (WSL 2)
  ↓ Docker bind mount
/workspace (Container)
```

#### macOS的I/O路径

```
/Users/.../DropWebP (APFS)
  ↓ VirtioFS（优化）
/workspace (Container)
```

**VirtioFS**是专为虚拟化环境设计的高速文件共享协议：

- 比9P快2-5倍
- 高效的元数据缓存
- 对大型项目特别有利

## 性能优化策略

### 推荐方法

#### 开发阶段

```powershell
# 仅构建x64 Linux（10-15分钟）
pnpm run build:tauri:linux-x64
```

#### 发布阶段

```yaml
# 在GitHub Actions上并行构建
jobs:
  build-x64:
    runs-on: ubuntu-latest
  build-arm64:
    runs-on: ubuntu-latest
```

## 总结

### 关键要点

✅ **相同架构很快**：x64 → x64需要10-15分钟
❌ **交叉架构很慢**：x64 → ARM64需要30-60分钟
🚀 **实用解决方案**：开发期间仅使用x64，发布使用GitHub Actions

### 开发者建议

```bash
# 日常工作足够使用
pnpm run build:tauri:linux-x64

# 发布自动化
git tag v3.2.1 && git push origin v3.2.1
```

高效使用时间和资源，实现无压力开发！
