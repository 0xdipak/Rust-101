Vectors
- The vec! macro can be used to make vectors
- Multiple pieces of data
    - Must be the same type
- Used for lists of information
- Can add, remove, and traverse the entries
Use for..in to iterate through items fo a vector
  E.g :

let my_numbers = vec![1,2,3];

OR

let mut my_numbers = Vec::new();
my_numbers.push(1);
my_numbers.push(2);
my_numbers.push(3);
my_numbers.pop();
my_numbers.length(); // this is 2


let two = my_numbers[1];


Strings
- Two commonly used types of strings
  - String - owned
  - &str - borrowes String slice
- Must use an owned string to store in a struct
- Use &str when passing to a function
- Strings are automatically borrowed
- Use .to_owned() or String::from() to create owned copy of an string slice
- Use an owned string when storing in a struct

E.g :
fn print_it(data: &str) {
  println!("{:?}", data)
}

fn main {
  print_it("a string slice");
  let owned_string = "owner string".to_owned();
  let another_owned = String::from("another");
  print_it(&owned_string);
  print_it(&another_owned);
}