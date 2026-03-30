# Docker Build Performance Analysis

This document explains the performance characteristics of Docker builds on different platforms and why cross-compilation builds can be slow.

## Performance Comparison

### Actual Build Times

| Host Environment | Target | Build Time | QEMU Emulation |
|-----------------|--------|-----------|----------------|
| macOS (x64) | x64 Linux | 8-12 min | Not required ✅ |
| macOS (Apple Silicon) | x64 Linux | 10-15 min | Rosetta 2 used |
| Windows (x64) | x64 Linux | 10-15 min | Not required ✅ |
| Windows (x64) | ARM64 Linux | **30-60 min** | Required ❌ |

### Key Observation

**Why is Windows x64 → ARM64 Linux build so slow?**

While macOS x64 → x64 Linux builds are fast, Windows x64 → ARM64 Linux builds are extremely slow primarily due to **architectural differences**.

## Technical Reasons

### 1. QEMU Emulation Overhead

#### Same Architecture (Fast)

```bash
# macOS x64 → x64 Linux
Host: x86_64
Target: x86_64-unknown-linux-gnu
Processing: Native instruction execution
Speed: Near-native speed
```

When CPU architectures match, binary instructions can execute natively. Docker containers only abstract Linux kernel differences while CPU instructions run directly.

#### Cross-Architecture (Slow)

```bash
# Windows x64 → ARM64 Linux
Host: x86_64
Target: aarch64-unknown-linux-gnu
Processing: ARM instructions emulated via QEMU
Speed: 10-50x slower ❌
```

For different architectures, all ARM instructions must be translated to x86_64 instructions.

**QEMU Overhead:**

```dockerfile
# QEMU used in Dockerfile
RUN apt-get install -y qemu-user-static

# During build, all ARM instructions are translated
# ARM: ADD R0, R1, R2
#   ↓ QEMU translation
# x86: mov eax, [r1]
#      add eax, [r2]
#      mov [r0], eax
```

Such translations occur **millions of times**, causing extreme slowdown.

### 2. Docker Backend Differences

#### macOS Docker Desktop

```yaml
Virtualization:
  - Apple Virtualization Framework (macOS 13+)
  - QEMU + HVF (Hypervisor Framework)

Optimizations:
  - Native virtualization support
  - Efficient memory management
  - VirtioFS (fast file sharing)

Filesystem:
  macOS APFS → VirtioFS → Linux ext4
  Overhead: Low
```

#### Windows Docker Desktop + WSL 2

```yaml
Virtualization:
  - WSL 2 (Linux VM on Hyper-V)
  - Docker Engine in WSL 2

Layer Structure:
  Windows (NTFS)
    ↓ 9P protocol (slow)
  WSL 2 (ext4 in VM)
    ↓ Docker bind mount
  Container (ext4)

Overhead: High
```

### 3. Filesystem Overhead

#### I/O Path on Windows

```
C:\Users\...\DropWebP (NTFS)
  ↓ 9P network protocol
/mnt/c/Users/.../DropWebP (WSL 2)
  ↓ Docker bind mount
/workspace (Container)
```

**9P protocol** shares filesystems over network. Compared to local filesystems:

- **Many small file operations**: 10-100x slower
- **Metadata operations**: Especially slow
- **Rust compilation**: Tens of thousands of small file I/O operations

#### I/O Path on macOS

```
/Users/.../DropWebP (APFS)
  ↓ VirtioFS (optimized)
/workspace (Container)
```

**VirtioFS** is a high-speed file sharing protocol designed specifically for virtualized environments:

- 2-5x faster than 9P
- Efficient metadata caching
- Particularly advantageous for large projects

### 4. Rust Build Characteristics

```rust
// Drop Compress Image dependencies
dependencies {
  libavif,      // AVIF encoder
  jpegxl-rs,    // JPEG XL encoder
  libwebp-sys,  // WebP encoder
  oxipng,       // PNG optimizer
  jpegli,       // Fast JPEG encoder
  // ... many native libraries
}
```

When cross-compiling these native libraries for ARM:

1. **C compilation**: Generate ARM binaries with gcc
2. **Assembly**: ARM-optimized code with NASM/YASM
3. **Static linking**: Combine all libraries into one binary
4. **LTO**: Link-time optimization

All steps execute via QEMU, causing cumulative slowdown.

## Performance Optimization Strategies

### Recommended Approach

#### Development Phase

```powershell
# Build only x64 Linux (10-15 min)
pnpm run build:tauri:linux-x64
```

Reasons:
- No QEMU emulation needed
- Minimal file I/O
- Sufficient for debugging and testing

#### Release Phase

```yaml
# Parallel builds on GitHub Actions
jobs:
  build-x64:
    runs-on: ubuntu-latest
  build-arm64:
    runs-on: ubuntu-latest
```

Reasons:
- High-performance GitHub infrastructure
- Time savings through parallelization
- Zero load on local machine

### Advanced Optimizations

#### Option 1: Build Inside WSL 2

```bash
# Enter WSL 2 from Windows
wsl

# Build from WSL native path
cd ~/DropWebP  # ← Not /mnt/c/, but WSL native
bash scripts/build-linux-docker.sh arm64
```

**Effect**: Avoid 9P protocol overhead. **20-30% faster**.

#### Option 2: Docker Volume Utilization

```powershell
# Copy source to Volume
docker volume create dropwebp-source
docker run --rm `
    -v "${PWD}:/host:ro" `
    -v dropwebp-source:/workspace `
    alpine cp -r /host/. /workspace/

# Build from Volume (ultra-fast I/O)
docker run --rm `
    -v dropwebp-source:/workspace `
    # ...
```

**Effect**: All I/O stays within VM. **30-40% faster**.

#### Option 3: sccache for Compilation Cache

```bash
# Rust compilation cache tool
cargo install sccache

# Environment variable setup
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR=/cache/sccache
```

**Effect**: **50-80% faster** on rebuilds.

#### Option 4: Staged Builds

```dockerfile
# Dockerfile.deps
FROM base-image

# Build dependencies only
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --target aarch64-unknown-linux-gnu

# Main build only for changes
FROM deps-image
COPY src/ ./src/
RUN cargo build --release
```

**Effect**: After initial build, **60-70% faster**.

## Build Environment Selection Guide

### Scenario-Based Recommendations

| Scenario | Recommended Environment | Reason |
|----------|------------------------|--------|
| Daily development/testing | Windows x64 → x64 Linux | Fast, sufficient |
| Pre-release testing | GitHub Actions | Parallel, fast |
| Emergency release | Build in WSL 2 | Self-contained locally |
| CI/CD | GitHub Actions | Automated, parallelized |

### Practical Development Flow

```powershell
# 1. Daily development (x64 only)
pnpm run build:tauri:linux-x64

# 2. When creating pull request
#    → GitHub Actions automatically builds all platforms

# 3. When creating release tag
git tag v3.2.1
git push origin v3.2.1
#    → GitHub Actions generates artifacts
```

## Technical Notes

### Why is macOS Fast?

1. **Rosetta 2 Optimization** (Apple Silicon)
   - Fast execution of x86_64 binaries
   - JIT compilation for optimization

2. **Integrated Virtualization**
   - Kernel-level optimization
   - Apple controls entire stack

3. **VirtioFS**
   - Optimized for macOS
   - Integration with Metal API

### Windows Constraints

1. **WSL 2 Architecture**
   - Full Linux VM on Hyper-V
   - File sharing over network

2. **9P Protocol**
   - Generic protocol (insufficient optimization)
   - Weak integration with Windows kernel

3. **QEMU**
   - Software emulation
   - No hardware acceleration

## Summary

### Key Points

✅ **Same architecture is fast**: x64 → x64 takes 10-15 min
❌ **Cross-architecture is slow**: x64 → ARM64 takes 30-60 min
🚀 **Practical solution**: x64 only during development, GitHub Actions for releases

### Recommendation for Developers

```bash
# This is sufficient for daily work
pnpm run build:tauri:linux-x64

# Automate for releases
git tag v3.2.1 && git push origin v3.2.1
```

Use time and resources efficiently for stress-free development!
