Hashmap
- Collection that stores data as key-value pairs
    - Data is located using the "key"
    - The data is the "value"
- Similar to definitions in a dictionary
- Very fast to retrieve data using the key
- Very fast to insert & find data using the key
- Useful when you need to find information and know exactly where it is (via the key)

```
let mut people = HashMap::new();
people.insert("Susan", 21);
people.insert("Ed", 13);
people.remove("Susan")

match people.get("Ed") {
    Some(age) => println!("age = {:?}", age),
    None => println!("not found"),
}
```