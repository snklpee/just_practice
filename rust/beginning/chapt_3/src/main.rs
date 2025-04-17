fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    {
        let x = "`x` is now a string";
        println!("Is x really a string in the inner scope now ? \n
        lets se whats become of x:\n {x}")
    }

    println!("The value of x is: {x}");
}