use std::io;

fn main() {
    loop {

        println!("Please input your age:");
        
        let mut age = String::new();
        
        // Take input
        io::stdin()
            .read_line(&mut age)
            .expect("Failed to read line.");
        
        // convert to number
        let age: u32 = match age.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
       // multiply by 365
       let age_in_days: u32 = 365 * age;
       // print output
    
       println!("The age {0} in days is: {1}", age, age_in_days);
    }
}
