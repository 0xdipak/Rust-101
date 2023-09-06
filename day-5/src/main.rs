fn main() {
    // let mut a = 0;
    // loop {
    //     if a == 5 {
    //         break;
    //     }
    //     println!("{}", a);
    //     a += 1;
    // }

    // while a != 5 {
    //     println!("{}", a);
    //     a += 1;
    // }


    // Functions
    // fn first_name(name: &str) -> &str {
    //     return name;
    // }
    // fn last_name(last: &str) -> &str {
    //     return last;
    // }

    // let f_name = first_name("Dipak");
    // let l_name = last_name("Sharma");

    // println!("First Name : {} and Last Name : {}", f_name, l_name);


    // Arithmetics
    // let sum = 2 + 2;
    // println!("{}", sum);

    // fn sub(a:i32, b:i32) -> i32 {
    //     return  a-b;
    // }

    // let subt = sub(10,5);
    // println!("The total sub : {}", subt);

    fn add(x:i32, y:i32) -> i32 {
        return  x + y;
    }

    let total_sum = add(10, 20);
    println!("Total sum : {:?}", total_sum);



}
