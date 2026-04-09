# Windows Docker Build Support

## Summary

Linux package builds via Docker are supported from Windows.

## Added Files

- `scripts/build-linux-docker.ps1`
- `docker-build-windows.md`
- `docker-build.md`

## Updated Files

- `app/package.json` (cross-platform Linux build commands)
- root `package.json` (shortcut scripts)
- `.env` (Docker build settings examples)
- `ReadMe.md` (build instructions)

## Usage

```powershell
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
pwsh .\scripts\build-linux-docker.ps1 -Target x64
```

## Notes

- Docker Desktop must be running.
- You can tune CPU/memory settings in `.env`.
- Artifacts are generated under `app/src-tauri/target/<target>/release/bundle/`.
