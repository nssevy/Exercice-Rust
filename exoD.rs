/*

Écris une fonction est_majeur qui prend un i32 (un âge) et renvoie un bool : true si l'âge est supérieur ou égal à 18, sinon false. Dans main, appelle-la avec 15 et affiche le résultat.
Indice : le type de retour est -> bool, et le corps peut être directement l'expression de comparaison age >= 18 (une comparaison renvoie déjà un booléen).
Résultat attendu : false

*/

fn est_majeur (age: i32) -> bool {

	age >= 18

}

fn main () {

println!("{}",est_majeur(15));

}
