fn main() {
    let v2: &Vec<i32>; //alive till end-of-main
    {
        let v1 = vec![1, 2, 3]; //alive till end-of-block
        v2 = &v1;
    }
    println!("{:?}", v2);
}
