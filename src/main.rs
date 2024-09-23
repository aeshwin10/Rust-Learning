/* Two types of file. 
    ->Binary(What we are using) - exec program that can be run directly.
    ->Library 
*/

//cargo run ~ run the current file. 

use chrono::{Local};
fn main() { //Starting part of the code.
    // let ans = is_even(11);
    // println!("{}", ans);

    // let x = 4;
    // println!("{}", fib(x));

    // let name= String::from("Ashwin");
    // let len = get_str_len(&name);
    // println!("the length of the string is: {}", len);

    //---------------------------------------------------

    // let user = User {
    //     first_name: String::from("Ashwin"),
    //     last_name: String::from("Essan"),
    //     age: 20,
    // };

    // println!("{}", user.first_name);
    // println!("{}", user.last_name);
    // print!("{}", user.age);


    // let rect1 = Rect{
    //     width: 10,
    //     height: 10,
    // };

    // println!("{}", rect1.area());
    // println!("{}", rect1.perimeter());
    // println!("{}", Rect::debug());      //calling a static function - fun that belongs to the struct and not the any object of the struct.

    //---------------------------------------------------

    // let my_shape = Shape::Circle(5.0);

    // fn calculate_area(shape: Shape) -> f64 {
    //     let area = match shape {
    //         Shape::Circle(r) => 3.14 * r * r,       //Pattern matching
    //         Shape::Rectangle(w, h) => w * h,
    //         Shape::Square(s) => s * s,
    //     };
    //     return area;                                      //we can also implicitly return - no need to store in the variable area.
    // }

    // println!("{}", calculate_area(my_shape));

    // let index = find_first_a(String::from("ashwin"));

    // match index{
    //     Option::Some(value) => println!("index is {}", value),
    //     Option::None =>  println!("{}","a not found")
    // }

    //---------------------------------------------------
    
    // let ans = read_from_current(String::from("a.txt"));
    // match ans{
    //     Ok(data) => println!("{}", data),
    //     Err(err) => println!("{}", err),
    // }

    //---------------------------------------------------
    //Package Management - You can add an external crate to your project by running
    //cargo add crate_name
    //Example - cargo add chrono - also check in cargo.toml. One dependency will be added there
    

    let now = Local::now();
    println!("Current time is {}", now);



}
//--------------------------------------------------------------------------------------------------------------------------
//Intro

//Q1. Write a fun is_even that takes a number as an input and returns true if it is even.

fn is_even(num: i32) -> bool{ //i32, i64, u32, u64
    if num%2 == 0{
        return true;
    }
    else{
        return false;
    }
}

//Q2. Write a function that finds the fibbonacci of a number it takes as input.
fn fib(num: i32) -> i32{
    let mut first = 0;  //Type inference happens(no need to explicitly specify the datatype)
    let mut second = 1; //Also, all the variables are non-mutable by default, so use 'mut' keyword
    if num==0{
        return first;
    }
    if num==1{
        return second;
    }

    for _ in 0..num-1{         //better practise to use '_' than i, if we are not using i inside the loop.
        let temp = second;
        second = second + first;
        first = temp;

    }
    return second;
}

//Q3. Write a fun get_string_length that takes a string an an input and returns its length
fn get_str_len(s: &str) ->usize {
    s.chars().count()  //Implicit return
}

//--------------------------------------------------------------------------------------------------------------------------
//Struct
struct User{
    first_name: String,
    last_name: String,
    age: i32,
}

struct Rect{
    width: i32,
    height: i32,
}

impl Rect{                          //impl can be used for structs, enums
    fn area(&self) -> i32{              //similar to python, self will the instace of the struct.
        return self.width * self.height;
    }
    fn perimeter(&self) -> i32{
        2* (self.width + self.height)
    }
    fn debug() -> i32{                 //acts as a static function. Can be called direcly on the class.
        return 1;
    }
}

//--------------------------------------------------------------------------------------------------------------------------
//Enum - when we know a variable will take only particular set of values, we use ENUM.
//     - ENUM can also have associated valued with it.

enum Shape{
    Rectangle(f64, f64),
    Circle(f64),
    Square(f64),
}

//--------------------------------------------------------------------------------------------------------------------------
//Option - An enum in Rust used for representing an optional value. It can either be Some(T), 
//indicating the presence of a value of type T, or None, indicating the absence of a value.

fn find_first_a(s: String) -> Option<i32>{
    for (index, char) in s.chars().enumerate(){
        if char == 'a'{
            return Some(index as i32);        //index here is usize so typecast it to i32
        }
    }
    return None;
}


//--------------------------------------------------------------------------------------------------------------------------
//Result - Result Enum lets you return either OK value or Err Value.
// The result enum is how you can do error handling.

//Q. Write a function that reads the contents of a file.
use std::fs::read_to_string;

fn read_from_current(file_path: String) -> Result<String, String>{
    let result = read_to_string(file_path);
    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(String::from("File not read")),
    }
}

//--------------------------------------------------------------------------------------------------------------------------


