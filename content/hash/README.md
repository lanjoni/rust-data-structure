# Hash Tables

Hash tables are data structures that encapsulate a large group of data in a smaller but keyable data structure. Intuitively, think of hash maps as a collection of data that can be indexed based on the data structure you choose to hold it. In this example, we'll be using arrays, but you could use anything that is abstractly contiguous and indexable, as said beforewards.

## Abstraction

Hash tables are in essence, stored by a hash algorithm. Said so, we won't be able to retrieve a data by index like we would with an array. We still could, but since we aren't necessary storing every data contiguously, you may get something completely different in return.

To make this more simple, we have some data, make it go through a transformation, usually called "Hashing", and them store it based on a value calculated through it's hash value, which is usually a value used by the data structure that holds this hash table, like an index for an array, or a pointer for a linked list.

## Hashing

There are ton of hashing algorithms, but, for sake of simplicity, we'll be using a simple algorithm that sums the binary values of the data being stored, and then "mods" ( % ) the result to the max size of the structure holding it.

For example, if we try to hash ```"hello"``` inside a hash table with capacity of 20 slots:

```rust
const HS_CAP: i32 = 20;

fn main() {
    let data: String = String::from("hello");
    
    let sum: i32 = data
     .as_bytes()
     .iter()
     .sum();
     
    sum %= HS_CAP;

    println!("{}", sum);
}
```
