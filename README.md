# libppm

**Membres du groupe:**
```
Merzouk OUMEDDAH
Juba	SARNI
Aghylas SAI
```

**Présentation:**
```
Notre librairie libppm ,est une petite librairie Rust qui permet de manipuler le type d'image .ppm, on peut l'utiliser pour la création la lecture et la transformation des image .ppm ainsi que de leur sous-structure 'Pixel'
```
**Utilisation:**
```
extern crate libppm

avant la main()

use libppm::Image
use libppm::Pixel
pour utiliser Image ou Pixel dans notre programme main()

```
**Lancer des tests:**
```
utilisez cargo test  pour exécuter tous les tests disponibles dans notre librairie
utilisez cargo test "function name" pour exécuter le test de la fonction "function name"
```
**Lancer des benchs:**
```
utilisez cargo bench  pour exécuter tous les benchs disponibles dans notre librairie
utilisez cargo bench "function name" pour exécuter le bench de la fonction "function name"

```
**Lancer la  Doc:**
```
utilisez cargo doc  pour génerer la doc dans le dossier libppm/ppm/target/doc/libppm toutes la doc de notre librairie

```
**Lancer la  main:**

```
se positionner dans le dossier src et exécuter cargo run

```
