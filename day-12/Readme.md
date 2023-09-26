Impl
- E.g.

// Normal
struct Temperature {
    degree_f: f64,
}

fn show_temp(temp:Temperature) {
    println!("{:?} degree F", temp.degree_f);
}

fn main() {
    let hot = Temperature {
        degree_f:99.9,
    };
    shoe_temp(hot);
}


// Using impl
struct Temperature {
    degree_f: f64,
}

impl Temperature {
    fn show_temp(temp:Temperature) {
    println!("{:?} degree F", temp.degree_f);
}

}
fn main() {
    let hot = Temperature {
        degree_f:99.9,
    };
    Temperature::shoe_temp(hot);
}
