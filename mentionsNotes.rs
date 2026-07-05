/*

Écris une fonction séparée nommée afficher_mentions qui prend une slice &[i32] (des notes sur 20) et ne renvoie rien. À l'intérieur, parcours la slice avec une boucle while et, pour chaque note, utilise un match avec des ranges pour afficher la mention correspondante, sous la forme "Note X : mention". Les mentions sont :

0 à 9 inclus : "insuffisant"
10 à 13 inclus : "passable"
14 à 17 inclus : "bien"
18 à 20 inclus : "excellent"
tout autre cas : "note invalide"

Dans main, crée un Vec<i32> contenant 20, 8, 13, 15, 25, et appelle la fonction en lui passant une slice de tout le Vec.
Le résultat attendu à l'exécution :
Note 20 : excellent
Note 8 : insuffisant
Note 13 : passable
Note 15 : bien
Note 25 : note invalide

*/

fn afficher_mentions (a: &[i32]) -> () {
	
	let mut i: usize = 0;
		
	while i < a.len(){
		
		match a[i]  {
			v @ 0..=9 => println!("Note {} : insuffisant", v),
			v @ 10..=13 => println!("Note {} : passable", v),
			v @ 14..=17 => println!("Note {} : bien", v),
			v @ 18..=20 => println!("Note {} : excellent", v),
			_ => println!{"note invalide"},
		}
		i+=1;
	} 
}

fn main () {

	let notes: Vec<i32> = vec![20, 8, 13, 15, 25];
	
	afficher_mentions(&notes);

}
