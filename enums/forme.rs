/*

Définis un enum Forme avec trois variantes :

Cercle(f64) — le f64 est le rayon,
Rectangle(f64, f64) — largeur et hauteur,
Carre(f64) — le côté.

Écris une fonction aire qui reçoit une &Forme et retourne un f64 (l'aire). Utilise un match pour calculer selon la variante :

cercle : π × rayon²,
rectangle : largeur × hauteur,
carré : côté × côté.

Dans main, crée une forme de chaque type, appelle aire sur chacune, et affiche le résultat.
4. Résultat attendu (à l'exécution, avec par exemple rayon 2.0, rectangle 3.0 × 4.0, carré 5.0)
Aire du cercle : 12.566370614359172
Aire du rectangle : 12
Aire du carré : 25

*/


enum Forme {
	Cercle(f64),
	Rectangle(f64, f64),
	Carre(f64),
}

fn aire (f: &Forme) -> f64 {

	match f {
		Forme::Cercle(r) => std::f64::consts::PI * r * r,
		Forme::Rectangle(l,h) => l * h,
		Forme::Carre(c) => c * c
	}		

}

println!("Air du rectangle : {:?}", aire(Forme::Carre(3.0) ) );

