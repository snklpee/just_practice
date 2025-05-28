use std::io;

fn main() {
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: f32 = match guess.trim().parse(){
        Ok(number)=>number,
        Err(_)=>{
            let guess: f32 = 0.0;
            println!("setting guess as {} as the expected input was not found",guess);
            guess
        }
    };

    println!("guess is now: {} and guess+0.3 = {}",guess,guess+0.3);
    
}