fn main() {
    var_mut();

    var_shadown();

    var_declare();

    var_tuple();

    var_array();
}

fn var_shadown() {
    println!("shadow test");
    let y = 2;
    println!("shadow variable y {}", y);
    let y = 3;
    println!("shadow variable y {}", y);
    p()
}

fn var_mut() {
    //
    // let x = 100;
    // x = 99;
    println!("mut test");

    // 可修改
    let mut x = 10;
    println!("mux x {}", x);
    x = 100;
    println!("mux change x {}", x);
    p()
}

fn var_declare() {
    println!("delcare test");
    // error
    // let num: i8 = 128;
    // --release下不会panic 会取模
    let num: i8 = 127;
    println!("num  Decimal {}", num);
    // 8进制
    let num = 0o55;
    println!("num  Octal {}", num);

    // 16进制
    let num: i32 = 0xff;
    println!("num  Hex {}", num);

    // 2进制
    let num: i32 = 0b11111111;
    println!("num  binary {}", num);

    // 2进制
    let num: u8 = b'A';
    println!("num  binary {}", num);

    let t = true;
    println!("t   {}", t);
    let f: bool = false;
    println!("t   {}", f);
    // 4个字节
    let c = 'a';
    println!("c   {}", c);
    let str = "abc";
    println!("str   {}", str);

    p()
}

fn var_tuple() {
    println!("tuple test");
    let tup: (char, i32, bool) = ('a', 100, false);
    println!("tup.char   {}", tup.0);
    println!("tup.i32   {}", tup.1);
    println!("tup.bool   {}", tup.2);

    p()
}

fn var_array() {
    // 索引越界 只能进行简单检查
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    for  a in arr {
        println!("arr   {}", a);
    }
    p();
}

fn p() {
    println!("------------------------------------------------");
}
