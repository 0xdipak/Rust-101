Match
- Add logic to program
- Similar to if-else
- Exhaustive
    - All options muct be accounted for


Match VS else-if
- match will be checked by the compiler
    - If a new possibility is added, you will be nitified when this occurs
- else-if is not checked by the compiler
    - if a new possibility is added, your code may contain a bug

- Prefer match over else-if when working with single varuable
- match considers all possibilities
    - more robust code
- use underscore( _ ) to match "anything else"