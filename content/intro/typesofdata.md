# Types of data structures

Data structures can be differentiated and defined in several types. The types of data to be computed vary according to the programming language to be used, so we will also introduce the data types of the Rust language.

## Rust data types

You can check more detailed documentation by clicking [here](https://doc.rust-lang.org/book/ch03-02-data-types.html).

### Integer

Integers in Rust can have subtypes depending on their uses. Integers are numerical values that do not necessarily have floating points to match their decimal value.

| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

### Floating-Point

Numbers with values in decimal places, which can be defined in 32-bit or 64-bit sets, with declarations such as f32 and f64 respectively.

### Boolean

It only stores two types of value: true or false.

### Character

Used to store a pure character, being the primitive type that will store any type of data in a simple way.

### Other

There are still types for arrays and tuples, which can be seen in the documentation. Both won't be introduced yet, but be aware that they are also recognized as types.

## Data structures

Now let's see a little about the main types of data structures.

### Linear

We can define the type of linear data structure as the one that will store the data to be accessed by values in indexes and linked lists. An obvious example would be arrays.

### Tree

In this type of data structure we will have a root that will be responsible for enabling all access to the complete data structure through linked nodes. There are several types of trees, such as binary trees and red-black trees.

### Hash tables

The use of a table to map keys with values is a classic feature of hash tables, and may include concepts of separate chaining and linear probing.

### Graph

The applicability of principles on graph theory led to this data structure, using nodes and edges to represent directional flows of networks.

You can read a little more about graph theory by clicking [here](https://en.wikipedia.org/wiki/Graph_theory).

---

Are you ready to dive a little deeper into the main concepts involving linear data structures?

<p align="right">
  <a href="https://github.com/lanjoni/rust-data-structure/tree/main/content/linear">Next -> Linear data structures</a>
</p>

<p align="left">
  <a href="https://github.com/lanjoni/rust-data-structure#roadmap">Back to main menu</a>
</p>
