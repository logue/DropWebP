# Windows에서 Docker로 Linux 빌드

이 문서는 Windows 환경에서 Docker를 사용해 Linux 패키지를 빌드하는 방법을 설명합니다.

## 사전 요구 사항

- Windows 10/11 (64-bit)
- Docker Desktop (WSL 2 백엔드 권장)
- PowerShell 5.1 이상
- 메모리 8GB 이상 (16GB 권장)
- 디스크 여유 공간 20GB 이상

## 빌드 명령

프로젝트 루트에서 실행:

```powershell
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
```

직접 스크립트 실행:

```powershell
pwsh .\scripts\build-linux-docker.ps1 -Target x64
pwsh .\scripts\build-linux-docker.ps1 -Target arm64
```

AppImage 포함:

```powershell
pwsh .\scripts\build-linux-docker.ps1 -Target x64 -IncludeAppImage
```

## .env 설정

```bash
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
INCLUDE_APPIMAGE=false
```

## 산출물

```text
app/src-tauri/target/<target>/release/bundle/
```

- `.deb`
- `.rpm`
- `.AppImage` (선택)

## 문제 해결

- Docker Desktop이 실행 중인지 확인
- 메모리 부족 시 Docker 메모리 또는 `.env` 값을 증가
- 빌드가 느리면 CPU/메모리 설정과 캐시 볼륨 확인
