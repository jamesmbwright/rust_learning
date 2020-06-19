
//use highscore::generate_target;
//use highscore::guess_target;
//use mastermind::LENGTH;

fn main() {
   
    use std::collections::HashMap;

    let mut count: i32 = 0;  
    let mut sum: f32 = 0.0;
    let mut maxmodecount = 0;
    let v = vec![100.0, 32.0, 57.0, 58.2, 58.0, 32.0, 32.4, 58.0, 58.0];
    let mut modecount = HashMap::new();
   
    let mut modevalue = String::new();

    for i in &v {
        println!("{}", i);
        
        sum += i;
        count += 1;

        let mcount = modecount.entry(i.to_string()).or_insert(0); // inserts a new hashmap entry is one doesn't exists and always increments count by one
        *mcount += 1;
    }
    let avg: f32 = sum / count as f32;
    println!("sum {:.2}, count {} avg {:.2}", sum, count, avg);
    
   
    for (key, value) in &modecount {
        println!("{}: {}", key, value);
        if *value > maxmodecount {    // so the "for" iteration on hashmap returns reference so need a * derefe, took about 2 hours to fix this compile error
            modevalue = key.to_string();
            maxmodecount = *value;   // same here for dereferencing
        
        } 
    }       
    println!("a mode is {}, freq {}", modevalue, maxmodecount);
    
}
