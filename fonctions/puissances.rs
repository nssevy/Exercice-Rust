/*

puissance(base, exposant) : calcule base élevé à la puissance exposant, avec une boucle. base est un i32, exposant est un u32. Elle retourne un i32.

plus_grand(nombres) : reçoit une slice d'entiers et retourne le plus grand élément. Si la slice est vide, il n'y a pas de plus grand élément : le type de retour doit refléter ça.

afficher_bilan(base, resultats) : reçoit la base et une slice des résultats, affiche chaque résultat sur une ligne avec son exposant, puis affiche le plus grand en utilisant plus_grand. Cette fonction ne retourne rien.

Dans main : construis un Vec<i32> contenant puissance(2, e) pour e allant de 0 à 5 inclus, puis appelle afficher_bilan.
Contrainte : aucun return dans tout le fichier. Uniquement des tail expressions. C'est le cœur de l'exercice.

Résultat attendu à l'exécution
2^0 = 1
2^1 = 2
2^2 = 4
2^3 = 8
2^4 = 16
2^5 = 32
Le plus grand est 32

*/

fn plus_grand(a: &[i32]) -> Option<i32> {
	let mut i: usize = 0;

	if a.len() == 0 {
		return None
	}

	let mut grand: i32 = a[i];

	while i < a.len() {

		if a[i] > grand {
			grand = a[i];
		}

		i+=1;
	}

	Some(grand)
}

fn _puissance(base: i32, exposant: u32) -> i32 {

	let mut i = 0;
	let mut total = 1; 
	
	if exposant == 0 {
		 return 1;
	} 

	while i  < exposant {
		total *= base;
		i += 1;
	}

	total
}

fn main () {
	let v: Vec<i32> = vec![32, 54, 0999, 23];	
	println!("{:?}", plus_grand(&v).unwrap());

	let vide: Vec<i32> = vec![];
	println!("{:?}", plus_grand(&vide));

	let v_negatif: Vec<i32> = vec![-91, -9, -8, -3];
	println!("{:?}", plus_grand(&v_negatif).unwrap());

	/* println!("{}", puissance(4, 3)); */
}
