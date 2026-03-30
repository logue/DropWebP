# Construire Drop Compress Image pour Linux

Ce guide vous accompagne dans la configuration de l'environnement de développement et la construction de Drop Compress Image sur Ubuntu 24.04 LTS (et distributions similaires basées sur Debian).

## Prérequis

Avant de commencer, assurez-vous d'avoir :

- Ubuntu 24.04 LTS ou distribution similaire basée sur Debian
- Privilèges sudo pour installer des logiciels
- Familiarité de base avec les commandes terminal

## Étape 1 : Mettre à Jour les Paquets Système

Tout d'abord, mettez à jour vos paquets système pour vous assurer d'avoir les dernières versions :

```bash
sudo apt update
sudo apt upgrade -y
```

## Étape 2 : Installer les Dépendances de Construction

Installez les outils de construction essentiels et les bibliothèques requis pour le développement Tauri :

```bash
# Installer les essentiels de construction et les bibliothèques de développement
sudo apt install -y \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libwebkit2gtk-4.1-dev \
  patchelf
```

### Ce que Font ces Paquets

- **build-essential**: Fournit GCC, G++ et make
- **libssl-dev**: Bibliothèques de développement OpenSSL
- **libgtk-3-dev**: Bibliothèques de développement GTK3 pour l'interface
- **libayatana-appindicator3-dev**: Support de la barre d'état système
- **librsvg2-dev**: Support du rendu SVG
- **libwebkit2gtk-4.1-dev**: WebKit pour la webview de Tauri
- **patchelf**: Patcheur binaire ELF pour AppImage

### Vérifier l'Installation

```bash
gcc --version
```

Vous devriez voir une sortie montrant GCC version 13.x ou supérieure.

## Étape 3 : Installer Rust

Drop Compress Image est construit avec Rust, vous devrez donc installer la chaîne d'outils Rust.

### Installer Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Quand on vous le demande, choisissez l'option 1 (installation par défaut).

### Configurer Votre Shell

```bash
source $HOME/.cargo/env
```

Pour rendre cela permanent, ajoutez-le à votre profil shell :

```bash
echo 'source $HOME/.cargo/env' >> ~/.bashrc
source ~/.bashrc
```

### Vérifier l'Installation de Rust

```bash
rustc --version
cargo --version
```

Vous devriez voir les informations de version pour `rustc` et `cargo`.

## Étape 4 : Installer Node.js

Le frontend de Drop Compress Image est construit avec Vue.js et nécessite Node.js.

### Installer Node.js via le Dépôt NodeSource

```bash
# Installer Node.js 22.x (LTS)
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt install -y nodejs
```

### Vérifier l'Installation de Node.js

```bash
node --version
npm --version
```

Vous devriez voir Node.js version 22.x ou supérieure.

## Étape 5 : Installer pnpm

Drop Compress Image utilise pnpm comme gestionnaire de paquets pour de meilleures performances et efficacité disque.

### Installer pnpm

```bash
npm install -g pnpm
```

### Vérifier l'Installation de pnpm

```bash
pnpm --version
```

## Étape 6 : Configurer vcpkg et Installer les Dépendances

Ce projet utilise vcpkg pour gérer les bibliothèques de traitement d'images C/C++ (libaom, libavif, libjxl, etc.).

### Installer les Prérequis de vcpkg

```bash
# Installer les outils requis pour vcpkg
sudo apt install -y curl zip unzip tar cmake pkg-config
```

### Installer vcpkg

```bash
# Cloner vcpkg
git clone https://github.com/Microsoft/vcpkg.git ~/vcpkg

# Bootstrap vcpkg
cd ~/vcpkg
./bootstrap-vcpkg.sh

# Définir les variables d'environnement (ajouter à ~/.bashrc)
echo 'export VCPKG_ROOT="$HOME/vcpkg"' >> ~/.bashrc
echo 'export PATH="$VCPKG_ROOT:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Installer les Dépendances

Utilisez le script d'installation automatique (recommandé) :

```bash
cd ~/path/to/DropWebP/app/src-tauri
./setup-vcpkg.sh
```

Ou installez manuellement :

```bash
cd ~/vcpkg

# Pour Linux x64
./vcpkg install aom:x64-linux
./vcpkg install libavif[aom]:x64-linux
./vcpkg install libjxl:x64-linux
./vcpkg install libwebp:x64-linux
./vcpkg install openjpeg:x64-linux
./vcpkg install libjpeg-turbo:x64-linux
./vcpkg install lcms:x64-linux

# Pour Linux ARM64
./vcpkg install aom:arm64-linux
./vcpkg install libavif[aom]:arm64-linux
./vcpkg install libjxl:arm64-linux
./vcpkg install libwebp:arm64-linux
./vcpkg install openjpeg:arm64-linux
./vcpkg install libjpeg-turbo:arm64-linux
./vcpkg install lcms:arm64-linux
```

Bibliothèques installées :

- **libaom** : Encodeur AV1 (pour le format AVIF, **requis**)
- **libavif** : Format d'image AVIF
- **libjxl** : Format d'image JPEG XL
- **libwebp** : Format d'image WebP
- **openjpeg** : Format d'image JPEG 2000
- **libjpeg-turbo** : Traitement d'images JPEG (pour jpegli)
- **lcms** : Gestion des couleurs Little CMS

### Vérifier l'Installation

```bash
./vcpkg list | grep -E "aom|avif|jxl|webp|openjpeg|jpeg|lcms"
```

## Étape 7 : Cloner et Construire Drop Compress Image

Maintenant vous êtes prêt à cloner et construire Drop Compress Image.

### Cloner le Référentiel

```bash
git clone https://github.com/logue/DropWebP.git
cd DropWebP
```

### Installer les Dépendances Frontend

```bash
# Installer toutes les dépendances de l'espace de travail
pnpm install
```

### Installer Tauri CLI v2

```bash
# Installer Tauri CLI v2 globalement
pnpm add -g @tauri-apps/cli@next
```

### Construire l'Application

Pour le développement :

```bash
# Exécuter en mode développement
pnpm dev:tauri
```

Pour la production :

```bash
# Construire pour la production
pnpm build:tauri
```

L'application construite sera dans `app/src-tauri/target/release/`.

## Étape 8 : Formats de Distribution

Tauri sur Linux peut générer plusieurs formats de distribution :

### AppImage (Recommandé)

AppImage est un format de paquet universel qui fonctionne sur la plupart des distributions Linux :

```bash
pnpm build:tauri
```

L'AppImage sera dans `app/src-tauri/target/release/bundle/appimage/`.

### Paquet Debian (.deb)

Pour les distributions basées sur Debian/Ubuntu :

```bash
pnpm build:tauri
```

Le paquet .deb sera dans `app/src-tauri/target/release/bundle/deb/`.

Installez-le avec :

```bash
sudo dpkg -i app/src-tauri/target/release/bundle/deb/*.deb
```

### Paquet RPM (.rpm)

Pour les distributions basées sur Red Hat/Fedora, vous devrez installer des outils supplémentaires :

```bash
sudo apt install -y rpm
pnpm build:tauri
```

Le paquet .rpm sera dans `app/src-tauri/target/release/bundle/rpm/`.

## Dépannage

### Problèmes Courants

1. **libwebkit2gtk-4.1 Manquant**

   Si vous obtenez des erreurs concernant les bibliothèques webkit manquantes :

   ```bash
   # Essayez l'ancienne version webkit
   sudo apt install -y libwebkit2gtk-4.0-dev
   ```

2. **Permission Refusée pour npm/pnpm**

   ```bash
   # Corriger les permissions du répertoire global npm
   mkdir -p ~/.npm-global
   npm config set prefix '~/.npm-global'
   echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **Échecs de Construction avec les Dépendances Natives**

   ```bash
   # Nettoyer les caches de construction
   cargo clean
   pnpm clean

   # Tout reconstruire
   pnpm install
   pnpm build:tauri
   ```

4. **AppImage Non Exécutable**

   ```bash
   # Rendre l'AppImage exécutable
   chmod +x app/src-tauri/target/release/bundle/appimage/*.AppImage
   ```

5. **Version GLIBC Manquante**

   Si vous voyez des erreurs concernant la version GLIBC, assurez-vous d'être sur Ubuntu 24.04 LTS ou plus récent :

   ```bash
   ldd --version
   ```

### Problèmes de Pilotes Graphiques

Pour des performances optimales, assurez-vous d'avoir les pilotes graphiques appropriés installés :

```bash
# Pour NVIDIA
sudo ubuntu-drivers autoinstall

# Pour AMD
sudo apt install -y mesa-vulkan-drivers

# Pour Intel
sudo apt install -y intel-media-va-driver
```

### Obtenir de l'Aide

Si vous rencontrez des problèmes non couverts ici :

1. Vérifiez le [référentiel Drop Compress Image](https://github.com/logue/DropWebP) pour les problèmes connus
2. Consultez la [documentation Tauri v2](https://v2.tauri.app/start/prerequisites/) pour des conseils spécifiques à Linux
3. Recherchez les issues GitHub existantes ou créez-en une nouvelle

## Prochaines Étapes

Une fois que Drop Compress Image est construit avec succès :

1. **Exécuter les Tests** : Exécutez `pnpm test` pour vous assurer que tout fonctionne correctement
2. **Développement** : Utilisez `pnpm dev:tauri` pour le développement avec rechargement à chaud
3. **Personnalisation** : Explorez la base de code et apportez vos modifications
4. **Distribution** : Utilisez `pnpm build:tauri` pour créer des paquets distribuables

Vous êtes maintenant prêt à développer et construire Drop Compress Image sur Linux !
