use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("guess");
    let rand_num = rand::thread_rng().gen_range(1,101);
    println!("rand num is {}",rand_num);

    loop {
        // 可变变量
        let mut guess = String::new();

        // std::io
        io::stdin().read_line(&mut guess).expect("read line error");

        //转换类型
        // 因为rand_num是u32类型
        // shadow
        let guess: u32 = guess.trim().parse().expect("convert error");

        println!("guess num is {}", guess);

        match guess.cmp(&rand_num) {
            Ordering::Equal => {println!("equal");break;},
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("large")
        }
    }
}
