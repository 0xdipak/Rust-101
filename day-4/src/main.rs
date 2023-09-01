fn main() {
    let a = 10;
    if a > 10 {
        println!("Big number !");
    } else {
        println!("Small number !");
    }

    // if - else - else if
    if a > 10 {
        println!("Greater than 10 !");
    } else if a == 10 {
        println!("Equal to 10 !");
    } else {
        println!("Smaller than 10 !");
    }
}
