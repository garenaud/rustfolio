# Rustfolio — apprentissage Rust + Axum

Ce repo contient mes exercices et projets d'apprentissage Rust, en commençant par un Hello World CLI jusqu'à un serveur web avec Axum.

## Projets inclus
- **hello-rs** : apprentissage des bases Rust (variables, fonctions, tests, CLI, Result)
- **web-hello** : premier serveur web avec Axum

---

## Pré-requis
- Docker Desktop (WSL2 activé)
- `docker compose`

---

## Utilisation en dev

### Démarrer un shell Rust dans Docker
```bash
docker compose run --rm --service-ports dev
```

### Créer un nouveau projet
```bash
cargo new <nom-projet> --bin
cd <nom-projet>
```

---

## Projet 1 : hello-rs

Objectifs :
- Types de base (i32, u32, f64, String…)
- Fonctions pures et tests unitaires (`cargo test`)
- Arguments CLI (`std::env::args`)
- Gestion d’erreur propre (`Result`, `eprintln!`, `exit code`)

Exemple :
```bash
cargo run -- Bob
# Hello, Bob!
```

---

## Projet 2 : web-hello (Axum)

Objectifs :
- Découverte d’Axum + Tokio
- Première route HTTP (`/`)
- Route de santé (`/health`)

### Dépendances
Dans `Cargo.toml` :
```toml
axum = "0.7"
tokio = { version = "1", features = ["full"] }
```

### Exemple minimal
```rust
use axum::{routing::get, Router, serve};
use tokio::net::TcpListener;
use std::net::SocketAddr;

async fn hello() -> &'static str { "Hello, Rust! 🚀" }
async fn health() -> &'static str { "OK" }

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/health", get(health));

    let addr = SocketAddr::from(([0,0,0,0], 8080));
    println!("listening on http://{addr}");

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
```

### Lancer le serveur
```bash
cargo run
```

Puis tester :
```bash
curl http://localhost:8080/
# Hello, Rust! 🚀

curl http://localhost:8080/health
# OK
```

---

## Roadmap
- [x] Hello World CLI (`hello-rs`)
- [x] Variables / mutabilité / tests unitaires
- [x] Arguments CLI + gestion d’erreurs (`Result`)
- [x] Premier serveur Axum (`web-hello`)
- [x] Route `/health`
- [ ] Retourner du JSON
- [ ] Servir des fichiers statiques
- [ ] Templates (Askama/Tera)
- [ ] Formulaire de contact
- [ ] Dockerisation prod
- [ ] CI/CD GitHub Actions
