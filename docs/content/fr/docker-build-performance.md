# Analyse des performances de build Docker

Ce document explique les caractéristiques de performance des builds Docker sur différentes plateformes et pourquoi les builds de compilation croisée peuvent être lents.

## Comparaison des performances

### Temps de build réels

| Environnement hôte | Cible | Temps de build | Émulation QEMU |
|-------------------|-------|---------------|----------------|
| macOS (x64) | x64 Linux | 8-12 min | Non requis ✅ |
| macOS (Apple Silicon) | x64 Linux | 10-15 min | Rosetta 2 utilisé |
| Windows (x64) | x64 Linux | 10-15 min | Non requis ✅ |
| Windows (x64) | ARM64 Linux | **30-60 min** | Requis ❌ |

### Observation clé

**Pourquoi le build Windows x64 → ARM64 Linux est-il si lent ?**

Alors que les builds macOS x64 → x64 Linux sont rapides, les builds Windows x64 → ARM64 Linux sont extrêmement lents principalement en raison des **différences d'architecture**.

## Raisons techniques

### 1. Surcharge de l'émulation QEMU

#### Même architecture (rapide)

```bash
# macOS x64 → x64 Linux
Hôte : x86_64
Cible : x86_64-unknown-linux-gnu
Traitement : Exécution d'instructions natives
Vitesse : Vitesse quasi-native
```

#### Architecture croisée (lente)

```bash
# Windows x64 → ARM64 Linux
Hôte : x86_64
Cible : aarch64-unknown-linux-gnu
Traitement : Instructions ARM émulées via QEMU
Vitesse : 10-50x plus lent ❌
```

### 2. Surcharge du système de fichiers

#### Chemin I/O sous Windows

```
C:\Users\...\DropWebP (NTFS)
  ↓ Protocole réseau 9P
/mnt/c/Users/.../DropWebP (WSL 2)
  ↓ Docker bind mount
/workspace (Container)
```

#### Chemin I/O sous macOS

```
/Users/.../DropWebP (APFS)
  ↓ VirtioFS (optimisé)
/workspace (Container)
```

**VirtioFS** est un protocole de partage de fichiers haute vitesse conçu spécifiquement pour les environnements virtualisés :

- 2-5x plus rapide que 9P
- Cache de métadonnées efficace
- Particulièrement avantageux pour les grands projets

## Stratégies d'optimisation

### Approche recommandée

#### Phase de développement

```powershell
# Build uniquement x64 Linux (10-15 min)
pnpm run build:tauri:linux-x64
```

#### Phase de release

```yaml
# Builds parallèles sur GitHub Actions
jobs:
  build-x64:
    runs-on: ubuntu-latest
  build-arm64:
    runs-on: ubuntu-latest
```

## Résumé

### Points clés

✅ **Même architecture est rapide** : x64 → x64 prend 10-15 min
❌ **Architecture croisée est lente** : x64 → ARM64 prend 30-60 min
🚀 **Solution pratique** : x64 uniquement pendant le développement, GitHub Actions pour les releases

### Recommandation pour les développeurs

```bash
# Ceci est suffisant pour le travail quotidien
pnpm run build:tauri:linux-x64

# Automatiser pour les releases
git tag v3.2.1 && git push origin v3.2.1
```

Utilisez le temps et les ressources efficacement pour un développement sans stress !
