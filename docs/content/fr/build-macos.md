# Construire Drop Compress Image pour macOS

Ce guide vous accompagne dans la configuration de l'environnement de développement et la construction de Drop Compress Image sur les systèmes macOS.

## Prérequis

Avant de commencer, assurez-vous d'avoir :

- macOS 10.15 (Catalina) ou plus récent
- Privilèges administrateur pour installer des logiciels
- Familiarité de base avec les commandes Terminal

## Étape 1 : Installer les Outils en Ligne de Commande Xcode

Tout d'abord, installez les Outils en Ligne de Commande Xcode qui fournissent des outils de développement essentiels incluant `clang` et `make` :

```bash
xcode-select --install
```

Cela ouvrira une boîte de dialogue demandant si vous voulez installer les outils de développement en ligne de commande. Cliquez sur **Installer** et attendez que l'installation soit terminée.

### Vérifier l'Installation

Vérifiez que les outils sont correctement installés :

```bash
clang --version
```

Vous devriez voir une sortie similaire à :

```text
Apple clang version 15.0.0 (clang-1500.0.40.1)
Target: arm64-apple-darwin23.0.0
Thread model: posix
```

## Étape 2 : Installer Homebrew

Homebrew est un gestionnaire de paquets pour macOS qui facilite l'installation d'outils de développement et de bibliothèques.

### Installer Homebrew

Ouvrez Terminal et exécutez :

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### Ajouter Homebrew au PATH

Pour les Mac Apple Silicon (M1/M2/M3), ajoutez Homebrew à votre PATH :

```bash
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zshrc
source ~/.zshrc
```

Pour les Mac Intel, Homebrew est installé dans `/usr/local` et devrait déjà être dans votre PATH.

### Vérifier l'Installation de Homebrew

```bash
brew --version
```

## Étape 3 : Installer Rust

Drop Compress Image est construit avec Rust, vous devrez donc installer la chaîne d'outils Rust.

### Installer Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Quand on vous le demande, choisissez l'option 1 (installation par défaut).

### Configurer Votre Shell

```bash
source ~/.cargo/env
```

### Vérifier l'Installation de Rust

```bash
rustc --version
cargo --version
```

Vous devriez voir les informations de version pour `rustc` et `cargo`.

## Étape 4 : Installer Node.js

Le frontend de Drop Compress Image est construit avec Vue.js et nécessite Node.js.

### Installer Node.js via Homebrew

```bash
brew install node
```

### Vérifier l'Installation de Node.js

```bash
node --version
npm --version
```

## Étape 5 : Installer pnpm

Drop Compress Image utilise pnpm comme gestionnaire de paquets pour de meilleures performances et efficacité disque.

### Installer pnpm

```bash
brew install pnpm
```

### Vérifier l'Installation de pnpm

```bash
pnpm --version
```

## Étape 6 : Configurer vcpkg et Installer les Dépendances

Ce projet utilise vcpkg pour gérer les bibliothèques de traitement d'images C/C++ (libaom, libavif, libjxl, etc.).

### Installer vcpkg

```bash
# Cloner vcpkg
git clone https://github.com/Microsoft/vcpkg.git ~/Developer/vcpkg

# Bootstrap vcpkg
cd ~/Developer/vcpkg
./bootstrap-vcpkg.sh

# Définir les variables d'environnement (ajouter à ~/.zshrc)
echo 'export VCPKG_ROOT="$HOME/Developer/vcpkg"' >> ~/.zshrc
echo 'export PATH="$VCPKG_ROOT:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Installer les Dépendances

Utilisez le script d'installation automatique (recommandé) :

```bash
cd ~/path/to/DropWebP/app/src-tauri
./setup-vcpkg.sh
```

Ou installez manuellement :

```bash
cd ~/Developer/vcpkg

# Pour Apple Silicon (M1/M2/M3)
./vcpkg install aom:arm64-osx
./vcpkg install libavif[aom]:arm64-osx
./vcpkg install libjxl:arm64-osx
./vcpkg install libwebp:arm64-osx
./vcpkg install openjpeg:arm64-osx
./vcpkg install libjpeg-turbo:arm64-osx
./vcpkg install lcms:arm64-osx

# Pour Mac Intel
./vcpkg install aom:x64-osx
./vcpkg install libavif[aom]:x64-osx
./vcpkg install libjxl:x64-osx
./vcpkg install libwebp:x64-osx
./vcpkg install openjpeg:x64-osx
./vcpkg install libjpeg-turbo:x64-osx
./vcpkg install lcms:x64-osx
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

## Étape 8 : Considérations Spécifiques à la Plateforme

### Mac Apple Silicon (M1/M2/M3)

Si vous utilisez un Mac Apple Silicon, certaines dépendances pourraient nécessiter d'être compilées spécifiquement pour l'architecture `arm64`. La plupart des paquets modernes gèrent cela automatiquement, mais si vous rencontrez des problèmes :

```bash
# Vérifier votre architecture
uname -m
# Devrait afficher : arm64

# Si nécessaire, vous pouvez forcer Rust à construire pour la bonne cible
rustup target add aarch64-apple-darwin
```

### Mac Intel

Pour les Mac Intel, la cible par défaut `x86_64` devrait fonctionner sans problèmes :

```bash
# Vérifier votre architecture
uname -m
# Devrait afficher : x86_64

# S'assurer que la bonne cible Rust est installée
rustup target add x86_64-apple-darwin
```

### Signature de Code (Optionnel)

Si vous voulez distribuer votre application construite, vous devrez la signer avec un certificat Apple Developer :

```bash
# Vérifier les identités de signature disponibles
security find-identity -v -p codesigning

# Si vous avez un certificat développeur, Tauri peut signer automatiquement
# Ajoutez ceci à votre tauri.conf.json :
{
  "bundle": {
    "macOS": {
      "signing": {
        "identity": "Developer ID Application: Your Name (TEAM_ID)"
      }
    }
  }
}
```

## Dépannage

### Problèmes Courants

1. **Erreurs de Permission Refusée**

   ```bash
   # Corriger les permissions pour Homebrew
   sudo chown -R $(whoami) /opt/homebrew
   ```

2. **Commande Non Trouvée Après Installation**

   ```bash
   # Recharger votre profil shell
   source ~/.zshrc
   # Ou redémarrer votre terminal
   ```

3. **Échecs de Construction avec les Dépendances Natives**

   ```bash
   # Nettoyer les caches de construction
   cargo clean
   pnpm clean

   # Tout reconstruire
   pnpm install
   pnpm tauri build
   ```

4. **Problèmes de Cible Rust**

   ```bash
   # Lister les cibles installées
   rustup target list --installed

   # Ajouter la bonne cible pour votre système
   rustup target add aarch64-apple-darwin  # Apple Silicon
   rustup target add x86_64-apple-darwin   # Intel
   ```

### Obtenir de l'Aide

Si vous rencontrez des problèmes non couverts ici :

1. Vérifiez le [référentiel Drop Compress Image](https://github.com/logue/DropWebP) pour les problèmes connus
2. Consultez la [documentation Tauri v2](https://v2.tauri.app/start/prerequisites/) pour des conseils spécifiques à macOS
3. Recherchez les issues GitHub existantes ou créez-en une nouvelle

## Prochaines Étapes

Une fois que Drop Compress Image est construit avec succès :

1. **Exécuter les Tests** : Exécutez `pnpm test` pour vous assurer que tout fonctionne correctement
2. **Développement** : Utilisez `pnpm tauri dev` pour le développement avec rechargement à chaud
3. **Personnalisation** : Explorez la base de code et apportez vos modifications
4. **Distribution** : Utilisez `pnpm tauri build` pour créer des paquets distribuables

Vous êtes maintenant prêt à développer et construire Drop Compress Image sur macOS !

## Compilation pour Intel Mac

Si vous souhaitez compiler pour Intel Mac (x86_64) à partir d'un Mac Apple Silicon (M1/M2/M3), suivez ces étapes.

### Méthode 1 : Binaire universel (Recommandé)

Créez un binaire unique qui fonctionne sur les Mac Intel et Apple Silicon :

```bash
cd app
pnpm run build:tauri:mac-universal
```

**Avantages :**

- Aucune installation de bibliothèque supplémentaire nécessaire
- Un seul binaire prend en charge les deux architectures
- Les utilisateurs n'ont pas à se soucier de l'architecture de leur Mac

**Inconvénients :**

- La taille du fichier est environ doublée (contient le code pour les deux architectures)

### Méthode 2 : Compilation Intel uniquement

Si vous souhaitez créer un binaire spécifique Intel Mac, vous aurez besoin des bibliothèques x86_64.

#### Étape 1 : Installer Homebrew x86_64

```bash
arch -x86_64 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

#### Étape 2 : Installer les bibliothèques x86_64

```bash
arch -x86_64 /usr/local/bin/brew install libavif jpeg-xl
```

Ou utilisez le script de configuration :

```bash
./scripts/setup-x86-libs.sh
```

#### Étape 3 : Compiler

```bash
cd app
pnpm run build:tauri:mac-x64
```

### Aperçu des cibles de compilation

```bash
# Apple Silicon uniquement
pnpm run build:tauri:mac-arm64

# Intel Mac uniquement
pnpm run build:tauri:mac-x64

# Binaire universel (les deux)
pnpm run build:tauri:mac-universal
```

### Emplacement des artefacts de compilation

```text
app/src-tauri/target/
  ├── aarch64-apple-darwin/release/bundle/      # ARM64 uniquement
  ├── x86_64-apple-darwin/release/bundle/       # x86_64 uniquement
  └── universal-apple-darwin/release/bundle/    # Universel (les deux)
```
