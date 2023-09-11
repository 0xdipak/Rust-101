
fn main() {

    // let some_bool = true;
    // match some_bool {
    //     true => println!("its true"),
    //     false => println!("its false"),
    // };


    // let some_int = 4;
    // match some_int {
    //     1 => println!("its 1"),
    //     2 => println!("its 2"),
    //     3 => println!("its 3"),
    //     _ => println!("its something else"),
    // }

    // making decisions with match
    // let my_name = "Jayson";
    // match my_name {
    //     "Jayson" => println!("that is my name"),
    //     "bob" => println!("not my name"),
    //     "Alice" => println!("hello alice"),
    //     _ => println!("nice to meed you")
    // };


    // Repetition using loop

    // let mut i = 3;
    // loop {
    //     println!("{:?}", i);
    //     i = i - 1;
    //     if i == 0 {

    //         break;
    //     }
    // }

    // println!("Done !");

    let mut my_int = 1;
    loop {
        println!("{:?}", my_int);
        my_int = my_int + 1;
        if my_int > 4 {
            break;
        }
    }
    println!("Done !");



}
