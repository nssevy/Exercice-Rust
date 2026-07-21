/*
stocke le caractère 'A' dans une variable, affiche son code numérique.
Résultat attendu : 65
Indice : char ne calcule pas, mais se convertit avec as. Choisis entre u8 et u32 et demande-toi pourquoi.
*/

fn main(){
    /*
    let a: char = 'A';
    let octet: u8 = a as u8;
    */
    let octet: u8 = 'A' as u8; // Les deux fonctionnes
    println!("{octet}");
}
