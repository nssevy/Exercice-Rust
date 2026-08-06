/*Un moyen de paiement qui est soit des espèces (aucune information),
soit une carte bancaire (avec un numéro et le nom du titulaire), soit un chèque
(avec un numéro de chèque).*/
#![allow(dead_code)]
#[derive(Debug)]
enum TypeMoyenPaiement {
    Espece,
    Carte(i32, String),
    Cheque(i32)
}
#[derive(Debug)]
struct MoyenPaiement{
    moyen: TypeMoyenPaiement
}
impl MoyenPaiement {
    fn new(a: TypeMoyenPaiement) -> MoyenPaiement{
        MoyenPaiement {moyen: a}
    }
}
fn main() {
    let espece =  MoyenPaiement::new(TypeMoyenPaiement::Espece);
    let cb =  MoyenPaiement::new(TypeMoyenPaiement::Carte(90, String::from("Sevy")));
    let cheque =  MoyenPaiement::new(TypeMoyenPaiement::Cheque(90));
    println!("{:?}", espece);
    println!("{:?}", cb);
    println!("{:?}", cheque)
}