fn task1() {
    // region 1 start
    let x = 4;

    // region 1 end

    // 输出: x = 4
    println!("task1: x = {}", x);
}


fn task2() {
    // region 2 start
    // warning: value assigned to `x` is never read
    // let mut x:usize = 4;
    // x = 5;
    let x = 5;
    // region 2 end

    // 输出: x = 5
    println!("task2: x = {}", x);
}


fn calc(x: i32) -> i32 {
    // region 3 start
    if x > 4 {
        x
    }
    else {
        x / 2i32
    }
    // region 3 end
}

fn task3() {
    
    let x = 2;
    let y = calc(x);

    // 输出: x = 1
    println!("task3: x = {}", y);
}


fn task4() {
    let date = 724;
    
    // region 4 start
    let z = if date % 2 == 0 {
        4
    }
    else {
        2
    };
    // region 4 end

    // 输出: z = 4
    println!("task4: z = {}", z);
}


fn task5() {
    // region 5 start

    // 输出: 0 1 2 3 4 5 6 7 8 9 10
    print!("task5: ");
    for i in 0..=10 {
        print!("{} ", i);
    }
    println!();
    // region 5 end
}


fn task6() {
    // region 6 start
    #[allow(unused_assignments)] // 不抛出变量未使用的warning
    let mut x = 10;

    x = 1919810;
    // region 6 end

    // 输出: x = 1919810
    println!("task6: x = {}", x);
}


fn task7() {
    // region 7 start
    let mut x = 1;

    let y = &mut x;

    *y = 10;
    // region 7 end

    // 输出: x = 10
    println!("task7: x = {}", x);
}


fn task8() {
    // region 8 start
    let mut x = 8;

    let y = &mut x;
    
    *y = 10;

    let z = &x;
    // region 8 end

    // 输出: x = 10
    println!("task8: x = {}", z);
}


fn output9(_x: &String) {
    print!("task9: ");
}

fn task9() {
    let x = String::from("hello world");
    // region 9 start
    output9(&x); // borrowed here
    // region 9 end

    // 输出: x = hello world
    println!("x = {}", x);
}


fn task10() {
    let x = String::from("hello world");

    // region 10 start
    let y = &x;
    // region 10 end

    // 输出: 
    // x = hello world
    // y = hello world
    println!("task10: ");
    println!("x = {}", x);
    println!("y = {}", y);
}


fn task11() {
    let mut x = String::from("hello world");
    let y = &mut x;
    
    // region 11 start
    *y = String::from("hello rust");
    // region 11 end

    // 输出: x = hello rust
    println!("task11: x = {}", x);
}


fn task12() {
    let mut x = String::from("hello world");
    // region 12 start
    let y = &mut x;
    *y = String::from("hello rust");
    // region 12 end

    // 输出: x = hello rust
    println!("task12: x = {}", x);
}


fn task13() {
    let mut x = 10;
    // region 13 start
    // 请只进行语句交换，而不要修改语句内容
    let z = &mut x;
    *z = 20;
    let y = &x;
    // region 13 end

    // 输出: x = 20
    println!("task13: x = {}", y);
}


fn task14() {
    let x = 10;
    // region 14 start
    print!("task14: ");
    match x {
        10 => println!("x = 10"),
        20 => println!("x = 20"),
        _ => println!("Nothing...")
    };
    // region 14 end

    // 输出: x = 10
}


fn task15() {
    let x = String::from("hello world");
    let _y = String::from("hello rust");
    let _z = String::from("hello world");

    println!("task15: ");
    // region 15 start
    #[allow(unreachable_patterns)]  // to avoid unreachable_patterns warning
    match &x {
        _y => println!("x = hello world"),
        _z => println!("x = hello rust"),
        _ => println!("x = others"),
    }
    // region 15 end

    // 输出: (注意有两个)
    // x = hello world
    // x = hello world
    println!("x = {}", x);
}


fn main()
{
    task1();
    task2();
    task3();
    task4();
    task5();
    task6();
    task7();
    task8();
    task9();
    task10();
    task11();
    task12();
    task13();
    task14();
    task15();
}