# Windows Docker Linux 빌드 지원

## 요약

Windows에서 Docker를 통해 Linux 패키지(.deb, .rpm) 빌드가 가능합니다.

## 사용 방법

```powershell
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
pwsh .\scripts\build-linux-docker.ps1 -Target x64
```

## 참고

- Docker Desktop 실행 필요
- `.env`에서 CPU/메모리 튜닝 가능
- 결과물 경로: `app/src-tauri/target/<target>/release/bundle/`
