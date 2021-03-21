fn change(v: &Vec<i32>) {
    v[0] = 10;
}

fn main() {
    let mut v = vec![1, 2, 3];

    change(&v);
}
