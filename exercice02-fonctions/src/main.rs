// Exercice 2: Fonctions et structures de contrôle
// Apprendre les fonctions, conditions, et boucles

fn main() {
    println!("🔥 Exercice 2 : Fonctions et structures de contrôle!");
    
    // 1. Fonctions de base
    demo_fonctions();
    
    // 2. Structures conditionnelles
    demo_conditions();
    
    // 3. Boucles
    demo_boucles();
    
    // 4. Pattern matching
    demo_pattern_matching();
    
    // 5. Mini projet: Calculatrice simple
    mini_calculatrice();
}

fn demo_fonctions() {
    println!("\n=== Fonctions ===");
    
    // Fonction sans paramètres ni retour
    dire_bonjour();
    
    // Fonction avec paramètres
    let resultat = additionner(5, 3);
    println!("5 + 3 = {}", resultat);
    
    // Fonction avec plusieurs types de retour
    let (somme, produit) = calculer(4, 6);
    println!("4 + 6 = {}, 4 × 6 = {}", somme, produit);
    
    // Expression vs statement
    let x = {
        let y = 3;
        y + 1  // Expression (pas de ;)
    };
    println!("Valeur du bloc: {}", x);
}

fn dire_bonjour() {
    println!("Bonjour depuis une fonction !");
}

fn additionner(a: i32, b: i32) -> i32 {
    a + b  // Retour implicite (expression)
}

fn calculer(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)  // Tuple comme retour
}

fn demo_conditions() {
    println!("\n=== Conditions ===");
    
    let nombre = 42;
    
    // if/else classique
    if nombre > 50 {
        println!("{} est plus grand que 50", nombre);
    } else if nombre > 30 {
        println!("{} est entre 30 et 50", nombre);
    } else {
        println!("{} est inférieur ou égal à 30", nombre);
    }
    
    // if comme expression
    let statut = if nombre % 2 == 0 { "pair" } else { "impair" };
    println!("{} est {}", nombre, statut);
    
    // Conditions complexes
    let age = 25;
    let a_permis = true;
    
    if age >= 18 && a_permis {
        println!("Peut conduire !");
    } else {
        println!("Ne peut pas conduire.");
    }
}

fn demo_boucles() {
    println!("\n=== Boucles ===");
    
    // Boucle loop (infinie)
    let mut compteur = 0;
    let resultat = loop {
        compteur += 1;
        if compteur == 5 {
            break compteur * 10;  // Retourne une valeur
        }
    };
    println!("Résultat de la boucle loop: {}", resultat);
    
    // Boucle while
    let mut n = 5;
    print!("Compte à rebours while: ");
    while n > 0 {
        print!("{} ", n);
        n -= 1;
    }
    println!("🚀");
    
    // Boucle for avec range
    print!("Boucle for 1-5: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
    
    // Boucle for avec collection
    let fruits = ["pomme", "banane", "orange"];
    println!("Fruits:");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  {}. {}", index + 1, fruit);
    }
}

#[derive(Debug)]
enum Jour {
    Lundi,
    Mardi,
    Mercredi,
    Jeudi,
    Vendredi,
    Samedi,
    Dimanche,
}

fn demo_pattern_matching() {
    println!("\n=== Pattern Matching ===");
    
    let jour = Jour::Mercredi;
    
    match jour {
        Jour::Lundi => println!("Début de semaine... 😴"),
        Jour::Mardi | Jour::Mercredi | Jour::Jeudi => {
            println!("Milieu de semaine avec {:?}", jour);
        }
        Jour::Vendredi => println!("Presque le weekend ! 🎉"),
        Jour::Samedi | Jour::Dimanche => println!("Weekend ! 🏖️"),
    }
    
    // Match avec nombres
    let numero = 13;
    match numero {
        1..=10 => println!("{} est entre 1 et 10", numero),
        11 | 12 => println!("{} est 11 ou 12", numero),
        13 => println!("Treize porte-bonheur ! 🍀"),
        _ => println!("Autre nombre: {}", numero),
    }
}

fn mini_calculatrice() {
    println!("\n=== 🧮 Mini Calculatrice ===");
    
    let operations = [
        (10.0, 5.0, '+'),
        (15.0, 3.0, '-'),
        (7.0, 6.0, '*'),
        (20.0, 4.0, '/'),
        (10.0, 0.0, '/'),  // Division par zéro
    ];
    
    for (a, b, op) in operations {
        match calculer_operation(a, b, op) {
            Some(resultat) => println!("{} {} {} = {}", a, op, b, resultat),
            None => println!("{} {} {} = Erreur (division par zéro)", a, op, b),
        }
    }
}

fn calculer_operation(a: f64, b: f64, operation: char) -> Option<f64> {
    match operation {
        '+' => Some(a + b),
        '-' => Some(a - b),
        '*' => Some(a * b),
        '/' => {
            if b != 0.0 {
                Some(a / b)
            } else {
                None  // Division par zéro
            }
        }
        _ => None,  // Opération non supportée
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_additionner() {
        assert_eq!(additionner(2, 3), 5);
        assert_eq!(additionner(-1, 1), 0);
    }
    
    #[test]
    fn test_calculer() {
        let (somme, produit) = calculer(3, 4);
        assert_eq!(somme, 7);
        assert_eq!(produit, 12);
    }
    
    #[test]
    fn test_calculer_operation() {
        assert_eq!(calculer_operation(10.0, 5.0, '+'), Some(15.0));
        assert_eq!(calculer_operation(10.0, 5.0, '-'), Some(5.0));
        assert_eq!(calculer_operation(10.0, 5.0, '*'), Some(50.0));
        assert_eq!(calculer_operation(10.0, 5.0, '/'), Some(2.0));
        assert_eq!(calculer_operation(10.0, 0.0, '/'), None);
        assert_eq!(calculer_operation(10.0, 5.0, '%'), None);
    }
    
    #[test]
    fn test_jour_debug() {
        let jour = Jour::Lundi;
        assert_eq!(format!("{:?}", jour), "Lundi");
    }
}