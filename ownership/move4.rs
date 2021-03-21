fn fun1(v: Vec<u32>) {
    println!("{:?}", v);
}

fn main() {
    let v1 = vec![1, 2, 3];

    fun1(v1);

    println!("{}", v1[0]);
}
