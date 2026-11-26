// main.rs
// 通过 mod 引入其他文件，想运行哪个就取消注释哪行
#![allow(unused_variables)]  // 告诉编译器：我知道有些变量没用，别提醒了
//mod step1_variables;  // 第1步：变量与数据类型
//mod step2_control;    // 第2步：控制流
//mod step3_function;   // 第3步：函数与闭包
//mod step4_struct;     // 第4步：结构体
mod step5_ownership;     // 第5步：所有权（当前运行这个）
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


    println!("******************************");
    // 如果运行的是 step5，需要在这里调用它的 main 函数
    // 因为 mod 引入后，原来的 main 变成了 step5_ownership::main()
    //step1_variables::main();
    //step2_control::main();


    // 其他步骤同理：
    // step1_variables::main();
    // step2_control::main();
    // step3_function::main();
    // step4_struct::main();
    step5_ownership::main();
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // 最后一行是返回值，不用写 return
}