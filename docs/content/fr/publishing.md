# Publication des paquets

Ce guide explique comment publier Tauri Vue3 App sur Chocolatey et Homebrew.

## Prérequis

### Chocolatey (Windows)

- Installer Chocolatey : <https://chocolatey.org/install>
- Créer un compte sur <https://community.chocolatey.org/>
- Obtenir votre clé API depuis la page de votre compte

### Homebrew (macOS)

- Créer un dépôt GitHub pour votre Homebrew tap (par exemple, `homebrew-tap`)
- Générer un GitHub Personal Access Token avec le scope `repo`

## Construction des paquets

### 1. Construire l'application Tauri

```bash
pnpm build:tauri
```

Cela créera des installateurs spécifiques à chaque plateforme dans :

- Windows : `backend/target/release/bundle/msi/`
- macOS : `backend/target/release/bundle/dmg/`
- Linux : `backend/target/release/bundle/deb/` ou `appimage/`

### 2. Générer le paquet Chocolatey (Windows)

```powershell
pnpm package:chocolatey
```

Ou manuellement :

```powershell
.\scripts\build-chocolatey.ps1 -Version {VERSION}
```

Cela va :

- Calculer le checksum SHA256 du fichier MSI
- Mettre à jour `chocolateyinstall.ps1` avec le bon checksum
- Créer `.choco/{APP_NAME_KEBAB}.{VERSION}.nupkg`

### 3. Générer la formule Homebrew (macOS)

```bash
pnpm package:homebrew
```

Ou manuellement :

```bash
./scripts/build-homebrew.sh {VERSION}
```

Cela va :

- Calculer les checksums SHA256 pour les fichiers DMG ARM64 et x64
- Mettre à jour `.homebrew/{APP_NAME_KEBAB}.rb` avec les checksums

## Publication

### Chocolatey

1. **Tester d'abord localement :**

```powershell
choco install {APP_NAME_KEBAB} -source .choco
```

2. **Pousser vers le dépôt communautaire Chocolatey :**

```powershell
choco apikey --key YOUR-API-KEY --source https://push.chocolatey.org/
choco push .choco/{APP_NAME_KEBAB}.{VERSION}.nupkg --source https://push.chocolatey.org/
```

3. **Surveiller la file de modération :**

- Aller sur <https://community.chocolatey.org/packages/{APP_NAME_KEBAB}>
- Le paquet sera examiné par les modérateurs (généralement sous 48 heures)

### Homebrew

1. **Créer un dépôt tap** (première fois seulement) :

```bash
# Créer un nouveau dépôt sur GitHub : homebrew-tap
git clone https://github.com/YOUR-USERNAME/homebrew-tap.git
cd homebrew-tap
```

2. **Copier la formule :**

```bash
cp .homebrew/{APP_NAME_KEBAB}.rb Formula/{APP_NAME_KEBAB}.rb
git add Formula/{APP_NAME_KEBAB}.rb
git commit -m "Add {APP_NAME_KEBAB} {VERSION}"
git push
```

3. **Les utilisateurs peuvent maintenant installer avec :**

```bash
brew tap YOUR-USERNAME/tap
brew install {APP_NAME_KEBAB}
```

### Publication automatisée via GitHub Actions

Le dépôt inclut un workflow GitHub Actions (`.github/workflows/release.yml`) qui automatise l'ensemble du processus.

#### Configurer les secrets

Ajouter ces secrets aux paramètres de votre dépôt GitHub :

1. **CHOCOLATEY_API_KEY** : Votre clé API Chocolatey
2. **HOMEBREW_TAP_TOKEN** : GitHub Personal Access Token pour votre dépôt tap

#### Déclencher une Release

1. **Mettre à jour la version dans `tauri.conf.json` et `package.json`**

2. **Créer et pousser un tag git :**

```bash
git tag v{VERSION}
git push origin v{VERSION}
```

3. **Le workflow va automatiquement :**
   - Construire pour Windows, macOS (ARM64 & x64) et Linux
   - Créer une GitHub Release
   - Générer le paquet Chocolatey et le pousser vers le dépôt communautaire
   - Générer la formule Homebrew et créer une PR vers le dépôt tap

## Gestion des versions

Toujours garder les versions synchronisées entre :

- `package.json` → `version`
- `backend/tauri.conf.json` → `version`
- `backend/Cargo.toml` → `version`

## Dépannage

### Chocolatey

**Problème** : Le paquet échoue la modération

- Vérifier les commentaires de modération sur <https://community.chocolatey.org/>
- Problèmes courants : checksums incorrects, dépendances manquantes, problèmes d'installation
- Corriger et resoumettre avec une version incrémentée

**Problème** : Incompatibilité de checksum

- S'assurer que le fichier MSI n'a pas changé depuis le calcul du checksum
- Reconstruire le paquet avec `pnpm package:chocolatey`

### Homebrew

**Problème** : Incompatibilité SHA256 lors de l'installation

- Vérifier que les fichiers DMG sont téléchargés vers GitHub releases
- S'assurer que les URLs dans la formule correspondent aux assets de release réels
- Recalculer les checksums : `shasum -a 256 *.dmg`

**Problème** : L'application ne s'ouvre pas sur macOS

- S'assurer que l'application est correctement signée et notariée
- Vérifier si Gatekeeper bloque : `xattr -d com.apple.quarantine /Applications/Tauri\ Vue3\ App.app`

## Ressources

- [Chocolatey Package Creation](https://docs.chocolatey.org/en-us/create/create-packages)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Tauri Bundling](https://tauri.app/v1/guides/building/)

## Prerequisites

### Chocolatey (Windows)

- Install Chocolatey: <https://chocolatey.org/install>
- Create an account at <https://community.chocolatey.org/>
- Get your API key from your account page

### Homebrew (macOS)

- Create a GitHub repository for your Homebrew tap (e.g., `homebrew-tap`)
- Generate a GitHub Personal Access Token with `repo` scope

## Building Packages

### 1. Build the Tauri Application

```bash
pnpm build:tauri
```

This will create platform-specific installers in:

- Windows: `backend/target/release/bundle/msi/`
- macOS: `backend/target/release/bundle/dmg/`
- Linux: `backend/target/release/bundle/deb/` or `appimage/`

### 2. Generate Chocolatey Package (Windows)

```powershell
pnpm package:chocolatey
```

Or manually:

```powershell
.\scripts\build-chocolatey.ps1 -Version {VERSION}
```

This will:

- Calculate SHA256 checksum of the MSI file
- Update `chocolateyinstall.ps1` with the correct checksum
- Create `.choco/{APP_NAME_KEBAB}.{VERSION}.nupkg`

### 3. Generate Homebrew Formula (macOS)

```bash
pnpm package:homebrew
```

Or manually:

```bash
./scripts/build-homebrew.sh {VERSION}
```

This will:

- Calculate SHA256 checksums for both ARM64 and x64 DMG files
- Update `.homebrew/{APP_NAME_KEBAB}.rb` with checksums

## Publishing

### Chocolatey

1. **Test locally first:**

```powershell
choco install {APP_NAME_KEBAB} -source .choco
```

2. **Push to Chocolatey Community Repository:**

```powershell
choco apikey --key YOUR-API-KEY --source https://push.chocolatey.org/
choco push .choco/{APP_NAME_KEBAB}.{VERSION}.nupkg --source https://push.chocolatey.org/
```

3. **Monitor the moderation queue:**

- Go to <https://community.chocolatey.org/packages/{APP_NAME_KEBAB}>
- The package will be reviewed by moderators (usually within 48 hours)

### Homebrew

1. **Create a tap repository** (first time only):

```bash
# Create a new repository on GitHub: homebrew-tap
git clone https://github.com/YOUR-USERNAME/homebrew-tap.git
cd homebrew-tap
```

2. **Copy the formula:**

```bash
cp .homebrew/{APP_NAME_KEBAB}.rb Formula/{APP_NAME_KEBAB}.rb
git add Formula/{APP_NAME_KEBAB}.rb
git commit -m "Add {APP_NAME_KEBAB} {VERSION}"
git push
```

3. **Users can now install with:**

```bash
brew tap YOUR-USERNAME/tap
brew install {APP_NAME_KEBAB}
```

### Automated Publishing via GitHub Actions

The repository includes a GitHub Actions workflow (`.github/workflows/release.yml`) that automates the entire process.

#### Setup Secrets

Add these secrets to your GitHub repository settings:

1. **CHOCOLATEY_API_KEY**: Your Chocolatey API key
2. **HOMEBREW_TAP_TOKEN**: GitHub Personal Access Token for your tap repository

#### Trigger a Release

1. **Update version in `tauri.conf.json` and `package.json`**

2. **Create and push a git tag:**

```bash
git tag v{VERSION}
git push origin v{VERSION}
```

3. **The workflow will automatically:**
   - Build for Windows, macOS (ARM64 & x64), and Linux
   - Create a GitHub Release
   - Generate Chocolatey package and push to community repository
   - Generate Homebrew formula and create PR to tap repository

## Version Management

Always keep versions synchronized across:

- `package.json` → `version`
- `backend/tauri.conf.json` → `version`
- `backend/Cargo.toml` → `version`

## Troubleshooting

### Chocolatey

**Issue**: Package fails moderation

- Check the moderation comments at <https://community.chocolatey.org/>
- Common issues: incorrect checksums, missing dependencies, installation issues
- Fix and resubmit with an incremented version

**Issue**: Checksum mismatch

- Ensure the MSI file hasn't changed since checksum calculation
- Rebuild the package with `pnpm package:chocolatey`

### Homebrew

**Issue**: SHA256 mismatch during installation

- Verify the DMG files are uploaded to GitHub releases
- Ensure URLs in the formula match the actual release assets
- Recalculate checksums: `shasum -a 256 Drop.Compress.Image_*.dmg`

**Issue**: App won't open on macOS

- Ensure the app is properly signed and notarized
- Check if Gatekeeper is blocking: `xattr -d com.apple.quarantine /Applications/Drop\ Compress\ Image.app`

## Resources

- [Chocolatey Package Creation](https://docs.chocolatey.org/en-us/create/create-packages)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Tauri Bundling](https://tauri.app/v1/guides/building/)
