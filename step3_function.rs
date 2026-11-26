// step3_function.rs
// 知识点：fn/参数/返回值/闭包

// ========== 1. 函数定义 ==========
// 语法：fn 函数名(参数: 类型) -> 返回类型 { 函数体 }
// 注意：Rust 用蛇形命名法（snake_case），类似 Python

/// 计算两个数的和
/// 文档注释（///），鼠标悬停会显示
fn add(a: i32, b: i32) -> i32 {
    a + b  // ⚠️ 没有分号！这是表达式（expression），会作为返回值
    // 如果写 a + b; 就变成语句（statement），返回 ()（单元类型，类似 void）
}

// 返回多个值？用元组！
fn calculate(x: i32, y: i32) -> (i32, i32) {
    (x + y, x - y)  // 返回元组
}

// 无返回值（实际返回 ()，类似 void）
fn print_hello() {
    println!("Hello");
}

// 引用参数（不获取所有权，类似传指针）
fn calculate_length(s: &String) -> usize {
    s.len()
} // s 离开作用域，但不会释放内存（因为只是借用，不拥有）

// 可变引用参数（可以修改传入的值）
fn append_world(s: &mut String) {
    s.push_str(", world!");
}

// ========== 2. 闭包（Closure）- 类似 Python 的 lambda ==========
pub fn main() {
    // 基本函数调用
    let sum = add(3, 5);
    println!("3 + 5 = {}", sum);

    let (sum, diff) = calculate(10, 3);
    println!("和: {}, 差: {}", sum, diff);

    print_hello();

    // ========== 3. 引用示例 ==========
    let s = String::from("hello");
    let len = calculate_length(&s);  // & 表示借用，s 仍有效
    println!("'{}' 长度是 {}，s 仍可用: {}", s, len, s);

    let mut s2 = String::from("hello");
    append_world(&mut s2);  // &mut 可变借用
    println!("修改后: {}", s2);

    // ========== 4. 闭包 ==========
    // 闭包是可以捕获环境变量的匿名函数
    let x = 4;
    let add_x = |n| n + x;  // 捕获 x，自动推断类型
    println!("闭包结果: {}", add_x(3));  // 7

    // 闭包作为参数（类似 Java 的函数式接口）
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!(" doubled: {:?}", doubled);  // [2, 4, 6, 8, 10]

    // 闭包语法对比：
    // |参数| 表达式          // 单行
    // |参数| { 多行代码 }    // 多行
}