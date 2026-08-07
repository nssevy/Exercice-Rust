/*
: Écris fn mention(note: u8) -> char qui renvoie une lettre selon une note sur 20 :

>= 16 -> 'A'
>= 14 -> 'B'
>= 12 -> 'C'
>= 10 -> 'D'
sinon -> 'F'

Cette fois, pas de variable intermédiaire : tout le corps de la fonction doit être un seul
if / else if / else utilisé comme tail expression (la valeur de retour). Aucun return,
aucun ; sur la valeur des branches.

Test dans main :

println!("{}", mention(15));
println!("{}", mention(9));
println!("{}", mention(18));

Résultat attendu :

B
F
A
 */
fn mention(note: u8) -> char {
    if note >= 16 {'A'}
        else if note >= 14 {'B'}
            else if note >= 12 {'C'}
                else if note >= 10 {'D'}
                    else {'F'}
}
fn main() {
    println!("{}", mention(15));
    println!("{}", mention(9));
    println!("{}", mention(18));
}