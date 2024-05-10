fn main() {
    // Shadowing and some math
    let x = 5;

    let x = x * 2;
    println!("{x}");

    {
        let x = x * 3;
        println!("{x}");
    }
    println!("{x}");

    // Tuples
    let tup = ("Ben", "Grace", "Lucy");
    let (x, y, z) = tup;
    println!("{y}, {x}, {z}");

    // Arrays
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    let new_a = [7; 7];

    let two = a[1];
    let seven = new_a[6];
    println!("{two}, {seven}");

    // functions
    my_function();
    number_time(10);

    let y = { // This pretty much acts like a function with a return value
        let x = 4;
        x + 1
    };

    println!("{y}");

    let mut xn = five();
    println!("{xn}");

    xn = one_more(xn);
    println!("{xn}");


    // control flow
    let my_num = 3;

    if my_num < 5
    {
        println!("The number is less than 5.");
    }

    else 
    {
        println!("The number is greater than 5.");
    }

    let condition = true;
    let the_new_num = if condition {5} else {6};

    println!("The newest of nums is: {the_new_num}");

    // loops
    let mut break_loop = 0;
    let result = loop
    {
        break_loop += 1;
        println!("Again!");

        if break_loop == 10
        {
            break break_loop * 2;
        }
    };

    println!("The result is: {result}");


    let mut count = 0;
    'counting_up: loop{
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaing = {remaining}");
            if remaining == 9
            {
                break;
            }
            if count == 2
            {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    let mut this_num = 3;

    while this_num != 0
    {
        println!("{this_num}!");

        this_num -= 1;
    }

    println!("LIFT OFF!");

    for i in new_a
    {
        println!("The value of all of my sevens: {i}");
    }

    for number in (1..4).rev()
    {
        println!("Loop number be like: {number}")
    }

}

fn my_function()
{
    println!("This came from a function call!");
}

fn number_time(x: i32)
{
    println!("So did this: {x}");
}

fn five() -> i32 { // This is a return function
    5
}

fn one_more(x: i32) -> i32{
    x + 1
}