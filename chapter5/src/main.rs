fn main() {
    println!("{}", de());
    // empty()
    guess();
    blockDeclare();
    println!("{}", re());
    println!("{}", loopDeclare());
    it();
    let reStr = str();
    // move owner
   // let shadowStr =reStr;
   //  say(reStr);
    println!("{} size is {}", reStr,rref(&reStr));

    let a = String::from("cc aa");
    println!("bool={} ", strIsContainBlack(&a));


    sliceStr();
}

fn de() -> i32 {
    let a = 10;
    a
}

fn emtpy() {
    {
        ();
    }
    println!("Hello, world!");
}

fn guess() {
    let gues: u32 = "42".parse().expect("error");
    println!("{}", gues)
}

fn blockDeclare() {
    let x = {
        let y = 10;
        y + 10
    };
    println!("{}", x)
}

fn re() -> i32 {
    5
}

fn loopDeclare() -> i32 {
    let mut counter = 0;
    let re = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    re
}

fn it() {
    let a = [1, 2, 3, 4, 5];
    for i in a.iter() {
        println!("{}", i)
    }

    for i in (1..5).rev() {
        println!("{}", i)
    }
}

fn str() -> String {
    let mut str = String::from("heee");
    str.push_str("abc");
    str
}


fn say(s :String){
    println!("{}",s)
}

fn rref(str : &String) -> usize {
    str.len()
}

fn strIsContainBlack(s : &String) -> bool{
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' '{
            return true;
        }
    }
    false
}

fn sliceStr(){
    let str = String::from("abc");
    let first = &str[0..1];
    println!("{}",first)

}