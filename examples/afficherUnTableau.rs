fn afficher_un_tableau(a: &[i32]) {

	let mut i: usize = 0;

	while i < a.len() {
		let c  = a[i];
		println!("{c}");
		i += 1;	
	}

}


fn main () {

	let v: Vec<i32> = vec![2, 4, 5, 10];
	afficher_un_tableau(&v);
}
