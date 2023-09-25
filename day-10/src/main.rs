
enum Access {
    Admin,
    Manager,
    User,
    Guest
}



fn main() {
    // secret file : admins only
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
    println!("Can access {:?}", can_access_file);


    let value = 100;
    let is_gt_100 = value > 100;
    
    fn print_msg(gt_100:bool) {
        match gt_100 {
            true => println!("it's big"),
            false => println!("it's small")
        }
    }

    print_msg(is_gt_100);
}
