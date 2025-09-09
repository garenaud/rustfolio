// Exercice 1: Les bases de Rust
// Variables, types de données, et opérations de base

fn main() {
    println!("🦀 Bienvenue dans l'exercice 1 : Les bases de Rust!");
    
    // 1. Variables et mutabilité
    demo_variables();
    
    // 2. Types de données
    demo_types();
    
    // 3. Opérations mathématiques
    demo_operations();
    
    // 4. Chaînes de caractères
    demo_strings();
}

fn demo_variables() {
    println!("\n=== Variables et mutabilité ===");
    
    // Variable immutable par défaut
    let x = 5;
    println!("Variable immutable x = {}", x);
    
    // Variable mutable
    let mut y = 10;
    println!("Variable mutable y = {}", y);
    y = 20;
    println!("Après modification, y = {}", y);
    
    // Shadowing (masquage)
    let z = 5;
    let z = z * 2;
    let z = z.to_string();
    println!("Après shadowing, z = '{}'", z);
}

fn demo_types() {
    println!("\n=== Types de données ===");
    
    // Entiers
    let entier_signe: i32 = -42;
    let entier_non_signe: u32 = 42;
    println!("Entier signé: {}, non signé: {}", entier_signe, entier_non_signe);
    
    // Flottants
    let flottant: f64 = 3.14159;
    println!("Nombre flottant: {}", flottant);
    
    // Booléens
    let vrai = true;
    let faux: bool = false;
    println!("Booléens: vrai = {}, faux = {}", vrai, faux);
    
    // Caractère
    let caractere = '🦀';
    println!("Caractère Unicode: {}", caractere);
    
    // Tuple
    let tuple: (i32, f64, char) = (42, 3.14, '🔥');
    println!("Tuple: {:?}", tuple);
    println!("Premier élément du tuple: {}", tuple.0);
    
    // Array (tableau)
    let tableau = [1, 2, 3, 4, 5];
    println!("Tableau: {:?}", tableau);
    println!("Longueur du tableau: {}", tableau.len());
}

fn demo_operations() {
    println!("\n=== Opérations mathématiques ===");
    
    let a = 10;
    let b = 3;
    
    println!("a = {}, b = {}", a, b);
    println!("Addition: {} + {} = {}", a, b, a + b);
    println!("Soustraction: {} - {} = {}", a, b, a - b);
    println!("Multiplication: {} × {} = {}", a, b, a * b);
    println!("Division: {} ÷ {} = {}", a, b, a / b);
    println!("Modulo: {} % {} = {}", a, b, a % b);
}

fn demo_strings() {
    println!("\n=== Chaînes de caractères ===");
    
    // String literal (str)
    let salut = "Salut";
    println!("String literal: {}", salut);
    
    // String owned
    let mut nom = String::from("Rust");
    println!("String owned: {}", nom);
    
    // Modification d'une String
    nom.push_str("acé");
    println!("Après modification: {}", nom);
    
    // Formatage
    let age = 25;
    let message = format!("J'apprends {} depuis {} jours!", nom, age);
    println!("Message formaté: {}", message);
    
    // Longueur
    println!("Longueur du message: {} caractères", message.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_operations_basiques() {
        assert_eq!(5 + 3, 8);
        assert_eq!(10 - 4, 6);
        assert_eq!(3 * 4, 12);
        assert_eq!(8 / 2, 4);
    }
    
    #[test]
    fn test_strings() {
        let mut s = String::from("Hello");
        s.push_str(", world!");
        assert_eq!(s, "Hello, world!");
    }
    
    #[test]
    fn test_tuple_access() {
        let tuple = (42, "Rust", true);
        assert_eq!(tuple.0, 42);
        assert_eq!(tuple.1, "Rust");
        assert_eq!(tuple.2, true);
    }
}