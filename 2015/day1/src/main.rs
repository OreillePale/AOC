use std::io;

fn check_gender(gen: &mut String){

    gen.push_str("test");
}

fn main() {
    // get user input
    println!("Quel est votre genre ?");
    let mut gender = String::new();
    io::stdin().read_line(&mut gender).expect("Une erreur s'est produite.");
    gender = gender.trim().parse().expect("Didn't work");
    println!("{gender}");
    
    check_gender(&mut gender);
    println!("{gender}");
}
