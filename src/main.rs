use std::io;

//use std::io;

fn main() {

    let mut temperature = String::new();
    
    let mut unit = String::new();

    println!("Hello! Welcome to the temperature conversion");

    println!("Input a temperature!");
    
    io::stdin().read_line(&mut temperature).expect("Failed to take in value");

    while unit.trim() != "F".trim() || unit.trim() != "f".trim() || unit.trim() != "C".trim() || unit.trim() != "c".trim(){

        println!("Is this Fahernheit or Celesius? Type F or C. Pick one");
        
        unit = String::new();
        io::stdin().read_line(&mut unit).expect("Failed to take in value");       
    

        if unit.trim() == "F".trim() || unit.trim() == "f".trim(){
            
            let temp:f32 = temperature.trim().parse().expect("Not a valid number");
            let result:f32 = (5.0/9.0) * (temp - 32.0);
            
            println!("F to C: {}", result);
            break;
        }else if unit.trim() == "C".trim() || unit.trim() == "c".trim() {
            println!("Did I get here 2");
            let temp:f32 = temperature.trim().parse().expect("Not a valid number");
            let result:f32 = ((9.0/5.0) *temp) + 32.0;
            println!("F to C: {}", result);
            break;
        }else {
            println!("Not a valid inpus. Try again");
        }
    }

}
