# Build Linux avec Docker

Ce guide explique comment construire des paquets Linux de DropWebP avec Docker depuis Windows, macOS et Linux.

## Plateformes supportees

| OS hote               | Build x86_64 | Build ARM64 |
| --------------------- | ------------ | ----------- |
| Windows               | Oui          | Oui         |
| macOS (Intel)         | Oui          | Oui         |
| macOS (Apple Silicon) | Oui          | Oui         |
| Linux                 | Oui          | Oui         |

## Demarrage rapide

### Windows

```powershell
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
```

Voir aussi: [docker-build-windows.md](docker-build-windows.md)

### macOS / Linux

```bash
bash scripts/build-linux-docker.sh x64
bash scripts/build-linux-docker.sh arm64
```

## Prerequis

- Docker Desktop ou Docker Engine
- pnpm 10.2.0 ou plus
- 8 Go de RAM minimum (16 Go recommandes)
- 20 Go d'espace disque libre minimum

### Prerequis specifiques

#### Windows

- Windows 10/11 (64-bit)
- WSL 2 recommande
- PowerShell 5.1 ou plus

#### macOS

- macOS 10.15 ou plus
- Bash

#### Linux

- Distribution Linux 64-bit
- Docker Engine 20.10 ou plus
- Bash

## Variables de build (.env)

```bash
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
INCLUDE_APPIMAGE=false
```

## Resultats de build

Les artefacts sont generes dans:

```text
app/src-tauri/target/<target>/release/bundle/
```

Types de paquets:

- `.deb`
- `.rpm`
- `.AppImage` (si active)

## Nettoyage du cache Docker

```powershell
docker volume rm dropwebp-cargo-cache-linux-amd64
docker volume rm dropwebp-pnpm-cache-linux-amd64
docker volume rm dropwebp-target-cache-linux-amd64

docker volume rm dropwebp-cargo-cache-linux-arm64
docker volume rm dropwebp-pnpm-cache-linux-arm64
docker volume rm dropwebp-target-cache-linux-arm64
```

## Depannage

### Docker introuvable

- Verifier que Docker est installe
- Verifier que Docker est demarre
- Verifier le PATH

### Memoire insuffisante

- Augmenter `BUILD_MEMORY`
- Augmenter la memoire allouee dans Docker Desktop
- Reduire `BUILD_CPUS`

### Build lent

- Augmenter CPU/memoire
- Utiliser un SSD
- Conserver les volumes de cache

### Erreur AppImage

- FUSE peut etre limite sous Docker
- Activer explicitement `INCLUDE_APPIMAGE=true` si necessaire

## Liens utiles

- [docker-build-windows.md](docker-build-windows.md)
- [Tauri Documentation](https://tauri.app/v1/guides/building/)
- [Docker Documentation](https://docs.docker.com/)

## FAQ

### Peut-on builder sans Docker ?

Oui, sur une machine Linux native:

```bash
cd app
pnpm install
pnpm run build:tauri:linux-x64
```

### Ou sont les paquets ?

Dans `app/src-tauri/target/<target>/release/bundle/`.
