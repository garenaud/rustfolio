# 🦀 Rustfolio - Parcours d'Apprentissage Rust

Un repository structuré pour apprendre Rust étape par étape, avec pour objectif final de construire un portfolio complet.

## 🎯 Objectifs

- **Phase 1**: Maîtriser les concepts fondamentaux de Rust à travers des exercices pratiques
- **Phase 2**: Développer des projets plus complexes (CLI tools, web services, etc.)
- **Phase 3**: Construire un portfolio complet showcasing les compétences Rust acquises

## 📚 Progression d'Apprentissage

### ✅ Exercices Fondamentaux Complétés

#### 🟢 Exercice 1 - Les Bases (`exercice01-bases`)
**Concepts couverts :**
- Variables et mutabilité (`let`, `mut`, shadowing)
- Types de données primitifs (entiers, flottants, booléens, caractères)
- Collections de base (tuples, arrays)
- Chaînes de caractères (`&str` vs `String`)
- Opérations arithmétiques de base

**Pour exécuter :**
```bash
cargo run -p exercice01-bases
```

#### 🟠 Exercice 2 - Fonctions et Contrôle (`exercice02-fonctions`)
**Concepts couverts :**
- Définition et appel de fonctions
- Paramètres et valeurs de retour
- Structures conditionnelles (`if`, `match`)
- Boucles (`loop`, `while`, `for`)
- Pattern matching avec `enum`
- Mini-projet : Calculatrice simple

**Pour exécuter :**
```bash
cargo run -p exercice02-fonctions
```

#### 🔴 Exercice 3 - Propriété et Emprunt (`exercice03-propriete`)
**Concepts couverts :**
- Système de propriété (ownership)
- Emprunt (borrowing) et références (`&`, `&mut`)
- Slices et leur utilisation
- Gestion de la mémoire sans garbage collector
- Mini-projet : Gestionnaire de texte

**Pour exécuter :**
```bash
cargo run -p exercice03-propriete
```

## 🛠️ Structure du Projet

```
rustfolio/
├── Cargo.toml              # Configuration workspace
├── README.md               # Ce fichier
├── exercice01-bases/       # Exercice 1: Variables, types, opérations
│   ├── Cargo.toml
│   └── src/main.rs
├── exercice02-fonctions/   # Exercice 2: Fonctions, contrôle de flux
│   ├── Cargo.toml
│   └── src/main.rs
└── exercice03-propriete/   # Exercice 3: Ownership, borrowing
    ├── Cargo.toml
    └── src/main.rs
```

## 🚀 Commandes Utiles

### Exécuter tous les tests
```bash
cargo test
```

### Exécuter un exercice spécifique
```bash
cargo run -p [nom-exercice]
```

### Compiler tout le workspace
```bash
cargo build
```

### Vérifier la syntaxe sans compiler
```bash
cargo check
```

## 📋 Prochaines Étapes

### 🔄 Exercices à Venir
- [ ] **Exercice 4 - Structs et Implémentations** : Structures, méthodes, traits de base
- [ ] **Exercice 5 - Enums et Pattern Matching** : Enums complexes, `Option`, `Result`
- [ ] **Exercice 6 - Collections** : `Vec`, `HashMap`, `BTreeMap`, itérateurs
- [ ] **Exercice 7 - Gestion d'Erreurs** : `Result`, `Option`, propagation d'erreurs
- [ ] **Exercice 8 - Traits et Génériques** : Définition de traits, génériques, lifetime
- [ ] **Exercice 9 - Modules et Packages** : Organisation du code, crates, visibilité

### 🏗️ Projets Portfolio (Phase 2)
- [ ] **CLI Tool** : Outil en ligne de commande (ex: gestionnaire de tâches)
- [ ] **API Web** : Service REST avec framework comme Actix-web ou Axum
- [ ] **Analyse de Données** : Parser et analyser des fichiers CSV/JSON
- [ ] **Game Simple** : Jeu console (ex: Jeu du pendu, Pierre-papier-ciseaux)
- [ ] **Crypto/Hash** : Implémentation d'algorithmes cryptographiques

### 🌟 Portfolio Final (Phase 3)
- [ ] Site web statique généré présentant tous les projets
- [ ] Documentation complète avec exemples
- [ ] Benchmarks et tests de performance
- [ ] Intégration continue (CI/CD)
- [ ] Publication de crates sur crates.io

## 💡 Conseils d'Apprentissage

1. **Lisez attentivement les messages du compilateur** - Rust a d'excellents messages d'erreur
2. **Expérimentez** - Modifiez le code des exercices pour voir ce qui se passe
3. **Utilisez `cargo doc --open`** pour explorer la documentation
4. **Consultez le [Rust Book](https://doc.rust-lang.org/book/)** pour approfondir
5. **Pratiquez régulièrement** - Un peu chaque jour vaut mieux que beaucoup d'un coup

## 🔗 Ressources Utiles

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Petits exercices interactifs
- [Rust Std Documentation](https://doc.rust-lang.org/std/)
- [Crates.io](https://crates.io/) - Registry des packages Rust

---

*Commencé le : [Date]*  
*Dernière mise à jour : [Date]*
