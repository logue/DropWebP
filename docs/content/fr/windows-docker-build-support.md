# Support du build Linux avec Docker depuis Windows

## Resume

Le projet supporte maintenant le build de paquets Linux (.deb et .rpm) depuis Windows via Docker.

## Fichiers ajoutes

### `scripts/build-linux-docker.ps1`

Script PowerShell pour Windows avec:

- Verification de Docker Desktop
- Support des cibles x86_64 et ARM64
- Parametrage CPU/memoire/parallelisme
- Gestion de cache Docker
- Option AppImage

### `docker-build-windows.md`

Guide detaille pour Windows:

- Prerequis
- Commandes de build
- Depannage
- Optimisation des performances

### `docker-build.md`

Guide global multi-plateforme:

- Plateformes supportees
- Processus de build
- Gestion du cache
- FAQ

## Fichiers modifies

### `app/package.json`

Les scripts Linux Docker sont multiplateforme:

```json
"build:tauri:linux-docker-x64": "node -e \"require('child_process').execSync(process.platform === 'win32' ? 'pwsh ..\\\\scripts\\\\build-linux-docker.ps1 -Target x64' : 'bash ../scripts/build-linux-docker.sh x64', {stdio: 'inherit'})\"",
"build:tauri:linux-docker-arm64": "node -e \"require('child_process').execSync(process.platform === 'win32' ? 'pwsh ..\\\\scripts\\\\build-linux-docker.ps1 -Target arm64' : 'bash ../scripts/build-linux-docker.sh arm64', {stdio: 'inherit'})\""
```

### `package.json` (racine)

Ajout de raccourcis:

```json
"build:tauri:linux-x64": "pnpm --filter app build:tauri:linux-docker-x64",
"build:tauri:linux-arm64": "pnpm --filter app build:tauri:linux-docker-arm64"
```

### `.env`

Variables de build Docker documentees:

```bash
# BUILD_CPUS=4
# BUILD_MEMORY=8g
# CARGO_BUILD_JOBS=4
# MAKEFLAGS=-j4
# INCLUDE_APPIMAGE=false
```

## Utilisation

### Windows

```powershell
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64

pwsh .\scripts\build-linux-docker.ps1 -Target x64
pwsh .\scripts\build-linux-docker.ps1 -Target arm64 -IncludeAppImage
```

### macOS/Linux

```bash
bash scripts/build-linux-docker.sh x64
bash scripts/build-linux-docker.sh arm64
```

## Prerequis

### Windows

- Windows 10/11 (64-bit)
- Docker Desktop
- WSL 2 recommande
- 8 Go RAM minimum (16 Go recommandes)
- PowerShell 5.1+

### macOS/Linux

- Docker Desktop ou Docker Engine
- 8 Go RAM minimum (16 Go recommandes)
- Bash

## Resultats

Les artefacts sont dans:

```text
app/src-tauri/target/<target>/release/bundle/
```

## Depannage

### Docker Desktop non demarre

```text
❌ Erreur: Docker Desktop n'est pas demarre.
```

Lancer Docker Desktop puis relancer la commande.

### Memoire insuffisante

Augmenter la memoire Docker ou la valeur `BUILD_MEMORY` dans `.env`.

## Liens

- [docker-build-windows.md](docker-build-windows.md)
- [docker-build.md](docker-build.md)
- [ReadMe.md](../../../ReadMe.md)
