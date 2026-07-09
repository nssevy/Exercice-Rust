fn addition_tab(a: &[i32]) -> i32 {
    let mut i: usize = 0;
    let mut stock: i32 = 0;

    while i < a.len() {
 
        stock += a[i];

        i += 1;
    }
    return stock
}

fn main() {
    let v: Vec<i32> = vec![10, 20, 30];

    let total = addition_tab(&v);

    println!("{total}");
}
