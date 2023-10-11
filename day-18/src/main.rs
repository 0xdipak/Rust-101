use std::collections::HashMap;


#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            content: "shirt".to_owned(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "pant".to_owned(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "hoodie".to_owned(),
        },
    );

    for(locker_number, content) in lockers.iter() {
        println!("number: {:?}, content : {:?}", locker_number, content);
    }




    // Activity
    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couch", 0);


    let mut total_stock = 0;
    for (item, qty) in stock.iter() {
        total_stock = total_stock + qty;
        let stock_count = if qty == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", qty)
        };

        println!("item = {:?}, stock = {:?}", item, stock_count);
    }
    println!("total stock = {:?}", total_stock);
}
