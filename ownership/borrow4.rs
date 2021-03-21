fn change(v: &mut Vec<i32>) {
    v[0] = 10;
}

fn main() {
    let mut v = vec![1,2,3];

    change(&mut v);

    println!("{}", v[0]); // prints 10
}
