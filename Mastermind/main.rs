
use mastermind::generate_target;
use mastermind::guess_target;
//use mastermind::LENGTH;

fn main() {
   
    let target = generate_target();

//    println!("The target is {}", target);
   
    guess_target(&target);
}

