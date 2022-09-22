use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

//here is an example of scoop in rust the s variable at the end of curly brackets rust call a function "drop"
//and it drop s from memory soo in line 16 i can't access the variable s
    // {
    //     let mut s = String::from("hello");

    //     s.push_str(", world");

    //     println!("{}",s);
    // }

    // println!("{}", s);

    


}



fn guessing_game() {

    loop {
        // generate a number from the range 1-100
        let number = rand::thread_rng().gen_range(1..=10);
        // the "mut" it means this variable is mutable because all variables in rust are immutable
        let mut guess = String::new();
        println!("guess the Number");
        println!("please input a number");

        //read input 
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the line");

        //here we parse the String to Number soo we could compare it with the number generated
        let guess: u32 = guess.trim().parse().expect("please provide number");

    // here we compare the number generated and the number provided
            match guess.cmp(&number) {
                Ordering::Less => println!("too small"),
                Ordering::Equal => {
                    println!("is equal");
                    break;
                },
                Ordering::Greater => println!("too biig")
                
            };
    };
}
