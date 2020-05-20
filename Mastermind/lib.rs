

use std::io;
use rand::Rng;

pub const LENGTH:usize = 4; // VERY CONFUSED ABOUT USIZE AND U32, CANT USE ANYTHING OTHER THAN USIZE FOE ARRAY??
pub enum IsMMCodeMatched {
    Exact,
    Partial,
    NotMatched
}

pub struct MMcodestruct { // VERY CONFUSED wanted to use this in array or vector??
    matchedstatus: IsMMCodeMatched,
    mmcodechar : char
}

pub fn generate_target() -> String
   {
    let mut mmcode = String::new();
    
    for x in 0..LENGTH
        {
    
        let secret_number:i32 = rand::thread_rng().gen_range(1, 5);

        if secret_number == 1 {mmcode.push_str("G");}
        if secret_number == 2 {mmcode.push_str("Y");}
        if secret_number == 3 {mmcode.push_str("B");}
        if secret_number == 4 {mmcode.push_str("R");}
        }

    mmcode

}


pub fn guess_target(target:&String) {
   
    let mut x:usize = 0;
    let mut targetarray: [char; 4] = [' '; 4];
        
    for c in target.chars() {
        targetarray[x] = c;    
        x = x + 1;
    }
    
    let mut tries:u8 = 0;
  
    loop {
     
        let mut guess = String::new();

        println!("Please input {} character guess (G)reen, (R)ed, (B)lue , (Y)ellow or (E)nd.",LENGTH );  

        io::stdin().read_line(&mut guess)
           .expect("Failed to read line");
        
        guess=guess.trim().to_string();

        if guess == "E" { 
            println!("thanks and goodbye, you selected (E)nd. the answer was {}", target);  
            break 
        }
 // initially tried to use array of struct and then vec of struct. couldn't get it to work. 
 //     let guessvec: Vec<MMcodestruct> = Vec::new();

//      for c in guess.chars() {
//      println!("{}", c);
//      guessvec.mmcodechar.push(&c);
//      println!("{}", &guessvec.mmcodechar[1]);
//}
        let mut number_exact:u32 = 0;
        let mut number_colourmatch:u32 = 0;
        let mut guessarray: [char; LENGTH] = [' '; LENGTH];
        let mut guessstatusarray: [IsMMCodeMatched; 4] = [IsMMCodeMatched::NotMatched, IsMMCodeMatched::NotMatched,IsMMCodeMatched::NotMatched,IsMMCodeMatched::NotMatched];
        let mut targetstatusarray: [IsMMCodeMatched; 4] = [IsMMCodeMatched::NotMatched, IsMMCodeMatched::NotMatched,IsMMCodeMatched::NotMatched,IsMMCodeMatched::NotMatched];
 
        //      cycle thru and find exact matches;
        
        let mut position:usize = 0;
  
        for c in guess[0..LENGTH].chars() {
            guessarray[position] = c;    
//          println!("exact c{} t{} g{} p{}", c, targetarray[position],guessarray[position], position );
            if targetarray[position] == guessarray[position] {
                guessstatusarray[position] = IsMMCodeMatched::Exact;
                targetstatusarray[position] = IsMMCodeMatched::Exact;
                number_exact = number_exact + 1;
            }
        //      some debug messages and practice using enum match and enum if letcycle thru and find exact matches;
//
//           match guessstatusarray[position] {
//                IsMMCodeMatched::Exact => println!("Exact match using match"),
//                _ => (),
//           }
//           if let IsMMCodeMatched::Exact = guessstatusarray[position] {
//                println!("enum if let for exact, current number exact {}", number_exact);
 //          }
            position = position + 1;
        }

        //      cycle thru and find colour matches if target position not exact matched or partially matched once;
        let mut position:usize = 0;
  
        for c in guess[0..LENGTH].chars() {
    
//          println!("colour{} t{} g{} p{}", c, targetarray[position],guessarray[position], position );
            if let IsMMCodeMatched::NotMatched = guessstatusarray[position] {  // no exact matches
                let mut index: usize = 0;
                while index < LENGTH {
                    if let IsMMCodeMatched::NotMatched = targetstatusarray[index] { // if terget exact or partially matched once then exclude
                        if targetarray[index] == guessarray[position] { // new partial match and target char not partially matched already
                            targetstatusarray[index] = IsMMCodeMatched::Partial; // set target status so only 1 partial
                            number_colourmatch = number_colourmatch + 1;
                            index = LENGTH;  // inelegant way of escaping while, could have used loop and break
                        }    
                    }
                    index += 1;
                }
            }

            position += 1;
        }

        tries += 1;

        if number_exact == 4 {
            println!("Your guess {} was correct after {} tries", guess, tries);
            break
        } else {
            println!("for guess {} exact matches {} colour matches {} after {} tries", guess, number_exact, number_colourmatch, tries);
        }    
    }
}
