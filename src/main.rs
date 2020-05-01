use std::io;

fn main() {
    println!("Hello, temprture conversion by james wright!");
    
    loop {
     
        let mut temp_type = String::new();

        println!("Please input type of temp (C)elsius, (F)arenheit or (E)nd.");  

        io::stdin().read_line(&mut temp_type)
           .expect("Failed to read line");
        
        if temp_type.trim() == "E" { 
            println!("thanks and goodbye, you selected (E)nd.");  
            break 
        }

        println!("Now please enter temperature value.");
        let mut temp_to_convert = String::new();
            
        io::stdin().read_line(&mut temp_to_convert)
            .expect("Failed to read line");
           
        let temp_to_convert: f32 = match temp_to_convert.trim().parse() {
               Ok(num) => num,
               Err(_) => continue,
           };
            
        if temp_type.trim() == "C" || temp_type.trim() == "c" {
            let temp_converted = james_lib::celsius_convert_lib(temp_to_convert); 
            let temp_type_from = " Celsius";
            let temp_type_to ="Farenheit";
            println!("The value of {} {} is {:.2} {}", temp_to_convert, temp_type_from, temp_converted, temp_type_to);
        }
        if temp_type.trim() == "F" || temp_type.trim() == "f" {
            let temp_converted = farenheit_convert(temp_to_convert); 
            println!("The value of {} Farenheit is {:.2} Celsius", temp_to_convert, temp_converted);
        }
    }
}

fn celsius_convert(a: f32) -> f32 {
    let a = a / 5.0;
    let a = a * 9.0;
    a + 32.0
}

fn farenheit_convert(a: f32)-> f32 {
    let a = a - 32.0;
    let a = a / 9.0;
    a * 5.0
}