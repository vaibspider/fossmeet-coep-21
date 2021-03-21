fn find(v: &Vec<i32>, elem: i32) -> usize {
    let mut index = 0;
    while index < v.len() {
        if v[index] == elem {
            break;
        }
        index += 1;
    }
    index
}
fn main() {
    let mut v = vec![10, 20, 30, 40, 50, 60];
    let index = find(&v, 40);
    v.clear();
    println!("{}", v[index]);
}
