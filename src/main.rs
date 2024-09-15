fn main() { //Starting part of the code.
    let ans = is_even(11);
    println!("{}", ans);
}
//cargo run ~ run the current file. 

/* Two types of file. 
    ->Binary(What we are using)
    ->Library */

//Q1. Write a fun is_even that takes a number as an input and returns true if it is even.

fn is_even(num: i32) -> bool{ //i32, i64, u32, u64
    if num%2 == 0{
        return true;
    }
    else{
        return false;
    }
}
