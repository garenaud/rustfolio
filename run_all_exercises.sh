#!/bin/bash

# Script pour exécuter tous les exercices en séquence
# Usage: ./run_all_exercises.sh

echo "🦀 Démarrage de tous les exercices Rust..."
echo "========================================"

echo ""
echo "📚 Exécution de l'Exercice 1 - Les Bases"
echo "----------------------------------------"
cargo run -p exercice01-bases

echo ""
echo "🔥 Exécution de l'Exercice 2 - Fonctions et Contrôle"
echo "---------------------------------------------------"
cargo run -p exercice02-fonctions

echo ""
echo "🔒 Exécution de l'Exercice 3 - Propriété et Emprunt"
echo "--------------------------------------------------"
cargo run -p exercice03-propriete

echo ""
echo "✅ Tous les exercices terminés !"
echo ""
echo "💡 Pour exécuter les tests: cargo test"
echo "📖 Pour lire la documentation: cargo doc --open"