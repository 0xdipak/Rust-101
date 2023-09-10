fn main() {
    //Control Flow if-else

    let age = 15;
    if age >= 21 {
        println!("ok to purchase");
    }else {
        println!("can not purchase");
    }

    let say_yes = true;
    if say_yes {
        println!("Hello, guys !");
    } else {
        println!("Goodbye, guys !");
    }


    let my_var = 1;

    if my_var > 5 {
        println!("Greater than 5...");
    } else if my_var == 5 {
        println!("Equal to 5...");
    } else {
        println!("Less than 5..");
    }
}
