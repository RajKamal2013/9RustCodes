fn main() {
    println!("Hello, world!");
    let mut res:u32 = highest(2, 34, 9);
    println!("Highest of 3, 34, and 9 is {}", res);
    res = basic_loops(23, 20);
}

fn basic_loops(a: u32, b:u32)->u32 {
    let mut n:u32 = 0;
    loop {
        println!("Hello : {}", n);
        n = n + 1;
        if n > 10 {
            break;
        }
    }
    return n;
}


fn highest(a:u32, b:i32, c:u32)->u32 {
    let mut res:u32 = a;

    if b as u32  > res {
        res = b as u32;
    }

    if c > res {
        res = c;
    }

    return res;
}
