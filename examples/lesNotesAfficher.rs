/* 

Afficher les notes avec comme début de phrase : la note, une, deux points et la note affichée. 

*/

fn note_afficher (a: &[i32]) {
	
	let mut i: usize = 0;
	
	while i < a.len() {
		println!("La note est de : {}.", a[i]);	
		i += 1;	
	}

}

fn main () {

	let v: Vec<i32> = vec![20, 8, 3, 10, 19];
	
	note_afficher(&v); 

}
