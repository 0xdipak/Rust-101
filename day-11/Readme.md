Memory Intermediate
- Memory is stored using binary
    - Bits : 0 or 1
- Computer optimized for bytes
    - 1 byte == 8 contiguous bits
- Fully contiguous

Addresses
- All data in memory has an "address"
    - Used to locate data
    - Always the same - only data changes
- Usually don't utilize addresses directly
    - Varibales handle most of the work

Offsets
- Items can be located at an address using an "offset"
- Offsets begin at 0;
- Represent the number of butes away from the original address
    - Normally deal with indexes instead



---- Recap ----
- Memory usees addresses & offsets
- Adddresses are permanent, data differs
- Offsets can be used to "index" into some data


Ownership
- Programs must track memory
    - If they fail to do so, a "leak" occurs
- Rust utilizes an "ownership" model to manage memory
    - The "owner" of memory is responsible for cleaning up the memory
- Memory can either be "moved" or "borrowed"

- Memory must be managed in some way to prevent leaks
- Rust uses "ownership" to accomplish memory management
    - The "owner" of data must clean up the memory
    - This occurs automatically at the end of the scope
- Default behaviour is to "move" memory to a new owner
    - Use an ampersand(&) to allow code to "borrow" memory