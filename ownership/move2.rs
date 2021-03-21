fn main() {
    let v1 = vec![1, 2, 3];

    let mut v2 = v1;
    v2.truncate(2); // truncate to two elements

    println!("{:?}", v1);
}
