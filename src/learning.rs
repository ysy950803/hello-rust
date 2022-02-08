pub fn main() {
    // ch_0301();
    // ch_0801();
    ch_0802();
}

fn ch_0301() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces)
}

fn ch_0801() {
    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    let first = v[0];
    v.push(6);
    println!("{}", first);
}

fn ch_0802() {
    let s = "नमस्ते";
    for c in s.chars() {
        println!("{}", c);
    }
    for b in s.bytes() {
        println!("{}", b);
    }
    let c = &s[0..3];
    println!("{}", c);
}
