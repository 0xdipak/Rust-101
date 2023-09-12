// 1
// enum Direction {
//     Left,
//     Right,
//     Up,
// }


// enum Color {
//     Red,
//     Black,
//     Blue,
// }

// fn print_color(my_color: Color) {
//     match my_color {
//         Color::Red => println!("red !"),
//         Color::Black => println!("black !"),
//         Color::Blue => println!("blue !"),
//     }
// }



// struct GroceryItem {
//     stock: i32,
//     price: f64,
// }

fn main() {
    // 1
    // let go = Direction::Left;
    // match go {
    //     Direction::Left => println!("go left !"),
    //     Direction::Right => println!("go right !"),
    //     Direction::Up => println!("up !"),
    // };

    // 2
    // print_color(Color::Black);



    // Struct

    // struct ShippingBox {
    //     depth : i32,
    //     width : i32,
    //     height: i32,
    // }

    // let my_box = ShippingBox {
    //     depth: 3,
    //     width: 2,
    //     height: 5,
    // };

    // let tall = my_box.height;
    // println!("the box is {:?} units tall.", tall);


    // let cereal = GroceryItem {
    //     stock: 10,
    //     price: 2.99
    // };

    // println!("stock : {:?}", cereal.stock);
    // println!("price : {:?}", cereal.price);



    enum Flavour {
        Sparkling,
        Sweet,
        Fruity,
    } 

    struct Drink {
        flavour: Flavour,
        fluid_oz: f64,
    }


    fn dfo(drink:Drink) {
        match drink.flavour {
           Flavour::Sparkling => println!("flavour : sparkling !"),
           Flavour::Sweet => println!("flavour : sweet !"),
           Flavour::Fruity => println!("flavour : fruity !"),
        }

        println!("oz: {:?}", drink.fluid_oz);
    }


    let sweet = Drink {
        flavour: Flavour::Sweet,
        fluid_oz: 6.0,
    };

    dfo(sweet);



}
