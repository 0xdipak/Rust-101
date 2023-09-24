fn main() {
    // enum Access {
    //     Full,
    // }

    // fn one_two_three() -> (i32, i32, i32) {
    //     (1,2,3)
    // }

    // let numbers = one_two_three();
    // let (x, y, z) = one_two_three();
    // println!("{:?}, {:?}", x, numbers.0);
    // println!("{:?}, {:?}", y, numbers.1);
    // println!("{:?}, {:?}", z, numbers.2);


    // let (employee,access) = ("Jack", Access::Full);
    // println!("Employee :{:?}, Access:{:?} ", employee, access);


    // let coord = (2,3);
    // println!("{:?}, {:?}", coord.0, coord.1);
    // // or
    // let (x,y) = (2,3);
    // println!("{:?}, {:?}", x, y);

    // let (name, age) = ("Emma", 20);


    fn coord() -> (i32, i32) {
        (5,10)
    }

    let (x,y) = coord();

    if y > 5 {
        println!("Greater than 5 !");
    } else if y < 5 {
        println!("Less than 5 !");
    } else {
        println!("Equal to 5 !");
    }


}
