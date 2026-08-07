/*
Écris une fonction fn ajuster(x: u8) -> u8. À l'intérieur, calcule un résultat intermédiaire dans
un bloc-expression assigné à une variable : double x, puis si le double dépasse 10, retranche 3,
sinon garde-le tel quel. La fonction doit renvoyer ce résultat sans écrire return et sans ; sur
sa dernière ligne. Dans main, appelle-la sur deux valeurs et affiche les résultats.
 */
fn ajuster(x: u8) -> u8 {
    let double: u8 = {
        let d = x + x;
        if d > 10 {d - 3} else {d}
    };
    double
}
fn main() {
    let a: u8 = ajuster(4);
    let b: u8 = ajuster(7);
    dbg!(a);
    dbg!(b);
}