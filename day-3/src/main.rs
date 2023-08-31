

fn main() {

    // function
    fn add(a:i32, b:i32) -> i32 {
        a+b
    }

    let sum:i32 = add(5,5);

    println!("{:?}", sum);



    //println! macro

    let life = 22;
    println!("hello");
    println!("{:?}", life);
    println!("the meaning is {:?}", life);

    println!("{life}");
    println!("{life:?}"); // :? for debugging
}
