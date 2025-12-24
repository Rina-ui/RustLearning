fn main()
{
    let name = "Othniella";
    let age = 78;
    println!("hi {} you are really {} years old?", name, age);

    //change variable
    let mut x = 5;
    println!("Before x value is {}", x);
    x = 10;
    println!("After x value is {}", x);

    //different type
    let my_num: i32 = 5;
    let my_double: f64 = 5.99;
    let my_letter: char = 'D';
    let my_bool: bool = true;
    let my_text: &str = "Hello";


    //constant
    const BIRTHTEAR: i32 = 1665;
    const MINUTES_PER_HOUR: i32 = 60;
    /*
    *si on ecrit une constante sans definir le type comme i32, f64, etc il y aura une erreur
    *Si on veut qu'une variable puisse prendre une autre valeur lorsqu'on a definit on doit use mut et aussi les types ne sont pas obligatoire contrairemnet au const
    */

    //calcul arithmerique
    let add = 5 + 3;
    let sub  = 4 - 2;
    let mult = 5 * 4;
    let div = 8 / 3;

    println!("The value of add is {} , of sub is  {} , for mult {} and of div is {}", add, sub, mult, div);


    //exemple avec changement de variable
    let mut x = 5;
    println!("Before x value is {}", x);
    x += 4;
    println!("After x value is {}", x);
    x -= 6;
    println!("After x value is {}", x);
    x *= 3;
    println!("After x value is {}", x);
    x /= 3;
    println!("After x value is {}", x);


    let a = 5;
    let b = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);

    //operateurs logique
    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);

    //boolean variable
    let is_programming_fun: bool = true;
    let no = false;
    println!("Is programming fun: {}", no);

    let age = 20;
    let can_vote = age >= 18;

    println!("Is vote: {}", can_vote);

    let logged_in = false;

    if logged_in {
        println!("Welcome back!");
    }else {
        println!("Retry");
    }

    //match
    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    //differentes facons de l;use
    let day = 6;

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    //loop
    let mut count = 1;

    loop {
        println!("Hello World!");

        if count == 3 {
            break;
        }

        count += 1;
    }

    let mut count = 1;

    let result = loop {
        println!("Hello!");

        if count == 3 {
            break count; // Stop the loop and return the number 3
        }

        count += 1;
    };

    println!("The loop stopped at: {}", result);
}
