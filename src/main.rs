fn main() {
    // 1. 变量默认不可变
    let name = "RustRover";
    println!("Hello, {}!", name);

    // 2. 可变变量
    let mut count = 0;
    count += 1;
    println!("Count = {}", count);

    // 3. 调用函数
    let sum = add(3, 5);
    println!("3 + 5 = {}", sum);

    // 4. 循环
    for i in 1..=3 {
        println!("第 {} 次循环", i);
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // 最后一行是返回值，不用写 return
}