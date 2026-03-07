# Construction pour Linux (avec Docker)

Comment construire des binaires Linux depuis Windows, macOS ou Linux en utilisant Docker

## 📋 Prérequis

### Commun à toutes les plateformes

- Docker Desktop ou Docker Engine
- pnpm (v10.2.0 ou supérieur)
- 8 Go+ de RAM (16 Go recommandés)
- 20 Go+ d'espace disque libre

### Spécifique à la plateforme

#### Windows

- Windows 10/11 (64 bits)
- WSL 2 (recommandé)
- PowerShell 5.1 ou supérieur

#### macOS

- macOS 10.15 ou supérieur
- Bash
- Docker Desktop pour Mac

#### Linux

- Distribution Linux 64 bits
- Docker Engine 20.10 ou supérieur
- Bash

## 🚀 Utilisation

### Construction sur Windows

```powershell
# Exécuter depuis la racine du projet
pnpm run build:tauri:linux-x64    # Linux x86_64
pnpm run build:tauri:linux-arm64  # Linux ARM64

# Ou exécuter le script directement
pwsh .\scripts\build-linux-docker.ps1 -Target x64
pwsh .\scripts\build-linux-docker.ps1 -Target arm64
```

### Construction sur macOS / Linux

```bash
# Exécuter depuis la racine du projet
bash scripts/build-linux-docker.sh x64    # Linux x86_64
bash scripts/build-linux-docker.sh arm64  # Linux ARM64

# Ou depuis le répertoire app
pnpm run build:tauri:linux-docker-x64
pnpm run build:tauri:linux-docker-arm64
```

## 📦 Artefacts de construction

Les artefacts de construction sont générés dans les répertoires suivants :

```text
backend/target/
  ├── x86_64-unknown-linux-gnu/release/bundle/
  │   ├── deb/           # Paquets Debian/Ubuntu
  │   ├── rpm/           # Paquets Red Hat/Fedora
  │   └── appimage/      # AppImage (recommandé pour la distribution)
  │
  └── aarch64-unknown-linux-gnu/release/bundle/
      ├── deb/
      ├── rpm/
      └── appimage/
```

## ⚙️ Fonctionnement

1. Construction de l'image Docker depuis `Dockerfile.linux-build`
   - Basé sur Rust 1.83 + Debian Bookworm
   - Installe les dépendances Tauri (WebKit2GTK, GTK3, etc.)
   - Installe Node.js 22.x et pnpm

2. Exécution de la construction Tauri dans le conteneur Docker
   - Monte le répertoire du projet
   - Construit avec l'architecture cible spécifiée

3. Sortie des artefacts vers le répertoire macOS

## 🔧 Dépannage

### Reconstruire l'image Docker

```bash
docker build -f Dockerfile.linux-build -t tauri-vue3-linux-builder --no-cache .
```

### Supprimer l'image Docker

```bash
docker rmi tauri-vue3-linux-builder
```

### Effacer le cache de construction

```bash
rm -rf backend/target/x86_64-unknown-linux-gnu
rm -rf backend/target/aarch64-unknown-linux-gnu
```

## 📝 Remarques

- La construction initiale prend plus de temps en raison de la construction de l'image Docker et des téléchargements (20-30 minutes)
- Les constructions suivantes sont plus rapides car l'image Docker est réutilisée (10-15 minutes)
- Les constructions ARM64 peuvent prendre plus de temps que les constructions x86_64

## 🎯 Format de distribution recommandé

- **AppImage** : Recommandé pour la distribution (fonctionne sur toutes les distributions Linux)
- **.deb** : Pour les utilisateurs Debian/Ubuntu
- **.rpm** : Pour les utilisateurs Red Hat/Fedora
