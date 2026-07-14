/*

Écris une fonction qui prend une slice de i32 et détermine si ses éléments sont triés dans l'ordre croissant (chaque élément inférieur ou égal au suivant). Dans main, appelle-la sur au moins deux tableaux : un trié, un non trié. Affiche pour chacun le tableau et un message qui dit s'il est trié ou non.
4. Résultat attendu (exemple)

[1, 3, 5, 8] est trié dans l'ordre croissant
[4, 2, 7, 1] n'est pas trié dans l'ordre croissant

*/

fn croissant (c: &[i32]) -> i32 {

	let mut i: usize = 0;
	let mut valeur: i32 = 0;

	while i < c.len()-1 {

		if c[i] <= c[i+1]{
			valeur = 1;
			i+=1;
		}

        }

	valeur
}

fn decroissant (d: &[i32]) -> i32 {

        let mut i: usize = 0;
        let mut valeur: i32 = 0;

        while i < d.len()-1 {

                if d[i] >= d[i+1]{
                        valeur = 2;
                        i+=1;
                }

        }

        valeur
}

fn verificateur_de_tableau(a: &[i32]) -> i32 {
	/*
	Dans la fonction les valeurs de croissant sont défini par : 
	
	0 = tableau pas trier
	1 = Tableau dans l'ordre croissant
	2 = Tableau décroissant
	3 = Tableau vide ou pas assez d'éléments pour comparer (au mininmum 2)
	*/
	
	let i: usize = 0;
	let mut valeur_retour: i32 = 0;

	if a.len() < 2 {
		valeur_retour  = 3;
		return valeur_retour;
	}
	
	if a[i] < a[i+1] {
		croissant(&a)
	} else if a[i] > a [i+1]{
		decroissant(&a)
	} else {
		valeur_retour = 0;
		return valeur_retour;
	}
	
}

fn main () {

	let tab_trier: Vec<i32> = vec![23, 56, 65, 91];
	
	let tab_pas_trier: Vec<i32> = vec![2, 23, 2, 34, 56];

	let _tab_doublon: Vec<i32> = vec![23, 23, 34, 56];

	let aleatoire: Vec<i32> = vec![];

	let decroissant: Vec<i32> = vec![100, 50, 25, 1];
	
	println!("{:?}", verificateur_de_tableau(&tab_trier));	
 
	println!("{:?}", verificateur_de_tableau(&decroissant));
	
	println!("{:?}", verificateur_de_tableau(&aleatoire));

	println!("{:?}", verificateur_de_tableau(&tab_pas_trier));
}

