/*
Consigne (énoncé B) : Un point dans un plan, défini par une abscisse et une ordonnée (des entiers).
Tu écris la définition du type, puis dans main tu construis un point et tu l'affiches.
*/
#[allow(dead_code)]
#[derive(Debug)]
struct Point {
    abscisse: i32,
    ordonnee: i32,
}

fn main(){
    let paris = Point { abscisse: 100, ordonnee: 9000 };
    println!("{:?}", paris);}
