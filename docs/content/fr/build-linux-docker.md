# Construction pour Linux (avec Docker)

Comment construire des binaires Linux depuis macOS en utilisant Docker

## 📋 Prérequis

- Docker Desktop pour Mac doit être installé
- Espace disque suffisant (environ 5 Go nécessaires pour la construction initiale)

## 🚀 Utilisation

### Construction pour x86_64 (AMD64)

```bash
# Exécuter depuis la racine du projet
./scripts/build-linux-docker.sh x64

# Ou depuis le répertoire app
pnpm run build:tauri:linux-docker-x64
```

### Construction pour ARM64 (AArch64)

```bash
# Exécuter depuis la racine du projet
./scripts/build-linux-docker.sh arm64

# Ou depuis le répertoire app
pnpm run build:tauri:linux-docker-arm64
```

## 📦 Artefacts de construction

Les artefacts de construction sont générés dans les répertoires suivants :

```text
app/src-tauri/target/
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
docker build -f Dockerfile.linux-build -t dropwebp-linux-builder --no-cache .
```

### Supprimer l'image Docker

```bash
docker rmi dropwebp-linux-builder
```

### Effacer le cache de construction

```bash
rm -rf app/src-tauri/target/x86_64-unknown-linux-gnu
rm -rf app/src-tauri/target/aarch64-unknown-linux-gnu
```

## 📝 Remarques

- La construction initiale prend plus de temps en raison de la construction de l'image Docker et des téléchargements (20-30 minutes)
- Les constructions suivantes sont plus rapides car l'image Docker est réutilisée (10-15 minutes)
- Les constructions ARM64 peuvent prendre plus de temps que les constructions x86_64

## 🎯 Format de distribution recommandé

- **AppImage** : Recommandé pour la distribution (fonctionne sur toutes les distributions Linux)
- **.deb** : Pour les utilisateurs Debian/Ubuntu
- **.rpm** : Pour les utilisateurs Red Hat/Fedora
