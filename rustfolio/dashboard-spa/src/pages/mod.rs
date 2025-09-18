// Déclare tous tes sous-modules de pages
pub mod account;
pub mod builder;
pub mod overview;
pub mod profile;
pub mod cv_form; // existe chez toi
pub mod nav;     // existe aussi chez toi (attention: autre "nav" que components::nav)

// Ne ré-exporte que ce qui est utilisé par le router
pub use account::Account;
pub use builder::Builder;
pub use overview::Overview;
pub use profile::Profile;

// ⚠️ On NE ré-exporte PAS cv_form::CvForm ici,
// car ton fichier ne semble pas définir un symbole exact "CvForm".
// Tu pourras l’ajouter plus tard si le nom du composant correspond.
