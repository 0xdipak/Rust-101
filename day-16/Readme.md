Option
- A type that may be one of two things
    - Some data of a specified type
    - Nothing
- Used in scenarios where data may not be required or is unavailable
    - Unable to find something
    - Ran out of items in a list
    - Form field not filled out


    ```
    enum Option<T> {
        Some(T),
        None
    }
    ```

    ```
    struct Customer {
        age: Option<i32>,
        email: String
    }

    let mark = Customer {
        age: Some(22), email: "abc@gmail.com".to_owned(),
    };
    let becky = Customer {
        age: None, email: "xyz@gmail.com".to_owned(),
    };

    match becky.age {
        Some(age) => println!("customer age is{:?} years old", age),
        None => println!("customer age not provided"),
    }
    ```
    
- Option represents either some data or nothing
    - Some(variable_name)
        - Data is available
    - None
        - No data is available
- Useful when needing to work with optional data
- Use Option<type> to declare an optional type