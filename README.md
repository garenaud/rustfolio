# Rustfolio — apprentissage Rust + Axum

Ce repo contient mes exercices et projets d'apprentissage Rust, en commençant par un Hello World CLI jusqu'à un serveur web avec Axum, puis un début de portfolio avec templates et JSON.

## Projets inclus
- **hello-rs** : apprentissage des bases Rust (variables, fonctions, tests, CLI, Result)
- **web-hello** : serveur web avec Axum (routes, JSON, POST, templates Askama, statiques)

---

## Pré-requis
- Docker Desktop (WSL2 activé)
- `docker compose`

---

## Utilisation en dev

### Démarrer un shell Rust dans Docker
```bash
docker compose up --build dev
```

Hot reload activé avec `cargo-watch`.

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
- Routes GET simples (`/`, `/health`)
- Path params (`/hello/:name`)
- Query params (`/greet?name=...`)
- Retourner du JSON avec `serde::Serialize`
- Recevoir du JSON (POST) avec `serde::Deserialize`
- Réponses HTTP avec codes (`201 Created`, `400 Bad Request`)
- Servir des fichiers statiques (CSS, images)
- Intégrer un moteur de templates (Askama)
- Charger des données JSON (expériences, projets, compétences) et les afficher

### Dépendances principales
Dans `Cargo.toml` :
```toml
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
askama = "0.12"
askama_axum = "0.4"
tower-http = { version = "0.5", features = ["fs"] }
chrono = { version = "0.4", default-features = false, features = ["clock"] }
```

### Exemple routes API
```bash
# Route santé
curl http://localhost:8080/health
# OK

# Path param
curl http://localhost:8080/hello/Alice
# Hello, Alice!

# Query param
curl "http://localhost:8080/greet?name=Bob"
# Hello, Bob!

# Retour JSON
curl http://localhost:8080/api/info
# {"status":"ok","app":"web-hello","version":"0.1.0"}

# POST JSON (echo)
curl -X POST http://localhost:8080/api/echo -H "Content-Type: application/json" -d '{"name":"Alice","age":30}'
# {"name":"Alice","age":30}

# POST JSON (register avec validations)
curl -i -X POST http://localhost:8080/api/register -H "Content-Type: application/json" -d '{"name":"Alice","age":30}'
# HTTP/1.1 201 Created
# {"id":1,"name":"Alice","age":30}
```

### Exemple templates
- `templates/base.html` : layout principal
- `templates/index.html` : page d’accueil avec nom, tagline, compétences, projets
- CSS statique dans `assets/css/style.css`

---

## Roadmap
- [x] Hello World CLI (`hello-rs`)
- [x] Variables / mutabilité / tests unitaires
- [x] Arguments CLI + gestion d’erreurs (`Result`)
- [x] Premier serveur Axum (`web-hello`)
- [x] Route `/health`
- [x] Retourner du JSON (`/api/info`)
- [x] Path params et Query params
- [x] POST JSON (echo + register avec validation et codes HTTP)
- [x] Servir des fichiers statiques (`/assets/...`)
- [x] Templates (Askama)
- [x] Charger du contenu JSON (expériences, projets, compétences)
- [ ] Pages projets/contact complètes
- [ ] Formulaire de contact
- [ ] Dockerisation prod multi-stage
- [ ] CI/CD GitHub Actions
