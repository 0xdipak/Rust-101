Enumerations 
- Data that can be one of multiple different possibilities
    - Each possibility is called a "varient"
- provides information about your program to the compiler
    - More robust programs

- eg. :
enum Direction {
    Up,
    Down,
    Left,
    Right
   }

   fn which_way(go:Direction) {
    match go {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }

}
- Make program code easier to read


Struct 
- A type contains multiple pieces of data 
    - All or nothing - can not have some pieces of data and not others
- Each piece of data is called a "field"
- Makes working with data easier
    - Similar data can be grouped together
- All field must be present to create a struct
- Fields can be accessed using a dot.