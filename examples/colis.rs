/*

Déclare une enum Colis avec trois variants :

Lettre(f64) — le poids en grammes
Boite(f64, f64, f64) — la longueur, la largeur et la hauteur en centimètres
Palette(u32) — le nombre de cartons

Écris une fonction prix(colis: &Colis) -> f64 qui calcule le tarif d'envoi :

une lettre : 1.50 € de base, plus 0.02 € par gramme
une boîte : 3.00 € de base, plus 0.50 € par litre de volume (1 litre = 1000 cm³)
une palette : 25.00 € de base, plus 4.00 € par carton

Dans main, crée une lettre de 120 g, une boîte de 30 x 20 x 10 cm, une palette de 12 cartons, et affiche le prix de chacune à deux décimales.

Résultat attendu :

Prix de la lettre : 3.90 €
Prix de la boîte : 6.00 €
Prix de la palette : 73.00 €

*/

enum Colis {
    Lettre(f64), // le poids en grammes,
    Boite(f64, f64, f64), // Longueur, Largeur, Hauteur en cm
    Palette(u32), // nbr de cartons
}

fn prix(colis: &Colis) -> f64 { // Le tarif d'envoi 
    match colis {
        Colis::Lettre(poid) => 1.50 + (0.02 * poid),
        Colis::Boite(lo, la, ha) => 3.00 + 0.50 * ((lo * la * ha) / 1000.0),
        Colis::Palette(carton) => 25.00 + (4.0 * (*carton as f64)),
    }
}

fn main() {
    println!("Prix de la lettre : {:.2} €", prix(&Colis::Lettre(120.0)));
    println!("Prix de la boite : {:.2} €", prix(&Colis::Boite(30.0, 20.0, 10.0)));
    println!("Prix de la palette : {:.2} €", prix(&Colis::Palette(12)));
}
