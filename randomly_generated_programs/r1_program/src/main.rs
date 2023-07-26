/*
 * TODO https://summer23.net9.org/backend/rust/#_73
*/

/**
 * judge if the `year` is gap year
*/
fn is_gap_year(year:i32) -> bool
{
    if year%4 == 0 && year%100 != 0 {
        return true;
    }
    else if year%400 == 0 {
        return true;
    }
    else {
        return false;
    }
}

/**
* print the info(is or not a gap year) about the `year`
*/
fn print_is_gap_year(year:i32)
{
    if is_gap_year(year) {
        println!("Year {} is a gap year", year);   
    }
    else {
        println!("Year {} is NOT a gap year", year);   
    }
}

fn f(x:String)
{
    println!("{}", x);
}

fn f_ref(x:&String)
{
    println!("{}", x);
}




fn main() {
    println!("Hello, world!");

    // 函数调用
    {
        let year:i32 = 1900;
        let year2:i32 = 2000;
        let year3:i32 = 1992;
        let year4:i64 = 1993;

        print_is_gap_year(year);
        print_is_gap_year(year2);
        print_is_gap_year(year3);

        // 参数类型转化的两种方法
        print_is_gap_year(year4.try_into().unwrap());
        print_is_gap_year(year4 as i32);
    }

    // match 语句
    {
        let x = 3;
        let y = match x {
            1 => 101,
            2 => 202,
            3 => 10086,
            _ => 99999
        };

        println!("y = {}", y);
    }

    // for 循环
    {
        for z in 0..10
        {
            println!("{}^2 = {}", z, z*z);
        }

        for z in 0..=10
        {
            println!("{}^2 = {}", z, z*z);
        }
    }

    // loop 循环
    {
        let mut x = 0;
        let y = loop {
            x += 1;
            if x*x > 100 {
                break x;    // x 作为返回值
            }
        };

        x = 0;
        let z = loop {
            x += 1;
            if x*x > 100 {
                break;      // 无指定返回值 返回unit
            }
        };

        println!("y = {}", y);
        println!("z = {:#?}", z);   // unit(以及其他的一些特殊类型)的打印方式
    }

    // 数组的定义与数组元素的索引
    {
        let arr1 = [1,2,3,4,5];
        let arr2 = [0;5];
        println!();
        for arr1_element in arr1
        {
            println!("{}", arr1_element);
        }
        println!();
        for arr2_element in arr2
        {
            println!("{}", arr2_element);
        }
    }

    // 所有权转移 move
    {
        let x = String::from("hello world");
        let y = x;
        // f(x); // Error: use of moved value: `x`
        f(y);
    }

    // 使用 clone 方法显式 Copy
    {
        let x = String::from("I am Sherlock Holmes");
        let y = x.clone();
        f(x);
        f(y);
        // f(x); // Error: use of moved value: `x`
    }

    // 引用, NOT move
    {
        let x = String::from("Welcome to Rust language!");
        let y = &x;
        f_ref(&x);
        f_ref(y);
        f_ref(&x);
        f_ref(y);
        f_ref(&x);
        f_ref(y);
        f_ref(&x);

        // drop(x); 
        // Error: cannot move out of `x` because it is borrowed (after this statement)

        f_ref(y);
    }

    // 修改被引用变量的值
    {
        let mut x = 1001;
        let y = &mut x;

        println!("y = {}", *y); // but `&x` or `&mut x` is not allowed

        *y += 1;

        println!("after increment, y = {}", *y);
    }

    // 数组切片
    {
        let mut x = [0,1,2,3,4];
        let x_slice = &mut x[1..3];

        println!("x[1..3] = {:#?}", x_slice);

        x_slice[0] = 1024;

        println!("x[1..3] = {:#?}", x_slice);
        println!("x = {:#?}", x);

    }

    // 字符串 `String`与`&str`类型
    {
        let mut s = String::new();
        s.push_str("hello ");
        s.push_str("computer programming");

        f_ref(&s);

        let s_slice = &s[1..3];

        // f_ref(s_slice);
        /* expected reference `&String`, found reference `&str` */

        println!("{}", s_slice);

        let mut s2 = "hello world";
        println!("{}", s2);

        s2 = "hello rusttttttt!";

        println!("{}", s2);

        let s2_string = String::from(s2);

        f(s2_string);
    }

    // 元组 tuple
    {
        let my_tuple = (1,1.0,String::from("hello world"),"hello world");
        let (x,y,z,t) = my_tuple;

        assert!(x == 1);
        assert!(y == 1.0);
        assert!(z == String::from("hello world"));
        assert!(t == "hello world");

        println!("All work right!");
    }

    // 元组的另一种索引方法
    {
        let my_tuple = (1,1.0,String::from("hello world"),"hello world");
        let x = my_tuple.0;
        let y = my_tuple.1;
        let z = my_tuple.2;
        let t = my_tuple.3;

        assert!(x == 1);
        assert!(y == 1.0);
        assert!(z == String::from("hello world"));
        assert!(t == "hello world");

        println!("All work right again!");
    }

    // 结构体定义、实例化、成员变量引用
    {
        struct Student
        {
            name:String,
            age:u8,
            score:f64
        }

        fn show_student_info(xiao_ming:&Student)
        {
            println!("Hello, {}! You are {} years old now, and your score is {}",
                        xiao_ming.name,
                        xiao_ming.age,
                        xiao_ming.score);
        }

        let xiao_ming = Student
        {
            name: String::from("Xiaoming"),
            score: 3.9,
            age: 23,
        };

        show_student_info(&xiao_ming);

        let age = 24;
        let score = 4.0;

        let xiao_bai = Student
        {
            name: String::from("小白"),
            age:age,
            score:score
        };

        show_student_info(&xiao_bai);

        let xiao_hei = Student
        {
            name: String::from("小黑"),
            age,
            score
        }; // age, score用之前定义的【同名】变量

        show_student_info(&xiao_hei);

        let xiao_bai_shadow = Student
        {
            name: String::from("小白的影子"),
            ..xiao_bai // 其他属性和xiao_bai相同
        };

        show_student_info(&xiao_bai_shadow);
        
        // xiao_bai的所有成员变量均未失效 故可以正常访问
        // 为什么
        // 因为 xiao_bai 的 age, score 变量是基本类型，赋值时copy而非move
        show_student_info(&xiao_bai);

        let xiao_bai_last_year = Student
        {
            score: 3.3, // 小白去年的成绩是3.3
            ..xiao_bai
        };

        show_student_info(&xiao_bai_last_year);

        // show_student_info(&xiao_bai);
        /*
            partial move occurs 
            because `xiao_bai.name` has type `String`, 
            which does not implement the `Copy` trait
         */

        println!("After partial move, we can NOT access the whole struct, but part of it which NOT be moved");
        println!("xiao_bai's age is {}, score is {}", xiao_bai.age, xiao_bai.score);


    }

    // 结构体方法 impl语句
    {
        struct Circle
        {
            x:f64,  // x of origin
            y:f64,  // y of origin
            r:f64   // radius of the circle
        }

        impl Circle {
            /**
             * create a new instance of Circle
             */
            fn new() -> Self    // Self：指代被实现的方法对应的结构体类型
            {
                Circle { x: 0.0, y: 0.0, r: 1.0 }
            }

            /**
             * get the area of the Circle
             */
            fn get_area(&self) -> f64   // self: 指代调用该方法的实例; &self: 对实例 s 的不可变引用
            {
                3.14159 * self.r * self.r
            }

            fn update(&mut self, x:f64, y:f64, r:f64)   // &mut s: 对实例 s 的可变引用
            {
                self.x = x;
                self.y = y;
                self.r = r;
            }
        }

        let mut my_circle = Circle::new();

        println!("The original circle's area is {}", my_circle.get_area());

        println!("Let's change the radius of it, from 1.0 to 4.0...");
        my_circle.update(0.0,0.0,4.0);
        
        println!("Now, the area is {}", my_circle.get_area());

    }

    // 元组结构体
    {
        struct RGBColor (u8, u8, u8);

        let white = RGBColor(255,255,255);

        println!("For white color, R = {}, G = {}, B = {}", 
                white.0,
                white.1,
                white.2);

    }

    // 枚举
    {
        enum Status
        {
            Stop,   // 红灯停
            Go,     // 绿灯行
            Wait    // 黄灯亮了等一等
        }

        let x = Status::Go as usize;
        println!("绿灯亮了 {:?}", x);
        let y = Status::Stop as usize;
        println!("红灯亮了 {:?}", y);
        let z = Status::Wait as usize;
        println!("黄灯亮了 {:?}", z);
    }

    {
        enum Status
        {
            Stop = 20230726,   // 红灯停
            Go,     // 绿灯行 数值比Stop加一
            Wait = 2023    // 黄灯亮了等一等
        }

        let x = Status::Go as usize;
        println!("绿灯亮了 {:?}", x);
        let y = Status::Stop as usize;
        println!("红灯亮了 {:?}", y);
        let z = Status::Wait as usize;
        println!("黄灯亮了 {:?}", z);
    }

    {
        #[allow(dead_code)]
        enum CardColor
        {
            Hongtao,
            Heitao,
            Meihua,
            Zuanshi
        }
        #[allow(dead_code)]
        struct Poker
        {
            color: CardColor,
            value: u8
        }

        #[allow(unused_variables)]
        {
            // 一张红桃6、一张黑桃8
            let poker_1 = Poker {color: CardColor::Hongtao, value:6};
            let poker_2 = Poker {color: CardColor::Heitao, value:9};
        }


    }

    // 枚举 + 成员属性
    {
        #[allow(dead_code)]
        enum CardColor
        {
            Hongtao(u8),
            Heitao(u8),
            Meihua(u8),
            Zuanshi(u8)
        }

        #[allow(unused_variables)]
        {
            // 一张红桃6、一张黑桃8
            let poker_1 = CardColor::Hongtao(6);
            let poker_2 = CardColor::Heitao(8);
        }


    }

    // Option 枚举
    {
        let x = Some(8);

        // 取出 Option 内的值
        let y = match x {
            Some(value) => value,
            None =>
            {
                panic!("Wrong!!!");
            }            
        };

        assert!(y == 8);

        let z = x.unwrap();

        assert!(z == 8);

        println!("All work right, 3rd...");
    }

    //  Result 枚举类型
    {
        fn divide(x:i32, y:i32) -> Result<i32, String>
        {
            if y==0 
            {
                Err("divided by ZERO!".to_string())
            }
            else {
                Ok(x / y)
            }
        }

        println!("{} / {} = {:#?}", 57, 3, divide(57, 3));
        println!("{} / {} = {:#?}", 57, 0, divide(57, 0));

        let x = divide(57,3);
        match x
        {
            Ok(x) => println!("the result is {}", x),
            Err(msg) => println!("Error! {}", msg)
        }

        println!("------");

        let x = divide(57,0);
        match x
        {
            Ok(x) => println!("the result is {}", x),
            Err(msg) => println!("Error! {}", msg)
        }

    }

    // 泛型
    {
        fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T
        {
            a + b
            /* 直接这么写会报错
            错误信息 cannot add `T` to `T`  
            需要用一些特征约束 `T`  
             */

        }

        println!("1 + 1 = {}", add(1, 1));
        println!("1.2 + 3.14 = {}", add(1.2, 3.14));
        // println!("hello and world means {}", add("hello", "world"));

        /**
         * 获取数组或切片的首个元素
         */
        fn get_first_element<T:Copy>(arr_slice: &[T]) -> T
        {
            let x = arr_slice[0];
            x
        }

        let x = [2333.33;5];
        let y = [1,1,2,3,5,8,13];

        println!("x[0] = {}", get_first_element(&x));
        println!("let z = y[2..5], then z[0] = {}", get_first_element(&y[2..5]));
    }

    {
        println!("未完待续");
    }
}
