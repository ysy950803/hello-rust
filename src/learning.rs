pub fn main() {
    ch_0801();
}

fn ch_0301() {}

fn ch_0801() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("{}", first);
}
