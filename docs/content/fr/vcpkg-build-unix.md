# Guide vcpkg (macOS/Linux)

Ce document decrit la configuration de vcpkg pour lier statiquement les dependances C/C++ sur macOS et Linux.

## Plateformes

- macOS: x64 (Intel) et ARM64 (Apple Silicon)
- Linux: x64 et ARM64

## Prerequis

### macOS

- Xcode Command Line Tools: `xcode-select --install`
- Rust (rustup recommande)
- Git

### Linux

- GCC/Clang
- Rust (rustup recommande)
- Git
- Outils de build: `sudo apt install build-essential curl zip unzip tar pkg-config`

## Installer vcpkg

```bash
git clone https://github.com/Microsoft/vcpkg.git ~/vcpkg
cd ~/vcpkg
./bootstrap-vcpkg.sh

export VCPKG_ROOT="$HOME/vcpkg"
export PATH="$VCPKG_ROOT:$PATH"
```

Ajoutez ces variables dans `~/.bashrc` ou `~/.zshrc`.

## Installer les dependances

### Methode automatique (recommandee)

```bash
cd app/src-tauri
chmod +x setup-vcpkg.sh
./setup-vcpkg.sh
```

Triplets utilises automatiquement:

- macOS ARM64: `arm64-osx`
- macOS x64: `x64-osx`
- Linux ARM64: `arm64-linux`
- Linux x64: `x64-linux`

### Methode manuelle (exemple x64-linux)

```bash
vcpkg install aom:x64-linux
vcpkg install libavif[aom]:x64-linux
vcpkg install libjxl:x64-linux
vcpkg install libwebp:x64-linux
vcpkg install openjpeg:x64-linux
vcpkg install libjpeg-turbo:x64-linux
vcpkg install lcms:x64-linux
```

## Build

```bash
export VCPKG_ROOT="$HOME/vcpkg"
cd app/src-tauri
cargo build --release
```

## Bibliotheques principales

- libaom
- libavif
- libjxl
- libwebp
- openjpeg
- libjpeg-turbo
- lcms

## Depannage

### `vcpkg: command not found`

- Verifier `VCPKG_ROOT`
- Verifier `PATH`

```bash
echo $VCPKG_ROOT
which vcpkg
```

### Erreur "library not found"

```bash
vcpkg list | grep aom
vcpkg list | grep avif
vcpkg list | grep jxl
```

### Erreur `xxx.h file not found`

- Verifier le triplet
- Verifier l'installation des paquets

```bash
uname -m
ls -la ~/vcpkg/installed/
```

## vcpkg vs Homebrew/apt

### Avantages vcpkg

1. Procedure unifiee multi-plateforme
2. Gestion des versions
3. Binaire plus portable (lien statique)
4. Gestion centralisee des dependances C/C++

### Avantages Homebrew/apt

1. Installation plus simple
2. Meilleur partage des bibliotheques systeme
3. Mises a jour gerees par l'OS

## Liens

- [vcpkg](https://github.com/Microsoft/vcpkg)
- [vcpkg-rs](https://github.com/mcgoo/vcpkg-rs)
- [docker-build.md](docker-build.md)
