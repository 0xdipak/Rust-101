mod db;
use db::*;

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    // making connection to the database
    let conn = init_database().expect("Failed to initialize the database");
    clr();
    let ascii = r#"
    
__________                        ____   ____            .__   __   
\______   \_____    ______ ______ \   \ /   /____   __ __|  |_/  |_ 
 |     ___/\__  \  /  ___//  ___/  \   Y   /\__  \ |  |  \  |\   __\
 |    |     / __ \_\___ \ \___ \    \     /  / __ \|  |  /  |_|  |  
 |____|    (____  /____  >____  >    \___/  (____  /____/|____/__|  
                \/     \/     \/                 \/                 

    "#;

    println!("{ascii}");

    loop {
        println!("Password manager menu: ");
        println!("1. Add Entry ");
        println!("2. List Entry ");
        println!("3. Search Entry ");
        println!("4. Quit ");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service :"),
                    prompt("Username :"),
                    prompt("Password :"),
                );
                write_password_to_db(&conn, &entry.service, &entry.username, &entry.password)
                    .expect("Failed to write to the database");
                println!("Entry added successfullt");
            }
            "2" => {
                clr();
                let services = read_passwords_from_db(&conn).unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });

                for item in &services {
                    println!(
                        "Service = {}
                        - Username : {}
                        - Password : {}
                        ",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                let search = prompt("Search by service name: ");
                match search_service_by_name(&conn, &search) {
                    Ok(Some(entry)) => {
                        println!(
                            "Service = {}
                        - Username : {}
                        - Password : {:?}
                        ",
                            entry.service, entry.username, entry.password
                        );
                    }
                    Ok(None) => {
                        println!("Service not found.");
                    }
                    Err(err) => {
                        eprintln!("Error searching for services : {}", err);
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choive."),
        }
        println!("\n\n");
    }
}
