# Docker建置效能分析

本文件說明不同平台上Docker建置的效能特徵，以及為什麼交叉編譯建置會很慢。

## 效能比較

### 實際建置時間

| 主機環境 | 目標 | 建置時間 | QEMU模擬 |
|---------|------|---------|---------|
| macOS (x64) | x64 Linux | 8-12分鐘 | 不需要 ✅ |
| macOS (Apple Silicon) | x64 Linux | 10-15分鐘 | 使用Rosetta 2 |
| Windows (x64) | x64 Linux | 10-15分鐘 | 不需要 ✅ |
| Windows (x64) | ARM64 Linux | **30-60分鐘** | 需要 ❌ |

### 關鍵觀察

**為什麼Windows x64 → ARM64 Linux建置如此緩慢？**

雖然macOS x64 → x64 Linux建置很快，但Windows x64 → ARM64 Linux建置極其緩慢，主要是由於**架構差異**。

## 技術原因

### 1. QEMU模擬開銷

#### 相同架構（快速）

```bash
# macOS x64 → x64 Linux
主機：x86_64
目標：x86_64-unknown-linux-gnu
處理：原生指令執行
速度：接近原生速度
```

#### 交叉架構（緩慢）

```bash
# Windows x64 → ARM64 Linux
主機：x86_64
目標：aarch64-unknown-linux-gnu
處理：透過QEMU模擬ARM指令
速度：慢10-50倍 ❌
```

### 2. 檔案系統開銷

#### Windows的I/O路徑

```
C:\Users\...\DropWebP (NTFS)
  ↓ 9P網路協定
/mnt/c/Users/.../DropWebP (WSL 2)
  ↓ Docker bind mount
/workspace (Container)
```

#### macOS的I/O路徑

```
/Users/.../DropWebP (APFS)
  ↓ VirtioFS（最佳化）
/workspace (Container)
```

**VirtioFS**是專為虛擬化環境設計的高速檔案共享協定：

- 比9P快2-5倍
- 高效的中繼資料快取
- 對大型專案特別有利

## 效能最佳化策略

### 推薦方法

#### 開發階段

```powershell
# 僅建置x64 Linux（10-15分鐘）
pnpm run build:tauri:linux-x64
```

#### 發布階段

```yaml
# 在GitHub Actions上平行建置
jobs:
  build-x64:
    runs-on: ubuntu-latest
  build-arm64:
    runs-on: ubuntu-latest
```

## 總結

### 關鍵要點

✅ **相同架構很快**：x64 → x64需要10-15分鐘
❌ **交叉架構很慢**：x64 → ARM64需要30-60分鐘
🚀 **實用解決方案**：開發期間僅使用x64，發布使用GitHub Actions

### 開發者建議

```bash
# 日常工作足夠使用
pnpm run build:tauri:linux-x64

# 發布自動化
git tag v3.2.1 && git push origin v3.2.1
```

高效使用時間和資源，實現無壓力開發！
