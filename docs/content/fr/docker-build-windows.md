# Build Linux avec Docker depuis Windows

Ce guide explique comment effectuer un build Linux depuis un environnement Windows en utilisant Docker.

## Prerequis

### 1. Installer Docker Desktop

1. Telechargez et installez [Docker Desktop for Windows](https://www.docker.com/products/docker-desktop/).
2. Lancez Docker Desktop et attendez qu'il soit completement demarre.
3. Verifiez que l'option "Use WSL 2 based engine" est activee dans les parametres (recommande).

### 2. Configuration systeme

- **Windows 10/11** (64-bit)
- **WSL 2** (recommande)
- **Memoire**: minimum 8 Go de RAM (16 Go recommandes)
- **Espace disque libre**: minimum 20 Go

### 3. Version PowerShell

PowerShell 5.1 ou version ulterieure est requis (fourni par defaut sur Windows 10/11).

```powershell
# Verifier la version de PowerShell
$PSVersionTable.PSVersion
```

## Procedure de build

### Build de base

Depuis la racine du projet, executez l'une des commandes suivantes:

```powershell
# Linux x86_64 (AMD64)
pnpm run build:tauri:linux-x64

# Linux ARM64
pnpm run build:tauri:linux-arm64
```

Vous pouvez aussi executer le script directement:

```powershell
# Build x86_64
pwsh .\scripts\build-linux-docker.ps1 -Target x64

# Build ARM64
pwsh .\scripts\build-linux-docker.ps1 -Target arm64
```

### Build avec AppImage

Par defaut, seuls les paquets `.deb` et `.rpm` sont generes. Pour generer aussi une AppImage:

```powershell
pwsh .\scripts\build-linux-docker.ps1 -Target x64 -IncludeAppImage
```

> **Remarque**: la generation AppImage necessite FUSE, qui peut etre limite en environnement Docker.

## Personnaliser les parametres de build

Vous pouvez personnaliser les parametres via le fichier `.env`:

```bash
# Docker Build Settings
BUILD_CPUS=4              # Nombre de coeurs CPU a utiliser
BUILD_MEMORY=8g           # Limite memoire
CARGO_BUILD_JOBS=4        # Nombre de jobs Cargo en parallele
MAKEFLAGS=-j4             # Niveau de parallelisme Make
INCLUDE_APPIMAGE=false    # Inclure ou non AppImage
```

## Artefacts de build

Une fois le build termine, les artefacts sont generes ici:

```
app/src-tauri/target/<target>/release/bundle/
├── deb/
│   └── drop-compress-image_<version>_<arch>.deb
└── rpm/
    └── drop-compress-image-<version>-1.<arch>.rpm
```

Exemples:

- `app/src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/deb/`
- `app/src-tauri/target/aarch64-unknown-linux-gnu/release/bundle/deb/`

## Depannage

### Docker Desktop n'est pas demarre

```
❌ Erreur: Docker Desktop n'est pas demarre.
```

**Solution**: demarrez Docker Desktop, puis relancez la commande.

### Erreur de memoire insuffisante

```
error: linking with `cc` failed: exit status: 1
```

**Solution**: augmentez la limite memoire dans le fichier `.env` ou dans Docker Desktop:

1. Docker Desktop -> Settings -> Resources -> Memory
2. Augmentez le curseur memoire (recommande: 8 Go ou plus)
3. Cliquez sur Apply & Restart

### Build trop lent

**Solution**: augmentez le nombre de coeurs CPU et la memoire.

```bash
# A ajouter dans le fichier .env
BUILD_CPUS=8
BUILD_MEMORY=16g
CARGO_BUILD_JOBS=8
```

### Backend WSL 2 desactive

**Solution**:

1. Docker Desktop -> Settings -> General
2. Cochez "Use the WSL 2 based engine"
3. Cliquez sur Apply & Restart

### Nettoyer le cache de build

Supprimez les volumes Docker:

```powershell
docker volume rm dropwebp-cargo-cache-linux-amd64
docker volume rm dropwebp-pnpm-cache-linux-amd64
docker volume rm dropwebp-target-cache-linux-amd64
```

Pour ARM64:

```powershell
docker volume rm dropwebp-cargo-cache-linux-arm64
docker volume rm dropwebp-pnpm-cache-linux-arm64
docker volume rm dropwebp-target-cache-linux-arm64
```

## Utilisation avancee

### Build multiplateforme

Depuis Windows, vous pouvez builder les deux architectures:

```powershell
# Build x86_64 et ARM64
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
```

### Dockerfile personnalise

Pour utiliser un Dockerfile personnalise:

```powershell
docker build -f YourDockerfile.linux-x64 -t your-builder .
```

### Mode debug

Pour activer des logs detailles:

```powershell
$env:VERBOSE = "1"
pwsh .\scripts\build-linux-docker.ps1 -Target x64
```

## Optimisation des performances

### Parametres recommandes (PC performant)

```bash
# Fichier .env
BUILD_CPUS=12
BUILD_MEMORY=16g
CARGO_BUILD_JOBS=12
MAKEFLAGS=-j12
```

### Parametres recommandes (PC standard)

```bash
# Fichier .env
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
```

## Ressources

- [Docker Desktop for Windows](https://docs.docker.com/desktop/windows/)
- [WSL 2](https://docs.microsoft.com/en-us/windows/wsl/install)
- [Tauri Documentation](https://tauri.app/v1/guides/building/)

## Documents lies

- [DOCKER_BUILD.md](docker-build.md) - Procedure de build sur macOS/Linux
- [WINDOWS_BUILD_VCPKG.md](./WINDOWS_BUILD_VCPKG.md) - Build natif Windows
- [MACOS_COMPATIBILITY.md](./MACOS_COMPATIBILITY.md) - Informations de compatibilite macOS
