fn main() {
    let v1 = vec![10, 20, 30];

    let mut v2 = v1;
    v2.truncate(2); // truncate to two elements

    println!("{:?}", v2); // vector is now [10, 20]
}
