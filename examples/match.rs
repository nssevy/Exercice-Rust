/*
Déclare une variable feu de type &str (une chaîne) contenant la
valeur "orange". Elle représente la couleur d'un feu de
circulation.
En utilisant un match qui renvoie une valeur, affecte à une
variable action la consigne correspondante :

"vert" renvoie "passe"
"orange" renvoie "ralentis"
"rouge" renvoie "arrête-toi"
tout autre cas renvoie "feu inconnu"


Affiche action.

Le résultat attendu à l'exécution :
ralentis
*/

fn main (){
    let feu: &str = "orange";
    
    let action = match feu{
        "vert" => {"passe"}
        "orange" => {"ralentis"}
        "rouge" => {"arrête-toi"}
        _       => {"feu inconnu"}
    };
    
    println!("{}", action);
}