# Guide des formats pris en charge

## Formats d'entrée pris en charge

Drop Compress Image prend en charge les formats d'image d'entrée suivants:

- AV1 Image Format (`*.avif`)
- Microsoft Windows Bitmap Image (`*.bmp`)
- Direct Draw Surface (`*.dds`)
- Farbfeld (`*.ff`)
- Graphics Interchange Format (`*.gif`)
- Radiance High Dynamic Range image file (`*.hdr`)
- Computer icon encoded in ICO file format (`*.ico`)
- Joint Photographic Experts Group (`*.jpg`, `*.jpeg`)
- OpenEXR image (`*.exr`)
- Portable Network Graphic (`*.png`)
- Portable Any Map (`*.pnm`)
- Quite OK Image Format (`*.qoi`)
- Truevision Graphics Adapter (`*.tga`)
- Tagged Image File Format (`*.tif`, `*.tiff`)
- WebP (`*.webp`)
- JPEG 2000 (`*.jp2`, `*.j2c`, `*.j2k`, `*.jpf`, `*.jpx`, `*.jpm`, `*.mj2`, `*.jph`)
- JPEG XL (`*.jxl`)

## Formats de sortie pris en charge

Cette section décrit les formats de sortie pris en charge par ce programme.

### PNG (Oxipng)

PNG (Portable Network Graphics) est un format qui permet d'enregistrer des images sans perte de qualité.
Ce programme utilise Zopfli, une technologie de compression spéciale développée par Google qui réduit la taille des fichiers PNG, mais dont le traitement est plus long.

#### Fonctionnalités PNG

- **Compression non destructive (sans perte)** : Aucune perte de la qualité d’image d’origine, quel que soit le nombre d’enregistrements.
- **Transparence (Alpha)** : Permet un arrière-plan transparent.

#### Scénarios d’utilisation du PNG

- **Logos, icônes et graphiques** : Images avec des bordures nettes et peu de couleurs, ou images nécessitant un arrière-plan transparent.
- **Contenu web** : Utilisé pour éviter la dégradation de la qualité d’image.
- **Conserver les données d’origine** : Images susceptibles d’être modifiées à plusieurs reprises.

### JPEG (compression jpegli)

Le format JPEG (Joint Photographic Experts Group) est principalement utilisé pour les photographies, privilégiant la « petite taille, même au détriment de la qualité d'image ». jpegli, utilisé dans ce programme, est une technologie développée par le projet libjxl (le même que JPEG XL) pour réduire davantage la taille des fichiers JPEG tout en minimisant la dégradation visuelle. C'est un encodeur JPEG de haute qualité utilisant la technologie JPEG XL.

#### Caractéristiques du JPEG

- **Compression avec perte** : Pour réduire la taille du fichier, les informations difficiles à voir à l'œil nu sont supprimées, ce qui entraîne une légère dégradation de la qualité de l'image (plus le taux de compression est élevé, plus la dégradation est perceptible).
- **Taux de compression élevé** : Les images complexes contenant de nombreuses couleurs, comme les photographies, peuvent être réduites de manière extrêmement réduite.
- **jpegli** : Comparé au JPEG traditionnel, il permet d'obtenir une meilleure qualité d'image pour une taille de fichier identique, et même des tailles de fichier plus petites pour une qualité d'image identique. Il utilise la même technologie que JPEG XL, permettant une compression de haute qualité.

#### Scénarios d’utilisation du JPEG

- **Photos générales** : Images avec de nombreuses couleurs et dégradés, comme les paysages et les portraits.
- **Photos volumineuses sur les sites web** : Lorsque la taille du fichier est la priorité absolue pour un chargement plus rapide.

### WebP

WebP est un nouveau format d’image développé par Google qui combine les meilleures fonctionnalités du JPEG et du PNG.

#### Fonctionnalités WebP

- **Compression élevée et qualité d'image élevée** : Qualité d'image supérieure à celle du PNG, avec le même taux de compression que le JPEG.
- **Prise en charge avec/sans perte** : Prise en charge de la compression avec ou sans perte (pour les photos)
- **Transparence (Alpha) et Animation** : Prise en charge de la transparence d'arrière-plan comme le PNG et des animations comme le GIF (qualité d'image supérieure et fichiers plus petits que le GIF).

#### Scénarios d'utilisation de WebP

- **Images générales de sites web** : Remplacez la plupart des images, y compris les photos, les icônes et les animations, par ce format pour optimiser la vitesse de votre site.
- **Format d'image universel** : Utile lorsque vous ne souhaitez pas vous soucier du choix du format de fichier.

### AVIF

AVIF (AV1 Image Format) est un format d'image nouvelle génération plus récent que WebP. Il a été créé à l'aide d'une technologie vidéo haute qualité (appelée AV1).

#### Fonctionnalités AVIF

- **Taux de compression actuel de premier ordre** : Permet d'obtenir des fichiers plus petits avec une qualité d'image équivalente à celle des formats JPEG ou WebP.
- **Représentation des couleurs haute qualité** : Prise en charge du HDR (High Dynamic Range) et d'une large gamme de couleurs, pour un contraste plus riche et des couleurs éclatantes.
- **Transparence (Alpha) et Animation** : Prise en charge de la transparence et de l'animation.

#### Scénarios d'utilisation AVIF

- **Contenu Web nécessitant une qualité d'image élevée** : Idéal pour obtenir une qualité d'image élevée et des temps de chargement rapides, par exemple pour les vignettes de Netflix.
- **Vitesse de site Web supérieure** : À prendre en compte pour des vitesses encore plus rapides que celles du WebP.

### JPEG XL

JPEG XL est un nouveau format développé pour pallier les lacunes de l'ancienne norme JPEG et devenir le format d'image ultime.

#### Fonctionnalités de JPEG XL

- **Haute compatibilité** : Convertissez des images JPEG existantes en formats plus petits avec une perte de qualité minimale, voire nulle.
- **Prise en charge complète** : Obtenez une compression et une qualité d'image élevées pour tout type d'image, y compris les photos, les illustrations et les graphiques.
- **Multifonctionnalités** : Prend en charge la compression avec et sans perte, la transparence (alpha), l'animation et des couleurs haute qualité (large gamme de couleurs et HDR).
- **Affichage progressif** : Les images gagnent progressivement en netteté, réduisant ainsi le temps d'attente, notamment pour les images volumineuses.

#### Cas d'utilisation de JPEG XL

- **Unification des formats d'image** : Ce format unique devrait à terme remplacer de nombreux formats, notamment JPEG, PNG et GIF.
- **Archivage d'images** : Idéal pour enregistrer des images JPEG en format plus petit sans dégrader l'original.

## Résumé des formats de sortie

| Format | Principaux avantages | Utilisations recommandées | Avec perte | Sans perte | Alpha | HDR | Taux de compression | Charge de compression |

| --------------- | ---------------------------------------------------- | ------------------------------------------------------ | -------- | -------- | --- | ------ | -------- |

| PNG (Oxipng) | Aucune dégradation de l'image, transparence du fond. | Logos, icônes, diagrammes. | ❌️ | ✅️ | ✅️ | ❌️ | Faible | Élevée |

| JPEG (jpegli) | Possibilité de réduire considérablement la taille des photos avec compression de haute qualité. | Images couramment utilisées sur le web. | ✅️ | ❌️ | ❌️ | ❌️ | Élevée | Faible |

| WebP | Compression élevée, haute qualité, prise en charge de la transparence et des animations. | Images à usage général pour les sites web. | ✅️ | ✅️ | ✅️ | ❌️ | Moyen | Moyen |

| AVIF | **Taux de compression maximal et haute qualité d'image**, prise en charge d'une large gamme de couleurs (HDR). | Pour les sites web exigeant à la fois une haute qualité d'image et un affichage rapide. | ✅️ | ❌️[^1] | ✅️ | ✅️ | Élevé | Élevé |

| JPEG XL | **Le format ultime**, compatible avec tous les formats et avec JPEG. | Pour réduire la taille des fichiers JPEG existants et unifier les formats d'image à l'avenir. | ✅️ | ✅️ | ✅️ | ✅️ | Élevé | Faible |

[^1]:
    Le format AVIF est sans perte par définition, mais ce programme ne l'implémente pas en raison de sa charge élevée et de la taille importante des fichiers.
    Pour une utilisation sans perte, nous recommandons l'utilisation de JPEG XL. Référence : <https://github.com/AOMediaCodec/av1-avif/issues/111>
