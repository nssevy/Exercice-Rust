/*
Une forme géométrique qui est soit un cercle (défini par son rayon), soit un rectangle
(largeur et hauteur), soit un simple point (aucune dimension) et on veut pouvoir calculer son aire.
*/

#[allow(dead_code)]
enum Forme {
    Cercle(f64),
    Rectangle(f64, f64),
    Point,
}

impl Forme {
    fn calculer_air(&self) -> Option<f64> {
        match &self {
            Forme::Cercle(r) => Some(std::f64::consts::PI * r * r),
            Forme::Rectangle(l, h) => Some(l * h),
            Forme::Point => None
        }
    }
}

fn main() {
    let rectangle = Forme::Rectangle(2.0, 2.0).calculer_air();
    println!("{:?}", rectangle);
    let oke = 90;
}
