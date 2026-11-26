// step5_ownership.rs
// 知识点：Move/Clone/Copy/借用/引用/悬垂引用

pub fn main() {
    // ========== 1. 移动（Move）语义 ==========
    // Rust 默认是移动语义，不是复制！
    let s1 = String::from("hello");  // s1 拥有这个 String（堆内存）
    let s2 = s1;                     // s1 的所有权移动到 s2，s1 失效！

    // println!("{}", s1);  // ❌ 编译错误！s1 已失效（防止悬垂指针）
    println!("{}", s2);     // ✅ 正常

    // 为什么这样设计？防止二次释放（double free）
    // 类似 C++ 的 std::unique_ptr，但编译期强制

    // ========== 2. 克隆（Clone）- 深拷贝 ==========
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 显式克隆，堆数据也复制，s1 仍有效
    println!("s1: {}, s2: {}", s1, s2);

    // ========== 3. Copy trait - 栈上的简单类型自动复制 ==========
    // 基本类型（整数、浮点、布尔、字符）和它们的元组实现了 Copy
    let x = 5;
    let y = x;  // x 仍然有效，因为 i32 是 Copy 类型（存在栈上，复制成本低）
    println!("x: {}, y: {}", x, y);

    // 判断标准：如果类型实现了 Drop trait（需要清理资源），就不能是 Copy

    // ========== 4. 函数传参也会移动所有权 ==========
    let s = String::from("hello");
    takes_ownership(s);  // s 被移动进函数，之后失效
    // println!("{}", s);  // ❌ 报错！

    let x = 5;
    makes_copy(x);  // x 是 Copy 类型，仍然有效
    println!("x 在 main 中: {}", x);  // ✅

    // ========== 5. 返回值也会转移所有权 ==========
    let s1 = gives_ownership();  // 函数返回 String，所有权给 s1
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2 被移动，所有权给 s3

    // ========== 6. 引用（Reference）- 不获取所有权（借用）==========
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // & 表示借用，s1 仍有效
    println!("'{}' 长度是 {}，s1 仍可用: {}", s1, len, s1);

    // 借用规则（编译期检查）：
    // 1. 可以有多个不可变引用（&T）
    // 2. 只能有一个可变引用（&mut T）
    // 3. 可变引用和不可变引用不能同时存在

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} 和 {}", r1, r2);  // 多个不可变引用 OK
    // r1 和 r2 在这里之后不再使用（作用域结束）

    let r3 = &mut s;  // 现在可以创建可变引用了
    r3.push_str("!");
    // println!("{}", r1);  // ❌ 如果 r1 还在用，就不能创建 r3

    // ========== 7. 可变引用 - 同一时间只能有一个！==========
    let mut s = String::from("hello");
    change(&mut s);  // 传递可变引用
    println!("修改后: {}", s);

    // 为什么不能有多个可变引用？防止数据竞争（data race）
    // 类似 Java 的 ConcurrentModificationException，但编译期阻止

    // ========== 8. 悬垂引用（Dangling Reference）- Rust 编译期阻止 ==========
    // let reference_to_nothing = dangle();  // ❌ 编译错误！

    // ========== 9. 切片（Slice）- 另一种引用 ==========
    let s = String::from("hello world");
    let hello = &s[0..5];  // 字符串切片，&str 类型
    let world = &s[6..11];
    println!("{} {}", hello, world);

    // 切片也遵循借用规则
    let mut s = String::from("hello");
    let slice = &s[0..2];
    // s.clear();  // ❌ 报错！slice 还在借用 s
    println!("切片: {}", slice);
}

// 函数：获取所有权
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string 离开作用域，内存被释放（调用 drop）

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // 返回并转移所有权
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 返回并转移所有权
}

fn calculate_length(s: &String) -> usize {  // &String 是引用（借用）
    s.len()
} // s 离开作用域，但不会释放内存（因为不拥有）

fn change(some_string: &mut String) {  // &mut String 是可变引用
    some_string.push_str(", world");
}

// ❌ 错误示例：返回局部变量的引用
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // 返回局部变量的引用，但 s 在函数结束时被释放！
// } // s 在这里被释放，引用指向无效内存（悬垂指针）