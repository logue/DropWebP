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

## 8. 設置vcpkg（官方說明）

在所需目錄中運行以下命令以克隆vcpkg。

```powershell
git clone https://github.com/microsoft/vcpkg.git
```

導航到vcpkg目錄並運行設置命令。

```powershell
cd vcpkg
.\bootstrap-vcpkg.bat
```

將vcpkg.exe的路徑（例如`C:\path\to\vcpkg`）添加到環境變量`PATH`中。方法：將vcpkg目錄添加到系統環境變量中的"Path"。

此外，添加環境變數`VCPKG_DEFAULT_TRIPLET`並將其設置為`x64-windows-static-md`。這確保默認安裝適用於64位Windows的靜態庫。

安裝後，驗證路徑是否設置，並使用以下命令檢查版本。

```powershell
vcpkg version
```

> **警告：** 如果vcpkg的路徑包含非字母數字字符，可能無法正常工作。建議將其克隆到驅動器根目錄等位置。

## 9. 安裝所需庫

運行以下命令以安裝圖像轉換所需的庫。

```powershell
vcpkg install libavif libjxl libwebp libjpeg-turbo libpng
```

> **注意：** JPEG XL (`libjxl`) 通過 `vendored` 功能靜態鏈接，因此將來可能不需要 vcpkg。安裝可能需要一些時間，並且某些庫可能會構建失敗。在這種情況下，請參閱vcpkg文檔。

## 10. 安裝依賴項並構建

在項目目錄中安裝依賴項。

```powershell
pnpm install
```

以開發模式啟動應用程序。

```powershell
cd app
pnpm tauri dev
```

> **完成：** 第一次啟動可能需要一些時間來編譯Rust依賴項。一旦應用程序窗口出現，開發環境設置就完成了。
