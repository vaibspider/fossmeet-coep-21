fn main() {
    let v1 = vec![1,2,3];
    let v2: &Vec<i32>;

    v2 = &v1;
    println!("{:?}", v2);
}
