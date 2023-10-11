#[derive(Debug)]
enum MenuChoics {
    MainMenu,
    Start,
    Quit
}

fn get_choice(input: &str) -> Result<MenuChoics, String> {
    match input {
        "mainmenu" => Ok(MenuChoics::MainMenu),
        "start" => Ok(MenuChoics::Start),
        "quit" => Ok(MenuChoics::Quit),
        _ => Err("menu choice not found".to_owned()),
    }
}

fn main() {
    let choice = get_choice("mainmenu");
    println!("choice: {:?}", choice);



    // Activity 
    #[derive(Debug)]
    struct Adult {
        age: u8,
        name: String
    }

    impl Adult {
        fn new(age:u8, name: &str) -> Result<Self, &str> {
            if age >= 21 {
                Ok(Self {
                    age,
                    name: name.to_string(),
                })
            } else {
                Err("Age must be at least 21")
            }
        }
    }


    let child = Adult::new(15, "Anita");
    let adult = Adult::new(21, "Sanjay");

    match child {
        Ok(child) => println!("{} is {} years old", child.name, child.age),
        Err(e) => println!("{e}")
    }

    match adult {
        Ok(adult) => println!("{} is {} years old", adult.name, adult.age),
        Err(e) => println!("{e}")
    }

}