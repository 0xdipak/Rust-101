fn main() {
    //Advance Match
    // enum Discount {
    //     Percent(i32),
    //     Flat(i32),
    // }

    // struct Ticket {
    //     event: String,
    //     price: i32,
    // }

    // let n = 3;
    // match n {
    //     3 => println!("Three"),
    //     other => println!("number {:?}", other),
    // };

    // let flat = Discount::Flat(2);
    // match flat {
    //     Discount::Flat(2) => println!("Flat 2"),
    //     Discount::Flat(amount) => println!("Flat discount of : {:?}", amount),
    //     _ => (),

    // }

    // let concert = Ticket {
    //     event: "concert".to_owned(),
    //     price: 50,
    // };
    // match concert {
    //     Ticket {price, ..} => println!("price {:?}", price),
    // }


    // Ex
        enum Ticket {
            Backstage(f64, String),
            Standard(f64),
            Vip(f64, String),
        }

        let tickets = vec![
            Ticket::Backstage(50.0, "Billy".to_owned()),
            Ticket::Standard(15.0),
            Ticket::Vip(30.0, "Amy".to_owned()),
        ];

        for ticket in tickets {
            match ticket {
                Ticket::Backstage(price, holder)=> println!("Backstage = Holder : {:?}, price : {:?}", holder, price),
                Ticket::Standard(price)=> println!("Standard = price : {:?}",price),
                Ticket::Vip(price, holder)=> println!("Vip = Holder : {:?}, price : {:?}", holder, price),

            }
        }



}
