/*

Écris une fonction qui prend une slice de i32 et détermine si ses éléments sont triés dans l'ordre croissant (chaque élément inférieur ou égal au suivant). Dans main, appelle-la sur au moins deux tableaux : un trié, un non trié. Affiche pour chacun le tableau et un message qui dit s'il est trié ou non.
4. Résultat attendu (exemple)

[1, 3, 5, 8] est trié dans l'ordre croissant
[4, 2, 7, 1] n'est pas trié dans l'ordre croissant

*/

fn tab_croissant(a: &[i32]) -> u32 {
	
	let mut i: usize = 0;
	let mut croissant: u32 = 0;
	
	while i < a.len()-1 {
	
		if a[i] < a[i+1] {
			croissant = 1;
			i+=1
		} else {
			croissant = 0
		}
	
	}
	croissant
	
}

fn main () {

	let tab_trier: Vec<i32> = vec![23, 56, 65, 91];
	
	let tab_pas_trier: Vec<i32> = vec![2, 23, 2, 34, 56];
	
	println!("{:?}", tab_croissant(&tab_trier));	

}

