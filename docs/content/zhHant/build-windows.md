# 設置開發環境（Windows）

在Windows上為Drop Compress Image設置開發環境的指南。

## 選擇構建方法

Windows上有兩種構建方式：

1. **Docker環境構建（推薦）**：乾淨的環境避免依賴衝突
2. **原生環境構建**：更快但設置更複雜

---

## 方法1：Docker環境構建（推薦）

### 先決條件

- Windows 10/11 Pro、Enterprise或Education（支援Hyper-V）
- Docker Desktop for Windows

### 步驟

1. **安裝Docker Desktop**

   下載並安裝[Docker Desktop](https://www.docker.com/products/docker-desktop)。

2. **切換到Windows容器模式**

   右鍵點擊Docker Desktop托盤圖標，選擇「Switch to Windows containers...」。

3. **克隆專案**

   ```powershell
   git clone https://github.com/logue/DropWebP.git
   cd DropWebP
   ```

4. **構建Docker映像**（僅首次，需要30-60分鐘）

   ```powershell
   docker build -f Dockerfile.windows-x64 -t dropwebp-windows-builder .
   ```

5. **構建應用程式**

   ```powershell
   docker run --rm -v ${PWD}:C:\workspace dropwebp-windows-builder
   ```

6. **檢查構建產物**

   構建成功後，可執行文件和安裝程序將生成在`app/src-tauri/target/release/bundle/`目錄中。

### Docker環境的優勢

- ✅ 保持主機環境乾淨
- ✅ 避免依賴衝突
- ✅ 可重現的構建
- ✅ 乾淨的構建環境
- ✅ 與CI/CD管道保持一致

---

## 方法2：原生環境構建

## 1. 安裝Chocolatey

以管理員身份打開PowerShell並運行以下命令以安裝Chocolatey。

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force;
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```

安裝後，您可以使用以下命令檢查版本。

```powershell
choco -v
```

## 2. 安裝Git

使用Chocolatey安裝Git。

```powershell
choco install git -y
```

安裝後，驗證版本。

```powershell
git --version
```

## 3. 克隆項目

從GitHub克隆項目並導航到項目目錄。

```powershell
git clone https://github.com/logue/DropWebP.git
cd DropWebP
```

## 4. 安裝Visual Studio Community 2022

安裝Visual Studio Community 2022。

```powershell
choco install visualstudio2022community -y
```

接下來，安裝C++桌面開發工作負載。

```powershell
choco install visualstudio2022-workload-nativedesktop -y
```

安裝Clang/LLVM構建工具，這是構建某些圖像編解碼器庫所必需的。

```powershell
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Component.VC.Llvm.Clang --add Microsoft.VisualStudio.Component.VC.Llvm.ClangToolset" -y
```

安裝完成後，您可以使用Visual Studio安裝程序驗證已安裝的組件。

> **注意：** C++桌面開發工作負載包括構建Rust本機擴展所需的工具，例如MSVC（Microsoft的編譯器）、Windows SDK和CMake。

## 5. 安裝NASM和Ninja

安裝NASM和Ninja，這些是構建圖像編解碼器庫所必需的。

```powershell
choco install nasm ninja -y
```

安裝後，驗證版本。

```powershell
nasm -v
ninja --version
```

將NASM添加到系統PATH中，以便Cargo在構建時可以找到它。

```powershell
[System.Environment]::SetEnvironmentVariable('PATH', [System.Environment]::GetEnvironmentVariable('PATH', 'User') + ';C:\Program Files\NASM', 'User')
```

重新啟動終端或PowerShell會話以使PATH更改生效。

> **注意：** NASM（Netwide Assembler）是一種彙編器，用於構建優化的編解碼器庫，如libavif。Ninja是一種快速的構建系統，通常與CMake一起使用。

## 6. 安裝Node.js和pnpm

安裝Node.js和pnpm。

```powershell
choco install nodejs pnpm -y
```

安裝後，驗證版本。

```powershell
node -v
pnpm -v
```

## 7. 安裝Rust（官方方法）

在PowerShell或命令提示符中運行以下命令，使用官方方法安裝Rust。

```powershell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安裝後，驗證版本。

```powershell
rustc --version
```

> **警告：** 雖然可以通過Chocolatey安裝Rust，但它會使用MinGW工具鏈進行安裝，這可能會導致與庫的兼容性問題。

## 8. 設置vcpkg

1. 克隆vcpkg倉庫：

   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
   cd C:\vcpkg
   ```

2. 運行引導腳本：

   ```powershell
   .\bootstrap-vcpkg.bat
   ```

3. 設置環境變數（建議添加到系統環境變數）：

   ```powershell
   $env:VCPKG_ROOT = "C:\vcpkg"
   [System.Environment]::SetEnvironmentVariable('VCPKG_ROOT', 'C:\vcpkg', 'User')
   ```

> **重要：** VCPKG_ROOT環境變數是構建系統定位vcpkg庫所必需的。

## 9. 安裝依賴項

### 創建發布三元組

vcpkg的默認三元組包含調試符號，會導致Rust發布構建出現鏈接錯誤。創建自定義三元組：

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE x64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\x64-windows-static-release.cmake
```

### 安裝依賴項

> **注意（2026年2月更新）**：專案現在在Windows上使用`rav1e`（基於Rust的AV1編碼器）進行AVIF編碼。這樣就不再需要`libaom`和`aom`套件。`rav1e`避免了NASM的多遍最佳化要求，提高了Windows上的建構穩定性。

使用自動安裝腳本（推薦）：

```powershell
cd DropWebP\app\src-tauri
.\setup-vcpkg.ps1
```

或手動安裝：

```powershell
cd C:\vcpkg

# 使用x64-windows-static-release三元組安裝（僅發布版）
# 注意：不再需要aom和libavif[aom]（使用rav1e）
.\vcpkg install libjxl:x64-windows-static-release
.\vcpkg install libwebp:x64-windows-static-release
.\vcpkg install openjpeg:x64-windows-static-release
.\vcpkg install libjpeg-turbo:x64-windows-static-release
.\vcpkg install lcms:x64-windows-static-release
```

已安裝的庫：

- **rav1e**：AV1編碼器（基於Rust，用於AVIF編碼） - 由Cargo自動建構
- **libjxl**：JPEG XL圖像格式
- **libwebp**：WebP圖像格式
- **openjpeg**：JPEG 2000圖像格式
- **libjpeg-turbo**：JPEG圖像處理（用於jpegli）
- **lcms**：Little CMS色彩管理

> **macOS/Linux使用者注意**：由於這些平台上的NASM和CMake設定更加穩定，macOS和Linux仍然可以使用`libaom`。

驗證安裝：

```powershell
.\vcpkg list | Select-String "jxl|webp|openjpeg|jpeg|lcms"
```

## 10. 建構應用程式

1. 導航到 app 目錄並安裝相依性：

   ```powershell
   cd app
   pnpm install
   ```

2. 在開發模式下建構並執行應用程式：

   ```powershell
   pnpm run dev:tauri
   ```

3. 對於生產建構：

   ```powershell
   pnpm run build:tauri
   ```

現在應用程式應該可以在 Windows 上成功建構。如果遇到任何問題，請確保所有相依性都已正確安裝，並且環境變數已正確設置。

---

## Arm64 Windows 交叉編譯

您可以從 x64 Windows 機器交叉編譯 Arm64 Windows（Windows on ARM）。

### 先決條件

- 如上所述設置好的 x64 建構環境
- Arm64 目標的 vcpkg 相依性

### 1. 新增 Rust 工具鏈

```powershell
rustup target add aarch64-pc-windows-msvc
```

### 2. 為 Arm64 安裝 vcpkg 相依性

建立 Arm64 的發布三元組（如果尚未完成）：

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE arm64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\arm64-windows-static-release.cmake
```

安裝相依性：

```powershell
cd C:\vcpkg

# 注意：不再需要aom和libavif[aom]（使用rav1e）
.\vcpkg install libjxl:arm64-windows-static-release
.\vcpkg install libwebp:arm64-windows-static-release
.\vcpkg install openjpeg:arm64-windows-static-release
.\vcpkg install libjpeg-turbo:arm64-windows-static-release
.\vcpkg install lcms:arm64-windows-static-release
```

### 3. 為 Arm64 建構

```powershell
cd path\to\DropWebP\app
pnpm run build:tauri:windows-arm64
```

或手動建構：

```powershell
cd app\src-tauri
cargo build --release --target aarch64-pc-windows-msvc
cd ..
pnpm tauri build --target aarch64-pc-windows-msvc
```

### 注意事項

- Arm64 二進位檔案僅在 Arm64 Windows 設備（例如 Surface Pro X）上執行
- 交叉編譯的二進位檔案無法在 x64 機器上執行
- 建構產物在 `app/src-tauri/target/aarch64-pc-windows-msvc/release/` 中生成
