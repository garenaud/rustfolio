# hello-rs

Petit projet d'apprentissage Rust (WSL/Docker-friendly).

## Objectifs pédagogiques
- Types de base (i32, u32, f64, String, etc.)
- Fonctions pures et tests unitaires (`cargo test`)
- (À venir) Arguments CLI, gestion d'erreurs (`Result`)
- (À venir) Mini serveur web avec Axum

## Pré-requis
- Docker Desktop (WSL2 activé)
- `docker compose`

## Démarrer un shell de dev
```bash
docker compose run --rm dev
```

## Créer / lancer
```bash
cargo new hello-rs --bin
cd hello-rs
cargo run
cargo test
```

## Commandes utiles
- `cargo fmt` — formatage
- `cargo clippy -- -D warnings` — lint strict
- `cargo run -- Bob` — exécuter avec un argument (pour l'exo CLI)

## Structure
```
hello-rs/
  Cargo.toml
  src/
    main.rs
```

## Roadmap
- [x] Hello World
- [x] Variables, mutabilité, shadowing
- [x] Fonctions + tests (Exo 3)
- [ ] Arguments CLI (Exo 4)
- [ ] Gestion d’erreur avec Result (Exo 5)
- [ ] Serveur Axum (Hello + /health)
