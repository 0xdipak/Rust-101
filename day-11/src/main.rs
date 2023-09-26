// struct Book {
//     pages: i32,
//     ratings: i32,
// }

// fn display_page_count(book: &Book) {
//     println!("pages = {:?}", book.pages);
// }

// fn display_rating(book: &Book) {
//     println!("rating = {:?}", book.ratings);
// }



fn main() {
    // let book = Book {
    //     pages: 5,
    //     ratings: 9,
    // };
    // display_page_count(&book);
    // display_rating(&book);



    struct GroceryItem {
        quantity: i32,
        id:i32,
    }

    fn display_quantity(item: &GroceryItem) {
        println!("Quantity : {:?}",item.quantity);
    }
    fn display_id(item: &GroceryItem) {
        println!("Id : {:?}",item.id);
    }

    let my_item = GroceryItem {
        quantity: 2,
        id: 99,
    };

    display_quantity(&my_item);
    display_id(&my_item);
    


}
