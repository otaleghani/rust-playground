mod vecchio;
fn main() {
    let v = vecchio![1,2,3,4];
    println!("{}", v.iter().sum::<i32>());
}
