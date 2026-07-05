/*

Dans main, en utilisant une boucle while, affiche les nombres de 1 à 5 inclus, un par ligne.
Le résultat attendu à l'exécution :
1
2
3
4
5

*/

fn main() {
    let mut a: i32 = 1;
    while a <= 5 {
        println!("{}", a);
        a += 1;
    }
}
