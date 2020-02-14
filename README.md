# libppm

**Membres du groupe:**
```
Merzouk OUMEDDAH
Juba-Saadi	SARNI
Aghylas SAI

Groupe 7 - Classe 4IABD1
```

**Présentation:**
```
Notre librairie libppm ,est une petite librairie Rust qui permet de manipuler le type d'image .ppm. On peut l'utiliser pour la création, la lecture et la transformation des images .ppm ainsi que de leur sous-structure 'Pixel'
```

**Utilisation:**
A importer dans le code où nous souhaitons utiliser la librairie
```
extern crate libppm

use libppm::Image
use libppm::Pixel
```

**Lancer des tests:**
```
utilisez "cargo test"  pour exécuter tous les tests disponibles dans notre librairie
utilisez "cargo test [function name]" pour exécuter le test de la fonction "function name"
```

**Lancer des benchs:**
```
utilisez "cargo bench"  pour exécuter tous les benchs disponibles dans notre librairie
utilisez "cargo bench [function name]" pour exécuter le bench de la fonction "function name"
```

**Lancer la  Doc:**
```
utilisez "cargo doc"  pour génerer la doc.
Elle se trouvera ensuite dans le dossier "./libppm/ppm/target/doc/libppm" 
```

**Exemple d'utilisation:**

```
Se positionner dans le dossier "src" et exécuter cargo run pour executer le programme exemple d'utilisation de la librairie et traiter quelques images.

```
