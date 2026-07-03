/*

Le contexte : on gère des notes d'élève sur 20.

Dans main, crée un Vec<i32> nommé notes contenant 8, 14, 19, 11.
Écris une fonction séparée nommée somme qui prend une slice &[i32] et renvoie la somme de ses éléments (un i32). Tu l'as déjà écrite plusieurs fois, c'est le même schéma.
Écris une deuxième fonction séparée nommée mention qui prend un i32 (une note) et renvoie un &str, en utilisant un match avec des ranges :

0 à 9 inclus renvoie "insuffisant"
10 à 13 inclus renvoie "passable"
14 à 17 inclus renvoie "bien"
18 à 20 inclus renvoie "excellent"
tout autre cas renvoie "note invalide"


Dans main, fais deux choses :

calcule et affiche la somme des trois premières notes (donc une slice [..3])
affiche la mention de la dernière note, celle d'index 3 (la valeur 11)



Le résultat attendu à l'exécution :
41
passable

*/

fn somme (a: &[i32]) -> &i32 {
	a.iter().sum()
}

fn main () {
	let v = vec![8, 14, 19, 11];

	println!("{:?}",sum(&v));
}




