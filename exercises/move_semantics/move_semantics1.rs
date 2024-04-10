// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut ve = vec.clone();

    ve.push(22);
    ve.push(44);
    ve.push(66);

    ve.to_vec()
}
