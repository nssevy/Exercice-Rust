/*
Déclare une struct classique Trajet avec quatre champs :

depart: String
arrivee: String
distance_km: f64
duree_h: f64

Écris une fonction libre vitesse_moyenne(t: &Trajet) -> f64 qui renvoie la vitesse moyenne (distance divisée par durée).

Dans main, crée un trajet Paris vers Lyon, 465 km en 4.5 h, et affiche une ligne de résumé avec la vitesse à deux décimales.

Résultat attendu :

Paris -> Lyon : 465 km en 4.5 h, vitesse moyenne 103.33 km/h

*/

struct Trajet {
    depart : String,
    arrivee : String,
    distance_km : f64,
    duree_h : f64,
}

fn vitesse_moyenne(t: &Trajet) -> f64 {
    t.distance_km / t.duree_h
}

fn main() {

    let pl /* pour paris lyon*/ = Trajet {
        depart: String::from("Paris"),
        arrivee: String::from("Lyon"),
        distance_km: 465.0,
        duree_h: 4.5,
    };

    println!("{} -> {} : {} km en {} h, vitesse moyenne {:.2} km/h", pl.depart, pl.arrivee, pl.distance_km, pl.duree_h, vitesse_moyenne(&pl));
}
