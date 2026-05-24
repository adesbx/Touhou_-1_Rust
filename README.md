# Touhou--1---Rust-

Un shoot 'em up (Shmup) rétro au style pixel art inspiré de l'univers Touhou, développé en Rust avec le moteur de jeu **Bevy (v0.15)**. 

Affrontez des vagues d'Anges et de Cherubs aux patterns de déplacement uniques, gérez vos points de vie en temps réel et préparez-vous à affronter le Boss final !

## 💡 Contexte & Crédits (Solo Project)

Ce jeu a été développé dans un but d'**apprentissage pour découvrir le langage Rust** et la logique ECS (Entity Component System) du moteur Bevy.

* **Assets graphiques :** 100% fait maison ! Tous les sprites, les feuilles d'animations (spritesheets), le décor de fond ont été dessinés et intégrés par mes soins.
* **Audio :** Les musiques sont égalements faites maisons, les seuls éléments externes non réalisés à la main sont les bj'ruitages de type **SFX** (sons de tirs, impacts, explosions).

### 🎵 Crédits Audio (SFX)
Afin de donner vie aux affrontements, le projet intègre les bruitages externes suivants :
* **Bruit de tir (Joueur) :** [Laser Shoot - wolferCZ (Freesound)](https://freesound.org/people/wolferCZ/sounds/464553/)
* **Bruit de tir Boule de Feu (Joueur) :** [Fire Magic - Pixabay](https://pixabay.com/fr/sound-effects/films-et-effets-sp%c3%a9ciaux-fire-magic-5-378639/)
* **Bruit de tir Électrique (Boss) :** [Elemental Spell Impact Electric - Pixabay](https://pixabay.com/fr/sound-effects/la-technologie-elemental-spell-impact-electric-448567/)
* **Bruit de tir Arme Vortex (Espace) :** [Vortex Shoot - D4XX (Freesound)](https://freesound.org/people/D4XX/sounds/617043/)
* **Bruit de tir Ennemi classique :** [Robotic Shoot - Pixabay](https://pixabay.com/fr/sound-effects/films-et-effets-sp%c3%a9ciaux-robotic-shoot-138432/)
* **Bruit d'explosion (Bombe) :** [Bomb Explosion - Anomaex (Freesound)](https://freesound.org/people/Anomaex/sounds/490266/)

## 🚀 Comment lancer le jeu

### ⚠️ Note importante sur la fenêtre
Le jeu est actuellement optimisé pour une **petite fenêtre fixe (640x360 ou format arcade)**. 
* Les éléments de l'interface (HUD, barre de vie) utilisent des positions absolues.
* **Ne redimensionnez pas et n'agrandissez pas la fenêtre**, sous peine de décaler l'affichage des textes de l'interface (HP, etc.).

### Commandes de terminal
Assurez-vous d'avoir installé [Rust](https://www.rust-lang.org/). À la racine du projet, exécutez :

```bash
# Lancer le jeu en mode développement
cargo run

# Lancer le jeu avec les optimisations (recommandé pour un framerate stable)
cargo run --release

```

---

## 🎮 Commandes du jeu (Gameplay)

Le jeu utilise une configuration classique de type clavier "ZQSD + Action".

| Touche | Action | Description |
| --- | --- | --- |
| **Z** / **S** | **Haut / Bas** | Déplacer le joueur verticalement |
| **Q** / **D** | **Gauche / Droite** | Déplacer le joueur horizontalement |
| **K** | **Tirer** | Lance des projectiles en ligne droite |
| **L** | **Bombe** | Utilise une bombe pour tuer les ennemis proches |
| **T** | **Mute Musique** | Coupe / Active la musique de fond à tout moment |
| **Y** | **Voir la Hitbox** | Active ou Désactive le carré symbolisant la hitbox du joueur |

---

## 👾 Mécaniques de jeu & Contenu

* **Système de Vagues :** Progression rythmée à travers plusieurs vagues d'ennemis juste avant le Boss.
* **Graphismes Pixel-Art :** Rendu net ("crispy") assuré via `ImagePlugin::default_nearest()` et la désactivation du MSAA (`Msaa::Off`) directement sur la caméra pour éviter le flou sur les sprites et la police *PressStart2P*.
* **Power-Ups :** Les ennemis ont une chance sur trois (1/3) de faire apparaître un item de puissance à leur mort. Ces mêmes items peuvent spawn de manière  aléatoire durant la partie. Ramasser ces items débloque différents niveaux de tir améliorés. De plus, la cadence de tir (vitesse) augmente dynamiquement en scalant directement sur votre niveau de Power-Up actuel.


## 🛠️ Architecture du code

Le projet est découpé de manière modulaire (ECS - Entity Component System) :

* `components.rs` : Définition des structures de données (`Health`, `Enemy`, `Boss`, `Player`, `...`).
* `constants.rs` : Centralisation des tailles, vitesses et configurations de jeu.
* `player.rs` (ou `Player Plugin`) : On y retrouve toutes les fonctions en lien avec le joueur (gestion des entrées clavier ZQSD, tirs, utilisation des bombes et évolution du système de Power-Up).
* `...`
