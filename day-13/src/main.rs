
// struct Test {
//     score: i32
// }




fn main() {
    // let my_scores = vec![
    //     Test {score: 90},
    //     Test {score: 50},
    //     Test {score: 80},
    //     Test {score: 60},
    // ];
    // for test in my_scores {
    //     println!("score = {:?}", test.score);
    // }


    // let my_numbers = vec![10, 20, 30, 40];
    // for num in &my_numbers {
    //     match num {
    //         30 => println!("thirty"),
    //         _ => println!("{:?}", num),
    //     }
    // }
    // println!("number of elements = {:?}", my_numbers.len());




    // String
    // struct LineItem {
    //     name: String,
    //     count:i32,
    // }

    // fn print_name(name: &str) {
    //     println!("name : {:?}", name);
    // }

    // let receipt = vec![
    //     LineItem {
    //         name: "cereal".to_owned(),
    //         count: 1,
    //     },
    //     LineItem {
    //         name: String::from("fruit"),
    //         count: 3,
    //     }
    // ];

    // for item in receipt {
    //     print_name(&item.name);
    //     println!("count : {:?}", item.count);
    // }




    struct Person {
        name: String,
        fav_color: String,
        age: i32
    }

    fn print(data: &str) {
        println!("{:?}", data);
    }

    let people = vec![
        Person{
            name: String::from("George"),
            fav_color: String::from("Green"),
            age: 7
        },
        Person{
            name: String::from("Panda"),
            fav_color: String::from("Blue"),
            age: 9
        },
        Person{
            name: String::from("Ram"),
            fav_color: String::from("Pink"),
            age: 15
        },
    ];


    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color);
            println!("Age : {:?}", person.age);
        }
    }

}