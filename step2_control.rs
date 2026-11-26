// step2_control.rs
// 知识点：if/loop/while/for/match

pub fn main() {
    // ========== 1. if 表达式（注意：条件必须是 bool！）==========
    let number = 7;

    // Rust 没有 truthy/falsy，必须显式布尔
    if number < 5 {
        println!("小于 5");
    } else if number == 5 {
        println!("等于 5");
    } else {
        println!("大于 5");
    }

    // if 是表达式，可以返回值（类似三元运算符，但更强大）
    let condition = true;
    let x = if condition { 5 } else { 6 };
    println!("if 表达式结果: {}", x);

    // ========== 2. loop - 无限循环（类似 while(true)）==========
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // break 可以带返回值（类似 return）
        }
    };
    println!("loop 结果: {}", result);  // 20

    // ========== 3. while - 条件循环 ==========
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("发射！");

    // ========== 4. for - 最常用、最安全（类似 Python 的 for...in）==========
    let arr = [10, 20, 30, 40, 50];

    // 遍历元素（推荐）
    for element in arr.iter() {
        println!("值: {}", element);
    }

    // 遍历索引和值（类似 Python 的 enumerate）
    for (i, v) in arr.iter().enumerate() {
        println!("索引 {} 值 {}", i, v);
    }

    // Range 范围：1..4 表示 1,2,3（不包含 4）
    for i in 1..4 {
        println!("Range: {}", i);
    }
    // 1..=4 表示 1,2,3,4（包含 4）
    for i in 1..=4 {
        println!("Range=: {}", i);
    }

    // ========== 5. match - 模式匹配（类似 switch，但强大 100 倍）==========
    let x = 5;
    match x {
        1 => println!("一"),
        2 | 3 => println!("二或三"),      // 或
        4..=6 => println!("四到六"),        // 范围
        _ => println!("其他"),             // 通配符（类似 default，必须处理所有情况！）
    }

    // match 返回值
    let y = match x {
        1 => "一",
        2 => "二",
        _ => "其他",
    };
    println!("match 返回: {}", y);

    // ========== 6. if let - 简化 match（只关心一种模式时）==========
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("是三！");
    }

    // 相当于：
    // match some_value {
    //     Some(3) => println!("是三！"),
    //     _ => (),  // 忽略其他情况
    // }
}