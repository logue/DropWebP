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

## 8. Configurer vcpkg (Instructions officielles)

Exécutez la commande suivante dans le répertoire de votre choix pour cloner vcpkg.

```powershell
git clone https://github.com/microsoft/vcpkg.git
```

Naviguez vers le répertoire vcpkg et exécutez la commande de configuration.

```powershell
cd vcpkg
.\bootstrap-vcpkg.bat
```

Ajoutez le chemin vers vcpkg.exe (par exemple `C:\path\to\vcpkg`) à votre variable d'environnement `PATH`. Comment faire : Ajoutez le répertoire contenant vcpkg.exe à la variable d'environnement Path.

De plus, ajoutez une variable d'environnement `VCPKG_DEFAULT_TRIPLET` et définissez-la sur `x64-windows-static-md`. Cela garantit que les bibliothèques statiques pour Windows 64 bits sont installées par défaut.

Après l'installation, vérifiez que le chemin est défini et vérifiez la version avec la commande ci-dessous.

```powershell
vcpkg version
```

> **Avertissement :** vcpkg peut ne pas fonctionner correctement si son chemin contient des caractères non alphanumériques. Il est recommandé de le cloner à un emplacement comme la racine d'un lecteur.

## 9. Installer les bibliothèques requises

Exécutez la commande suivante pour installer les bibliothèques requises pour la conversion d'images.

```powershell
vcpkg install libavif libjxl libwebp libjpeg-turbo libpng
```

> **Remarque :** JPEG XL (`libjxl`) est lié statiquement via la fonctionnalité `vendored`, donc vcpkg pourrait ne plus être nécessaire à l'avenir. L'installation peut prendre un certain temps, et certaines bibliothèques peuvent échouer à se construire. Dans ce cas, veuillez vous référer à la documentation de vcpkg.

## 10. Installer les dépendances et construire

Installez les dépendances dans le répertoire du projet.

```powershell
pnpm install
```

Démarrez l'application en mode développement.

```powershell
cd app
pnpm tauri dev
```

> **Terminé :** Le premier démarrage peut prendre un certain temps pour compiler les dépendances Rust. Une fois que la fenêtre de l'application apparaît, la configuration de l'environnement de développement est terminée.
