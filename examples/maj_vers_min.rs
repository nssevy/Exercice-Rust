/*
pars de 'P', obtiens et affiche sa version minuscule par arithmétique ASCII (sans méthode toute faite).

Résultat attendu : p

Indice : lis le sens du décalage dans le schéma. Convertis, calcule, reconvertis. Attention : c'est peut-être l'inverse de ton intuition.
*/

fn main(){
    const CONVERSION: u8 = 32;

    let p_majuscule: u8 = 'P' as u8;
    let p_minuscule: u8 = p_majuscule + CONVERSION;

    let p_minuscule_ascii = p_minuscule as char;
    println!("{p_minuscule_ascii}");
}
