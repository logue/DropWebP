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
npm install -g pnpm
```

### Vérifier l'Installation de pnpm

```bash
pnpm --version
```

## Étape 6 : Installer les Dépendances Supplémentaires

Installez des outils supplémentaires requis pour la construction :

```bash
# Installer CMake (nécessaire pour certaines dépendances natives)
brew install cmake

# Installer pkg-config (nécessaire pour lier les bibliothèques)
brew install pkg-config
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
