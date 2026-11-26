// step4_struct.rs
// 知识点：struct/impl/方法/关联函数

// ========== 1. 定义结构体（类似 Java 的 Class）==========
// 注意：字段可以是不同类型，但必须指定类型
struct User {
    username: String,   // 这里用 String 而不是 &str（因为 String 拥有数据）
    email: String,
    sign_in_count: u64, // u64 是无符号 64 位整数
    active: bool,
}

// 元组结构体（没有字段名，适合简单的包装）
struct Color(i32, i32, i32);  // RGB
struct Point(i32, i32, i32);  // 坐标
// 注意：Color 和 Point 虽然结构相同，但是不同类型！

// ========== 2. 实现块（impl）- 给结构体添加方法 ==========
impl User {
    // 关联函数（类似 Java 的静态方法/构造器），用 :: 调用
    // 没有 self 参数
    fn new(username: String, email: String) -> User {
        User {
            username,      // 字段初始化简写（类似 JS 的对象简写）
            email,
            sign_in_count: 1,
            active: true,
        }
    }

    // 方法（第一个参数是 &self，类似 this）
    // &self 是不可变借用（只读），不获取所有权
    fn display(&self) {
        println!("用户: {}, 邮箱: {}", self.username, self.email);
    }

    // &mut self 是可变借用（可以修改字段）
    fn deactivate(&mut self) {
        self.active = false;
    }

    // 带参数的方法
    fn is_email_match(&self, domain: &str) -> bool {
        self.email.ends_with(domain)
    }

    // 消耗 self（获取所有权），类似 Java 的某个方法调用后对象失效
    fn destroy(self) {
        println!("销毁用户: {}", self.username);
        // self 离开作用域，内存被释放
    }
}

pub fn main() {
    // ========== 3. 创建实例 ==========
    // 注意：必须 mut 才能修改字段
    let mut user1 = User::new(
        String::from("alice"),
        String::from("alice@example.com"),
    );

    // 访问字段（用 . 号）
    println!("用户名: {}", user1.username);

    // 调用方法
    user1.display();

    // 修改字段（需要 mut）
    user1.email = String::from("new@example.com");
    user1.deactivate();

    // ========== 4. 结构体更新语法（类似 JS 的 ...spread）==========
    let user2 = User {
        email: String::from("bob@example.com"),
        ..user1  // 其余字段从 user1 复制/移动
    };
    // ⚠️ 注意：String 不是 Copy 类型，所以 user1.username 被移动到了 user2
    // println!("{}", user1.username); // ❌ 报错！user1 已部分失效

    // ========== 5. 元组结构体使用 ==========
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // black 和 origin 类型不同！即使都是三个 i32
    println!("Color: ({}, {}, {})", black.0, black.1, black.2);

    // ========== 6. 方法调用示例 ==========
    let mut user3 = User::new(String::from("charlie"), String::from("charlie@test.com"));
    if user3.is_email_match("test.com") {
        println!("是 test.com 邮箱");
    }

    user3.destroy();  // 消耗所有权
    // user3.display(); // ❌ 报错！user3 已被销毁
}