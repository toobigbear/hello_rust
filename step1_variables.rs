// step1_variables.rs
// 知识点：let/mut/基本类型/数组/元组/字符串

pub fn main() {
    // ========== 1. 变量默认不可变（类似 Java final）==========
    let name = "RustRover";
    println!("Hello, {}!", name);
    // name = "Tom"; // ❌ 编译错误！默认不可变

    // ========== 2. mut 可变变量 ==========
    let mut count: i32 = 0;  // i32 是 32位整数，默认类型
    count += 1;              // 有了 mut 才能修改
    println!("Count = {}", count);

    // ========== 3. 基本类型 ==========
    let int_num: i64 = 100_000;      // 下划线分隔数字，提高可读性
    let float_num: f64 = 3.14;       // f64 是默认浮点类型（类似 Java double）
    let is_rust_fun: bool = true;    // 布尔
    let letter: char = 'R';          // char 是 Unicode，占 4 字节（不是 1 字节！）

    println!("整数: {}, 浮点: {}, 布尔: {}, 字符: {}", int_num, float_num, is_rust_fun, letter);

    // ========== 4. 类型转换（必须显式！）==========
    let x = 42_i32;           // 后缀指定类型
    let y = x as f64;         // as 关键字转换（类似 C 的强制转换）
    let z: i32 = "123".parse().unwrap(); // 字符串转数字，可能失败所以返回 Result
    println!("转换: {} -> {}, 字符串解析: {}", x, y, z);

    // ========== 5. 数组（固定长度！不是 ArrayList）==========
    // 特点：长度固定、类型相同、栈分配（快！）
    let arr: [i32; 5] = [1, 2, 3, 4, 5];  // [类型; 长度]
    println!("数组第一个: {}, 长度: {}", arr[0], arr.len());
    // arr[10]; // ❌ 运行时 panic！越界检查（类似 Java 的 ArrayIndexOutOfBounds）

    // 初始化技巧：[初始值; 长度]
    let zeros = [0; 10]; // 10 个 0
    println!("zeros[5] = {}", zeros[5]);

    // ========== 6. 元组（Tuple）- 可以装不同类型 ==========
    let tup: (i32, f64, &str) = (500, 6.4, "元组");
    println!("元组: ({}, {}, {})", tup.0, tup.1, tup.2);

    // 解构（类似 Python 的 a, b = 1, 2）
    let (a, b, c) = tup;
    println!("解构: a={}, b={}, c={}", a, b, c);

    // ========== 7. 字符串（重点！Rust 有两种字符串）==========

    // &str：字符串切片（不可变，类似 Java 的 String 常量池）
    let s1: &str = "hello";

    // String：可变的、堆分配的字符串（类似 Java 的 new String()）
    let mut s2 = String::from("hello");
    s2.push_str(", world!");  // String 可以追加
    println!("String: {}", s2);

    // &str 和 String 的关系：
    // String 可以变成 &str（&s2），但 &str 不能直接变成 String（需要 .to_string() 或 String::from()）

    // ========== 8. Shadowing（变量隐藏）- Rust 特色 ==========
    let x = 5;
    let x = x + 1;           // 不是赋值，是创建了新变量（可以改类型！）
    let x = "现在我是字符串"; // 甚至可以变成字符串！
    println!("Shadowing 后: {}", x);
    // 与 mut 的区别：mut 是修改原变量，shadowing 是创建新变量（旧变量被隐藏）
}