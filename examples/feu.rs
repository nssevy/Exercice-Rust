/*

Définis un enum Feu avec trois variantes : Rouge, Orange, Vert.
Écris ensuite une fonction afficher_conseil qui reçoit un Feu en paramètre et qui affiche un conseil selon la variante, en utilisant un match :

Rouge → Arrête-toi
Orange → Ralentis
Vert → Passe

Dans main, appelle la fonction sur les trois variantes.
4. Résultat attendu (à l'exécution)
Arrête-toi
Ralentis
Passe

*/

enum Feu {
	Rouge,
	Orange,
	Vert
}

fn afficher_conseil (f: &Feu) -> () {
	match f {
		Feu::Rouge => println!("Arrête-toi"),
		Feu::Orange => println!("Ralentis"),
		Feu::Vert => println!("Passe")
	}
}

fn main () {
	afficher_conseil(&Feu::Rouge);
	afficher_conseil(&Feu::Orange);
	afficher_conseil(&Feu::Vert);

}
