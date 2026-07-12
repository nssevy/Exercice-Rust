/*

Écris une fonction plus_grand qui reçoit une slice d'entiers &[i32] et retourne le plus grand élément.
Le piège : une slice peut être vide. Ta fonction doit donc retourner un Option<i32> :

None si la slice est vide,
Some(valeur) sinon.

Dans main, teste avec au moins deux cas : une slice qui contient des valeurs, et une slice vide. Affiche le résultat des deux.
4. Résultat attendu (à l'exécution)
Avec par exemple [3, 7, 2, 9, 4] et [] :
Le plus grand : Some(9)
Le plus grand : None

*/

fn plus_grand (a: &[i32]) -> Option<i32> {
	
	let mut i: usize = 0;
	let mut grand: i32 = a[i];	
	
	if a.len() < i {
		return None
	}	

	while i < a.len() {
		if a[i] > grand {
			grand = a[i];
		}
		i += 1;
	}
	Some(grand)
}


fn main () {

	let tab_p: Vec<i32> = vec![3, 7, 2, 9, 4];
	let _tab_v: Vec<i32> = vec![];
	
	println!("{:?}", plus_grand(&tab_p).unwrap());

}
