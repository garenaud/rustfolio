// Exercice 3: Propriété (Ownership) et Emprunt (Borrowing)
// Concepts fondamentaux de la gestion mémoire en Rust

fn main() {
    println!("🔒 Exercice 3 : Propriété et Emprunt en Rust!");
    
    // 1. Propriété (Ownership)
    demo_propriete();
    
    // 2. Emprunt (Borrowing)
    demo_emprunt();
    
    // 3. Références mutables
    demo_references_mutables();
    
    // 4. Slices
    demo_slices();
    
    // 5. Mini projet: Gestionnaire de texte
    gestionnaire_texte();
}

fn demo_propriete() {
    println!("\n=== Propriété (Ownership) ===");
    
    // Propriété avec des types simples (Copy trait)
    let x = 5;
    let y = x;  // x est copié, les deux variables sont valides
    println!("x = {}, y = {} (types Copy)", x, y);
    
    // Propriété avec String (pas de Copy trait)
    let s1 = String::from("Hello");
    let s2 = s1;  // s1 est "moved" vers s2, s1 n'est plus valide
    // println!("s1 = {}", s1);  // ❌ Erreur de compilation
    println!("s2 = {} (après move)", s2);
    
    // Clone pour dupliquer
    let s3 = String::from("World");
    let s4 = s3.clone();  // Copie explicite
    println!("s3 = {}, s4 = {} (après clone)", s3, s4);
    
    // Propriété dans les fonctions
    let texte = String::from("Bonjour");
    prendre_propriete(texte);
    // println!("texte = {}", texte);  // ❌ texte n'est plus valide
    
    let nombre = 42;
    copier_valeur(nombre);
    println!("nombre = {} (toujours valide)", nombre);
}

fn prendre_propriete(s: String) {
    println!("Fonction a reçu: {}", s);
}  // s sort du scope et est libéré

fn copier_valeur(n: i32) {
    println!("Fonction a reçu: {}", n);
}  // n sort du scope mais rien de spécial (Copy)

fn demo_emprunt() {
    println!("\n=== Emprunt (Borrowing) ===");
    
    let s = String::from("Hello, borrowing!");
    
    // Emprunt immutable
    let longueur = calculer_longueur(&s);
    println!("'{}' a {} caractères", s, longueur);  // s est toujours valide
    
    // Plusieurs emprunts immutables simultanés sont autorisés
    let reference1 = &s;
    let reference2 = &s;
    println!("ref1: {}, ref2: {}", reference1, reference2);
    
    // Emprunt avec différents types
    let nombres = vec![1, 2, 3, 4, 5];
    afficher_vec(&nombres);
    println!("nombres est toujours valide: {:?}", nombres);
}

fn calculer_longueur(s: &String) -> usize {
    s.len()
}  // s sort du scope mais ne possède pas la donnée

fn afficher_vec(v: &Vec<i32>) {
    println!("Vector contient: {:?}", v);
    for (i, valeur) in v.iter().enumerate() {
        println!("  Index {}: {}", i, valeur);
    }
}

fn demo_references_mutables() {
    println!("\n=== Références mutables ===");
    
    let mut s = String::from("Hello");
    println!("Avant modification: {}", s);
    
    // Une seule référence mutable à la fois
    modifier_string(&mut s);
    println!("Après modification: {}", s);
    
    // Les références mutables et immutables ne peuvent pas coexister
    let mut texte = String::from("Test");
    {
        let r1 = &mut texte;  // référence mutable
        r1.push_str(" modifié");
        println!("r1: {}", r1);
    }  // r1 sort du scope ici
    
    // Maintenant on peut créer une référence immutable
    let r2 = &texte;
    println!("r2: {}", r2);
    
    // Démonstration des vecteurs mutables
    let mut nombres = vec![1, 2, 3];
    ajouter_nombre(&mut nombres, 4);
    ajouter_nombre(&mut nombres, 5);
    println!("Nombres après ajouts: {:?}", nombres);
}

fn modifier_string(s: &mut String) {
    s.push_str(", world!");
}

fn ajouter_nombre(v: &mut Vec<i32>, n: i32) {
    v.push(n);
    println!("Ajouté {} au vecteur", n);
}

fn demo_slices() {
    println!("\n=== Slices ===");
    
    let s = String::from("Hello, wonderful world!");
    
    // Slices de string
    let hello = &s[0..5];
    let world = &s[16..21];
    println!("Slice 1: '{}', Slice 2: '{}'", hello, world);
    
    // Slice complète
    let slice_complete = &s[..];
    println!("Slice complète: '{}'", slice_complete);
    
    // Premier mot d'une phrase
    let premier = premier_mot(&s);
    println!("Premier mot: '{}'", premier);
    
    // Slices de tableaux
    let nombres = [1, 2, 3, 4, 5, 6];
    let slice_nombres = &nombres[1..4];
    println!("Slice du tableau: {:?}", slice_nombres);
    
    // Fonction qui trouve le plus grand nombre
    let max = trouver_max(slice_nombres);
    match max {
        Some(val) => println!("Plus grand nombre dans la slice: {}", val),
        None => println!("Slice vide"),
    }
}

fn premier_mot(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]  // Si pas d'espace, retourne toute la string
}

fn trouver_max(slice: &[i32]) -> Option<&i32> {
    if slice.is_empty() {
        None
    } else {
        let mut max = &slice[0];
        for item in slice.iter() {
            if item > max {
                max = item;
            }
        }
        Some(max)
    }
}

// Struct pour le mini projet
#[derive(Debug, Clone)]
struct Document {
    titre: String,
    contenu: String,
    taille: usize,
}

impl Document {
    fn new(titre: &str, contenu: &str) -> Self {
        Self {
            titre: titre.to_string(),
            contenu: contenu.to_string(),
            taille: contenu.len(),
        }
    }
    
    fn afficher_info(&self) {
        println!("📄 Document: '{}' ({} caractères)", self.titre, self.taille);
    }
    
    fn ajouter_texte(&mut self, texte: &str) {
        self.contenu.push_str(texte);
        self.taille = self.contenu.len();
    }
    
    fn obtenir_apercu(&self, max_chars: usize) -> &str {
        if self.contenu.len() <= max_chars {
            &self.contenu
        } else {
            &self.contenu[..max_chars]
        }
    }
}

fn gestionnaire_texte() {
    println!("\n=== 📝 Gestionnaire de Texte ===");
    
    let mut doc = Document::new(
        "Mon Premier Doc", 
        "Ceci est le contenu initial du document."
    );
    
    doc.afficher_info();
    
    // Afficher aperçu
    let apercu = doc.obtenir_apercu(20);
    println!("Aperçu (20 chars): '{}'...", apercu);
    
    // Modifier le document
    doc.ajouter_texte(" Voici du texte supplémentaire ajouté au document.");
    doc.afficher_info();
    
    // Analyse du contenu
    analyser_document(&doc);
    
    // Créer un deuxième document
    let doc2 = Document::new("Second Doc", "Un autre document plus court.");
    comparer_documents(&doc, &doc2);
}

fn analyser_document(doc: &Document) {
    println!("\n🔍 Analyse du document '{}':", doc.titre);
    
    let mots = compter_mots(&doc.contenu);
    let phrases = compter_phrases(&doc.contenu);
    
    println!("  - Caractères: {}", doc.taille);
    println!("  - Mots: {}", mots);
    println!("  - Phrases: {}", phrases);
}

fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

fn compter_phrases(texte: &str) -> usize {
    texte.matches('.').count()
}

fn comparer_documents(doc1: &Document, doc2: &Document) {
    println!("\n📊 Comparaison des documents:");
    println!("  '{}': {} caractères", doc1.titre, doc1.taille);
    println!("  '{}': {} caractères", doc2.titre, doc2.taille);
    
    if doc1.taille > doc2.taille {
        println!("  → '{}' est plus long", doc1.titre);
    } else if doc2.taille > doc1.taille {
        println!("  → '{}' est plus long", doc2.titre);
    } else {
        println!("  → Les documents ont la même taille");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculer_longueur() {
        let s = String::from("Hello");
        assert_eq!(calculer_longueur(&s), 5);
    }
    
    #[test]
    fn test_premier_mot() {
        assert_eq!(premier_mot("Hello world"), "Hello");
        assert_eq!(premier_mot("Rust"), "Rust");
        assert_eq!(premier_mot(""), "");
    }
    
    #[test]
    fn test_trouver_max() {
        let nombres = [1, 5, 3, 9, 2];
        assert_eq!(trouver_max(&nombres), Some(&9));
        assert_eq!(trouver_max(&[]), None);
        assert_eq!(trouver_max(&[42]), Some(&42));
    }
    
    #[test]
    fn test_document() {
        let mut doc = Document::new("Test", "Contenu initial");
        assert_eq!(doc.taille, 15);
        
        doc.ajouter_texte(" plus de texte");
        assert_eq!(doc.taille, 29);
        
        assert_eq!(doc.obtenir_apercu(7), "Contenu");
        assert_eq!(doc.obtenir_apercu(100), &doc.contenu);
    }
    
    #[test]
    fn test_compter_mots() {
        assert_eq!(compter_mots("Hello world"), 2);
        assert_eq!(compter_mots("Un deux trois quatre"), 4);
        assert_eq!(compter_mots(""), 0);
        assert_eq!(compter_mots("   spaces   everywhere   "), 2);
    }
}