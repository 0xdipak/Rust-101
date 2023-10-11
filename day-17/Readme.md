Result
- A data type that contains one of two types of data 
    - "Successful" data
    - "Error" data
- Used in scenarios where an action needs to be taken, but has the possibility of failure
    - Copyting a file
    - Connecting to a website

```
enum Result<T, E> {
    ok(T),
    Err(E)
}
```
