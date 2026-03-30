# Configuration de l'environnement de développement (Windows)

Guide pour la configuration de l'environnement de développement de Drop Compress Image sur Windows.

## Choisissez votre méthode de construction

Il existe deux façons de construire sur Windows :

1. **Environnement Docker (Recommandé)** : Environnement propre évitant les conflits de dépendances
2. **Environnement natif** : Plus rapide mais configuration plus complexe

---

## Méthode 1 : Construction avec Docker (Recommandé)

### Prérequis

- Windows 10/11 Pro, Enterprise ou Education (avec support Hyper-V)
- Docker Desktop pour Windows

### Étapes

1. **Installer Docker Desktop**

   Téléchargez et installez [Docker Desktop](https://www.docker.com/products/docker-desktop).

2. **Basculer en mode conteneur Windows**

   Faites un clic droit sur l'icône Docker Desktop dans la barre des tâches et sélectionnez « Switch to Windows containers... ».

3. **Cloner le projet**

   ```powershell
   git clone https://github.com/logue/DropWebP.git
   cd DropWebP
   ```

4. **Construire l'image Docker** (première fois seulement, prend 30-60 minutes)

   ```powershell
   docker build -f Dockerfile.windows-x64 -t dropwebp-windows-builder .
   ```

5. **Construire l'application**

   ```powershell
   docker run --rm -v ${PWD}:C:\workspace dropwebp-windows-builder
   ```

6. **Vérifier les artefacts de construction**

   Une fois la construction réussie, les exécutables et installateurs seront générés dans le répertoire `app/src-tauri/target/release/bundle/`.

### Avantages de l'environnement Docker

- ✅ Garde l'environnement hôte propre
- ✅ Évite les conflits de dépendances
- ✅ Constructions reproductibles
- ✅ Environnement de construction propre
- ✅ Cohérence avec les pipelines CI/CD

---

## Méthode 2 : Construction en environnement natif

## 1. Installer Chocolatey

Ouvrez PowerShell en tant qu'administrateur et exécutez la commande suivante pour installer Chocolatey.

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force;
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```

Après l'installation, vous pouvez vérifier la version avec la commande ci-dessous.

```powershell
choco -v
```

## 2. Installer Git

Installez Git en utilisant Chocolatey.

```powershell
choco install git -y
```

Après l'installation, vérifiez la version.

```powershell
git --version
```

## 3. Cloner le projet

Clonez le projet depuis GitHub et naviguez vers le répertoire du projet.

```powershell
git clone https://github.com/logue/DropWebP.git
cd DropWebP
```

## 4. Installer Visual Studio Community 2022

Installez Visual Studio Community 2022.

```powershell
choco install visualstudio2022community -y
```

Ensuite, installez la charge de travail de développement de bureau C++.

```powershell
choco install visualstudio2022-workload-nativedesktop -y
```

Installez les outils de construction Clang/LLVM, qui sont nécessaires pour construire certaines bibliothèques de codecs d'images.

```powershell
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Component.VC.Llvm.Clang --add Microsoft.VisualStudio.Component.VC.Llvm.ClangToolset" -y
```

Une fois l'installation terminée, vous pouvez vérifier les composants installés à l'aide de l'installateur Visual Studio.

> **Remarque :** La charge de travail de développement de bureau C++ comprend les outils nécessaires pour construire des extensions natives Rust, tels que MSVC (le compilateur de Microsoft), le SDK Windows et CMake.

## 5. Installer NASM et Ninja

Installez NASM et Ninja, qui sont nécessaires pour construire des bibliothèques de codecs d'images.

```powershell
choco install nasm ninja -y
```

Après l'installation, vérifiez les versions.

```powershell
nasm -v
ninja --version
```

Ajoutez NASM à votre PATH système afin que Cargo puisse le trouver lors de la compilation.

```powershell
[System.Environment]::SetEnvironmentVariable('PATH', [System.Environment]::GetEnvironmentVariable('PATH', 'User') + ';C:\Program Files\NASM', 'User')
```

Redémarrez votre terminal ou session PowerShell pour que les modifications du PATH prennent effet.

> **Remarque :** NASM (Netwide Assembler) est un assembleur utilisé pour construire des bibliothèques de codecs optimisées comme libavif. Ninja est un système de construction rapide souvent utilisé en conjonction avec CMake.

## 6. Installer Node.js et pnpm

Installez Node.js et pnpm.

```powershell
choco install nodejs pnpm -y
```

Après l'installation, vérifiez les versions.

```powershell
node -v
pnpm -v
```

## 7. Installer Rust (Méthode officielle)

Installez Rust en utilisant la méthode officielle en exécutant la commande suivante dans PowerShell ou l'invite de commandes.

```powershell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Après l'installation, vérifiez la version.

```powershell
rustc --version
```

> **Avertissement :** Bien qu'il soit possible d'installer Rust via Chocolatey, il s'installe avec la chaîne d'outils MinGW, ce qui peut entraîner des problèmes de compatibilité avec les bibliothèques.

## 8. Configurer vcpkg

1. Clonez le référentiel vcpkg :

   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
   cd C:\vcpkg
   ```

2. Exécutez le script de bootstrap :

   ```powershell
   .\bootstrap-vcpkg.bat
   ```

3. Définissez les variables d'environnement (recommandé d'ajouter aux variables d'environnement système) :

   ```powershell
   $env:VCPKG_ROOT = "C:\vcpkg"
   [System.Environment]::SetEnvironmentVariable('VCPKG_ROOT', 'C:\vcpkg', 'User')
   ```

> **Important :** La variable d'environnement VCPKG_ROOT est requise pour que le système de construction localise les bibliothèques vcpkg.

## 9. Installer les dépendances

### Créer un triplet de version

Le triplet par défaut de vcpkg inclut des symboles de débogage qui causent des erreurs de liaison avec les builds de release Rust. Créez un triplet personnalisé :

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE x64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\x64-windows-static-release.cmake
```

### Installer les dépendances

> **Note (Mise à jour février 2026)** : Le projet utilise maintenant `rav1e` (un encodeur AV1 basé sur Rust) pour l'encodage AVIF sous Windows. Cela élimine le besoin des paquets `libaom` et `aom`. `rav1e` évite les exigences d'optimisation multipass de NASM et améliore la stabilité de la compilation sous Windows.

Utilisez le script d'installation automatique (recommandé) :

```powershell
cd DropWebP\app\src-tauri
.\setup-vcpkg.ps1
```

Ou installez manuellement :

```powershell
cd C:\vcpkg

# Installer avec le triplet x64-windows-static-release (release uniquement)
# Note : aom et libavif[aom] ne sont plus nécessaires (utilisation de rav1e)
.\vcpkg install libjxl:x64-windows-static-release
.\vcpkg install libwebp:x64-windows-static-release
.\vcpkg install openjpeg:x64-windows-static-release
.\vcpkg install libjpeg-turbo:x64-windows-static-release
.\vcpkg install lcms:x64-windows-static-release
```

Bibliothèques installées :

- **rav1e** : Encodeur AV1 (basé sur Rust, pour l'encodage AVIF) - compilé automatiquement par Cargo
- **libjxl** : Format d'image JPEG XL
- **libwebp** : Format d'image WebP
- **openjpeg** : Format d'image JPEG 2000
- **libjpeg-turbo** : Traitement d'images JPEG (pour jpegli)
- **lcms** : Gestion des couleurs Little CMS

> **Note pour les utilisateurs macOS/Linux** : macOS et Linux peuvent toujours utiliser `libaom` car les configurations NASM et CMake sont plus stables sur ces plateformes.

Vérifier l'installation :

```powershell
.\vcpkg list | Select-String "jxl|webp|openjpeg|jpeg|lcms"
```

## 10. Construire l'Application

1. Naviguez vers le répertoire app et installez les dépendances :

   ```powershell
   cd app
   pnpm install
   ```

2. Construisez et exécutez l'application en mode développement :

   ```powershell
   pnpm run dev:tauri
   ```

3. Pour une construction de production :

   ```powershell
   pnpm run build:tauri
   ```

L'application devrait maintenant se construire avec succès sur Windows. Si vous rencontrez des problèmes, assurez-vous que toutes les dépendances sont correctement installées et que les variables d'environnement sont correctement définies.

---

## Construction Croisée pour Windows Arm64

Vous pouvez construire en mode croisé pour Windows Arm64 (Windows on ARM) depuis une machine Windows x64.

### Prérequis

- Environnement de construction Windows x64 configuré comme décrit ci-dessus
- Dépendances vcpkg pour la cible Arm64

### 1. Ajouter la Chaîne d'Outils Rust

```powershell
rustup target add aarch64-pc-windows-msvc
```

### 2. Installer les Dépendances vcpkg pour Arm64

Créer un triplet de version pour Arm64 (si ce n'est pas déjà fait) :

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE arm64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\arm64-windows-static-release.cmake
```

Installer les dépendances :

```powershell
cd C:\vcpkg

# Note : aom et libavif[aom] ne sont plus nécessaires (utilisation de rav1e)
.\vcpkg install libjxl:arm64-windows-static-release
.\vcpkg install libwebp:arm64-windows-static-release
.\vcpkg install openjpeg:arm64-windows-static-release
.\vcpkg install libjpeg-turbo:arm64-windows-static-release
.\vcpkg install lcms:arm64-windows-static-release
```

### 3. Construire pour Arm64

```powershell
cd path\to\DropWebP\app
pnpm run build:tauri:windows-arm64
```

Ou construire manuellement :

```powershell
cd app\src-tauri
cargo build --release --target aarch64-pc-windows-msvc
cd ..
pnpm tauri build --target aarch64-pc-windows-msvc
```

### Remarques

- Les binaires Arm64 ne fonctionneront que sur les appareils Windows Arm64 (par exemple, Surface Pro X)
- Les binaires construits en mode croisé ne peuvent pas être exécutés sur des machines x64
- Les artefacts de construction sont générés dans `app/src-tauri/target/aarch64-pc-windows-msvc/release/`
