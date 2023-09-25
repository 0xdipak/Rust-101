Expressions

- Rust is an expression-based language
    - Most things are evaluated and return some value
- Expression values coalesce to a single point
    - Can be used for nesting logic
- it allows nested logic
- if and match expressions can be nested
    - Best to not use more than two ar three levels



- With if-else
```
let my_num = 3;
let is_it_5 = if my_num < 5 {
    true
}else {
    false
}

or
let is_it_5 = my_num < 5; // same result either true or false
```


- With match
```
let my_num = 3;
let message = match my_num {
    1 => "hello",
    _ => "goodbye",
}
```