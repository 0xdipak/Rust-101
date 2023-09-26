// struct Temperature {
//     degree_f: f64,
// }

// impl Temperature {
//     fn freezing() -> Self {
//         Self { degree_f: 32.0 }
//     }
//     fn show_temp(&self) {
//         println!("{:?} degree F", self.degree_f);
//     }
// }

fn main() {
    // let hot = Temperature { degree_f: 99.9 };
    // hot.show_temp();

    // let cold = Temperature::freezing();
    // cold.show_temp();


    enum Color {
        Brown,
        Red,
    }

    impl Color {
        fn print(&self) {
            match self {
                Color::Brown => println!("brown"),
                Color::Red => println!("red"),
            }
        }
    }

    struct  Dimensions {
        width:f64,
        height:f64,
        depth:f64,
    }

    impl Dimensions {
        fn print(&self) {
            println!("width: {:?}", self.width);
            println!("height: {:?}", self.height);
            println!("depth: {:?}", self.depth);
        }
    }

    struct ShippingBox {
        color:Color,
        weight:f64,
        dimensions: Dimensions
    }

    impl ShippingBox {
        fn new(weight: f64, color: Color, dimensions:Dimensions) -> Self {
            Self { color, weight, dimensions}
        }

        fn print(&self) {
            self.color.print();
            self.dimensions.print();
            println!("weight: {:?}", self.weight);
        }
    }

    let small_dimensions = Dimensions {
        width:1.0,
        height:2.0,
        depth:3.0,
    };

    let small_box = ShippingBox::new(5.0,Color::Red, small_dimensions);
    small_box.print();
}
