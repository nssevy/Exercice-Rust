/*Crée un Vec<i32> contenant les valeurs 7, 2, 9, 4, 6.
Écris une fonction séparée nommée maximum qui prend en paramètre une slice &[i32] et renvoie le plus grand élément (un i32).
Dans main, appelle cette fonction en lui passant une slice de tout le Vec, puis affiche le résultat.

Le résultat attendu à l'exécution :
9*/

fn maximum (s: &[i32]) -> &i32{
	s.iter().max().unwrap()
}

fn main (){

let v = vec![7, 2, 9, 4, 6];

println!("{:?}", maximum(&v));
}
