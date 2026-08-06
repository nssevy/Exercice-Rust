/*
Consigne : pars de l'entier 66 (typé u8), affiche le caractère correspondant.

Résultat attendu : B

Indice : tout u8 est un char valide, donc la conversion ne peut pas échouer ici.
*/

fn main(){
    let m_b: char = 66 as char;
    println!("{m_b}"); // m_b -> majuscule b
}
