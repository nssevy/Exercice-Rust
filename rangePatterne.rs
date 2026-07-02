/*
Déclare une variable note de type i32 avec la valeur 14.
Elle représente une note sur 20.
En utilisant un match qui renvoie une valeur, affecte à une
variable mention la mention correspondante :

une note de 0 à 9 inclus renvoie "insuffisant" (utilise un range)
une note de 10 à 11 inclus renvoie "passable" (utilise un range)
une note de 12 à 15 inclus renvoie "assez bien" (utilise un range)
les notes 16, 18 ou 20 renvoient "excellent" (utilise un
or-pattern avec |)
tout le reste renvoie "note inhabituelle"


Affiche mention.

Le résultat attendu à l'exécution (14 est dans l'intervalle
12 à 15) :
assez bien
*/

fn main (){
    let note: i32 = 14;
    //elle represente une note sur 20

    let resultat = match note{
      0..=9 => {"insuffisant"}
      10..=11 => {"passable"}
      12..=15 => {"assez bien"}
        16 | 18 | 20  => {"excellent"}
        _=> {"note inhabituelle"}
    };
    
    println!("{}",resultat);
}