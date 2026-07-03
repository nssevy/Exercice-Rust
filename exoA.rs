/*
Déclare une variable nombre de type i32 valant 7. Avec un if / else, affiche "pair" si le nombre est pair, sinon "impair".
Indice sur le vocabulaire : pour savoir si un nombre est pair, on regarde le reste de sa division par 2. En Rust, le reste s'obtient avec l'opérateur % (modulo). Un nombre est pair si nombre % 2 == 0.
Résultat attendu : impair

*/

fn main () {

let nombre: i32 = 7;
if nombre % 2 == 0 {println!("pair")} else { println!("impair") }; 

}
