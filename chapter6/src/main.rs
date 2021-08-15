fn main() {
    let v = struct_declare("name".to_string(), 11);
    println!("{} {}", v.name, v.age);
    println!("{:#?} ", v);
    v.say();

    println!("------------------------------");

    let c = color_declare();
    println!("{} {}", c.0, c.1);
    println!("{}", color_sum(c));

    let m = day::Monday;
    println!("{:?}", m);

    let so = Some("123");
    println!("{:?}", so);
}

#[derive(Debug)]
struct User {
    name: String,
    age: i16,
}

// Rust 并没有一个与 -> 等效的运算符；相反，Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。方法调用是 Rust 中少数几个拥有这种行为的地方。
//
// 他是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。也就是说，这些代码是等价的：
// p1.distance(&p2);
// (&p1).distance(&p2);

impl User {
    fn say(&self) {
        println!("name is {}", self.name)
    }
}

fn struct_declare(name: String, age: i16) -> User {
    User {
        name: name,
        age: age,
    }
}

struct Color(i16, i16);

fn color_declare() -> Color {
    Color(1, 2)
}

fn color_sum(co: Color) -> i16 {
    co.0 + co.1
}

#[derive(Debug)]
enum day {
    Monday,
    Tuesday,
}
