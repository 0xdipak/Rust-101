

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>
}

fn main() {
    let response = Survey {
        q1: Some(12),
        q2: Some(true),
        q3:Some(("A".to_owned()))
    };

    match response.q1 {
        Some(ans) => println!("q1: {:?}", ans),
        None => println!("q1: no response")
    }



    // Activity
    struct Student {
        name: String,
        locker: Option<i32>
    }

    let marry = Student {
        name: "Marry".to_owned(),
        locker: Some(3),
    };


    println!("student: {:?}", marry.name);
    match marry.locker {
        Some(num) => println!("locker number : {:?}", num),
        None => println!("No locker assigned"),
    }
}
