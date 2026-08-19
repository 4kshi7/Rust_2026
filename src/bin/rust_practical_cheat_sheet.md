# Rust Practical Cheat Sheet

A compact reference based on the Rust concepts, functions, data structures, and patterns covered so far.

---

## 1. Core Data Types

### Common types

```rust
let name: String = "alice".to_string();
let name_ref: &str = "alice";
let age: u32 = 25;
let average: f32 = 28.2;
let index: usize = 0;
let active: bool = true;
```

### Important distinction

- `String` = owned, growable string.
- `&str` = borrowed string slice.
- `u32` = unsigned integer.
- `f32` = floating-point number.
- `usize` = commonly used for collection indexes.
- `Option<T>` = a value may or may not exist.
- `Result<T, E>` = an operation can succeed or fail.

---

# 2. `String` and `&str`

## Convert `&str` to `String`

```rust
let name = "alice";
let owned_name = name.to_string();
```

Also commonly:

```rust
let owned_name = String::from("alice");
```

### Remember

```text
&str
 ↓ to_string()
String
```

`String` owns its data.

`&str` borrows data.

---

# 3. Structs

Used to represent related data.

```rust
#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
    city: String,
}
```

Create a struct:

```rust
let user = User {
    name: "alice".to_string(),
    age: 25,
    city: "Delhi".to_string(),
};
```

Access fields:

```rust
println!("{}", user.name);
println!("{}", user.age);
```

## `Debug`

Needed for:

```rust
println!("{:?}", user);
println!("{:#?}", user);
```

Add:

```rust
#[derive(Debug)]
```

## `Clone`

Allows:

```rust
let copy = user.clone();
```

Add:

```rust
#[derive(Clone)]
```

Can combine:

```rust
#[derive(Debug, Clone)]
```

---

# 4. `Vec<T>`

A growable collection.

```rust
let mut users: Vec<User> = Vec::new();
```

## Add an item

```rust
users.push(user);
```

## Length

```rust
users.len()
```

Returns `usize`.

## Indexing

```rust
let user = &users[0];
```

Be careful: indexing can panic if the index doesn't exist.

## Iterate without consuming

```rust
for user in &users {
    println!("{}", user.name);
}
```

Equivalent common form:

```rust
for user in users.iter() {
    println!("{}", user.name);
}
```

## Iterate with index

```rust
for (index, user) in users.iter().enumerate() {
    println!("{}: {}", index, user.name);
}
```

`index` is a `usize`.

### Mental model

```text
Vec
 |
 +-- index 0
 +-- index 1
 +-- index 2
```

---

# 5. Iterators

## `.iter()`

Borrows elements.

```rust
for item in items.iter() {
}
```

Usually you'll also see:

```rust
for item in &items {
}
```

Both are useful when you don't want to consume the collection.

## `.enumerate()`

Adds indexes:

```rust
for (i, item) in items.iter().enumerate() {
}
```

Result conceptually:

```text
0 -> first item
1 -> second item
2 -> third item
```

---

# 6. String Parsing

A very common practical pattern:

```rust
for line in data.lines() {
    // process each line
}
```

## `.lines()`

```rust
let data = "alice,25,Delhi
bob,31,Mumbai";

for line in data.lines() {
    println!("{}", line);
}
```

Produces:

```text
alice,25,Delhi
bob,31,Mumbai
```

---

# 7. `.split()`

Split a string into pieces.

```rust
let mut values = line.split(',');
```

For:

```text
alice,25,Delhi
```

you conceptually get:

```text
alice
25
Delhi
```

Important:

```rust
split(',')
```

returns an iterator.

It does NOT return a `Result` or `Option`.

---

# 8. `.next()`

Get the next item from an iterator.

```rust
let value = values.next();
```

Return type:

```rust
Option<&str>
```

Possible results:

```rust
Some("alice")
```

or:

```rust
None
```

## Safe extraction with `match`

```rust
let name = match values.next() {
    Some(value) => value,
    None => return Err("Missing name".to_string()),
};
```

### Remember

```text
iterator.next()
       ↓
Option<T>
       ↓
Some(value) / None
```

---

# 9. `.split_once()`

Useful when you want exactly two portions.

```rust
let (severity, message) = line.split_once(' ').unwrap();
```

For:

```text
ERROR Database connection failed
```

you get:

```text
severity = "ERROR"
message  = "Database connection failed"
```

Unlike `split_whitespace()`, this preserves the rest of the message as one piece.

---

# 10. `.parse()`

Convert text into another type.

```rust
let age: u32 = "25".parse().unwrap();
```

But `parse()` returns a `Result`.

```text
"25".parse()
     ↓
Ok(25)

"abc".parse()
       ↓
Err(...)
```

## Safe parsing

```rust
let age: u32 = match age_str.parse() {
    Ok(value) => value,
    Err(_) => return Err("Invalid age".to_string()),
};
```

### Important

```text
next()
  → Option

parse()
  → Result
```

This distinction is extremely important in Rust.

---

# 11. `Option<T>`

Represents:

> A value might exist.

```rust
let value: Option<u32> = Some(25);
```

No value:

```rust
let value: Option<u32> = None;
```

## `Some`

```rust
Some(value)
```

means a value exists.

## `None`

```rust
None
```

means no value exists.

## `match`

```rust
match value {
    Some(x) => println!("{}", x),
    None => println!("Nothing"),
}
```

## `.unwrap()`

```rust
let x = value.unwrap();
```

Extracts the value.

But it PANICS if the value is `None`.

Use carefully.

---

# 12. `Result<T, E>`

Represents:

> An operation succeeded or failed.

Example:

```rust
fn parse_age(value: &str) -> Result<u32, String> {
    match value.parse() {
        Ok(age) => Ok(age),
        Err(_) => Err("Invalid age".to_string()),
    }
}
```

Possible results:

```rust
Ok(25)
```

or:

```rust
Err("Invalid age".to_string())
```

## `Ok`

Success:

```rust
Ok(value)
```

## `Err`

Failure:

```rust
Err(error)
```

## Function returning `Result`

```rust
fn analyze(data: &str) -> Result<Vec<User>, String> {
    // ...
}
```

On success:

```rust
Ok(users)
```

On failure:

```rust
Err("Invalid input".to_string())
```

---

# 13. `match`

One of the most useful Rust tools.

General pattern:

```rust
match value {
    Some(x) => {
        // success
    }

    None => {
        // missing
    }
}
```

For `Result`:

```rust
match result {
    Ok(value) => {
        // success
    }

    Err(error) => {
        // failure
    }
}
```

---

# 14. `HashMap`

Import:

```rust
use std::collections::HashMap;
```

Create:

```rust
let mut map: HashMap<String, u32> = HashMap::new();
```

Conceptually:

```text
key          value
-------------------
"/users"       5
"/products"    3
```

---

# 15. `HashMap::entry()`

One of the most useful patterns you learned.

```rust
*map.entry(key).or_insert(0) += 1;
```

Meaning:

```text
Does key exist?
      |
   /     \
 yes      no
 |         |
existing   insert 0
value      |
 |          |
 +----+-----+
      |
    + 1
```

Example:

```rust
let mut freq: HashMap<String, u32> = HashMap::new();

for word in words {
    *freq.entry(word.to_string()).or_insert(0) += 1;
}
```

This is excellent for:

- frequency counting
- endpoint counts
- category totals
- grouped statistics

---

# 16. Fetching from a HashMap with `.get()`

```rust
let value = map.get("users");
```

Return type:

```rust
Option<&V>
```

For example:

```rust
HashMap<String, u32>
```

then:

```rust
map.get("users")
```

returns:

```rust
Option<&u32>
```

Possible:

```text
Some(&5)
```

or:

```text
None
```

## Extracting when you know the key exists

```rust
let count = *map.get("users").unwrap();
```

Now:

```text
count → u32
```

Remember the `*` because `.get()` gives you a reference.

---

# 17. Iterating over a HashMap

```rust
for (key, value) in &map {
    println!("{} -> {}", key, value);
}
```

Conceptually:

```text
key   → &String
value → &u32
```

This pattern is especially useful for finding:

- maximum
- minimum
- busiest category
- highest-frequency item

---

# 18. Using One HashMap's Key in Another

A pattern you practiced:

```rust
for (endpoint, count) in &endpoint_count {
    let total = *total_response.get(endpoint).unwrap();

    let average = total as f32 / *count as f32;
}
```

Mental model:

```text
Map A
/users → 5
   |
   | use "/users" as key
   ↓
Map B
/users → 1270
```

This is a very useful pattern for grouped data.

---

# 19. `String` Keys vs `&str` Keys

You encountered a borrowing issue here.

### Owned keys

```rust
HashMap<String, u32>
```

The HashMap owns its keys.

Often easiest for beginners.

Example:

```rust
map.entry(data.path.clone())
```

### Borrowed keys

```rust
HashMap<&str, u32>
```

The HashMap borrows string data owned somewhere else.

This introduces lifetime considerations.

For now:

> Prefer `String` keys when ownership gets complicated.

You can optimize toward borrowed keys later.

---

# 20. Ownership and Borrowing

### Owned value

```rust
let name: String = "alice".to_string();
```

`name` owns the string.

### Borrow

```rust
let name_ref: &str = &name;
```

`name_ref` does not own it.

Think:

```text
String
  owns data
     ↑
     |
    &str
  borrows data
```

## Borrowing a Vec

```rust
for user in &users {
}
```

You are borrowing the users.

The vector remains available afterward.

---

# 21. `.clone()`

Sometimes you need an owned copy.

```rust
let copy = user.clone();
```

For your structs:

```rust
#[derive(Clone)]
struct User {
    ...
}
```

You used this when finding the oldest/slowest item:

```rust
let slowest_request = request_vec[index].clone();
```

Why?

Because the vector owns the `Request`, and you wanted an independent `Request` to return.

Don't avoid `clone()` blindly while learning. First understand ownership; optimize later.

---

# 22. Numeric Conversions

You encountered an important issue with averages.

This:

```rust
let average: f32 = total / count;
```

doesn't work if both are integers.

Use:

```rust
let average = total as f32 / count as f32;
```

Important:

```text
u32 / u32
   ↓
integer division

as f32 BEFORE division
   ↓
floating-point division
```

Example:

```rust
let average = 141 as f32 / 5 as f32;
```

gives:

```text
28.2
```

not:

```text
28.0
```

---

# 23. Finding a Maximum

A pattern you used repeatedly:

```rust
let mut highest: u32 = 0;
let mut highest_index: usize = 0;

for (i, item) in items.iter().enumerate() {
    if item.value > highest {
        highest = item.value;
        highest_index = i;
    }
}
```

Then:

```rust
let highest_item = items[highest_index].clone();
```

Useful for:

- highest-value product
- oldest user
- slowest request
- largest expense

---

# 24. Finding a Maximum in a HashMap

Same idea, but use the key instead of an index:

```rust
let mut highest_count: u32 = 0;
let mut busiest: String = String::new();

for (endpoint, count) in &endpoint_count {
    if *count > highest_count {
        highest_count = *count;
        busiest = endpoint.clone();
    }
}
```

Remember:

```text
Vec → usually track index

HashMap → usually track key
```

---

# 25. Common Practical Parsing Pattern

For CSV-like data:

```rust
for line in data.lines() {
    let mut values = line.split(',');

    let name = match values.next() {
        Some(value) => value,
        None => return Err("Missing name".to_string()),
    };

    let age_str = match values.next() {
        Some(value) => value,
        None => return Err("Missing age".to_string()),
    };

    let age: u32 = match age_str.parse() {
        Ok(value) => value,
        Err(_) => return Err("Invalid age".to_string()),
    };
}
```

This pattern is worth remembering.

---

# 26. Common Data-Processing Pipeline

A lot of the practical problems we solved followed this architecture:

```text
Raw &str
   ↓
lines()
   ↓
split(',')
   ↓
next()
   ↓
parse()
   ↓
struct
   ↓
Vec<Struct>
   ↓
analysis
   ↓
HashMap / aggregation
   ↓
Result
```

When facing a new practical problem, ask:

1. What does one input line represent?
2. Should I create a struct?
3. What collection should store the records?
4. What values need aggregation?
5. Do I need a HashMap?
6. What can fail during parsing?
7. What should the function return?

---

# 27. Common Mistakes You Made — Keep These in Mind

### Mistake: `split()` + `unwrap()`

Wrong idea:

```rust
line.split(',').unwrap()
```

`split()` returns an iterator.

Correct:

```rust
let mut values = line.split(',');
```

Then:

```rust
values.next()
```

---

### Mistake: forgetting `()` on methods

Wrong:

```rust
values.next.unwrap()
```

Correct:

```rust
values.next().unwrap()
```

---

### Mistake: integer division before conversion

Wrong:

```rust
(total / count) as f32
```

Correct:

```rust
total as f32 / count as f32
```

---

### Mistake: confusing `Option` and `Result`

Remember:

```text
Option
Some / None
```

usually means:

> Value may not exist.

```text
Result
Ok / Err
```

usually means:

> Operation succeeded or failed.

---

### Mistake: `HashMap::get()` isn't the value itself

```rust
map.get(key)
```

returns:

```rust
Option<&V>
```

not:

```rust
V
```

---

### Mistake: wrong metric

Always ask:

> What exactly am I measuring?

For example:

```text
busiest endpoint
    ≠
highest average latency
```

Busiest means highest request count.

---

### Mistake: unnecessary nested loops

If you already have:

```rust
for request in &requests {
}
```

don't automatically create another loop over the same requests.

Often multiple aggregations can happen in the same loop:

```rust
for request in &requests {
    count_map...
    total_map...
}
```

---

# 28. Useful Method Reference

| Method | What it does |
|---|---|
| `.lines()` | Iterate over lines |
| `.split(',')` | Split into pieces |
| `.split_whitespace()` | Split on whitespace |
| `.split_once(',')` | Split into two parts |
| `.next()` | Get next iterator item |
| `.parse()` | Convert string to type |
| `.to_string()` | Create owned String |
| `.as_str()` | Borrow String as `&str` |
| `.push()` | Add item to Vec |
| `.len()` | Get collection length |
| `.iter()` | Borrow/iterate collection |
| `.enumerate()` | Add index to iterator |
| `.entry()` | Access/create HashMap entry |
| `.or_insert()` | Insert default if key missing |
| `.get()` | Look up HashMap value |
| `.clone()` | Create owned copy |
| `.contains()` | Check range/string/collection depending on type |

---

# 29. Useful Patterns

## Frequency counting

```rust
*map.entry(key).or_insert(0) += 1;
```

## Sum grouped by key

```rust
*map.entry(key).or_insert(0) += amount;
```

## Average grouped by key

```rust
let total = *totals.get(key).unwrap();
let count = *counts.get(key).unwrap();

let average = total as f32 / count as f32;
```

## Find maximum in Vec

```rust
let mut max = 0;
let mut max_index = 0;

for (i, item) in items.iter().enumerate() {
    if item.value > max {
        max = item.value;
        max_index = i;
    }
}
```

## Find maximum in HashMap

```rust
let mut max = 0;
let mut max_key = String::new();

for (key, value) in &map {
    if *value > max {
        max = *value;
        max_key = key.clone();
    }
}
```

---

# 30. `Result` Evolution

You currently know the explicit style:

```rust
let value = match something {
    Ok(value) => value,
    Err(_) => return Err("Invalid value".to_string()),
};
```

This is excellent for learning because you can clearly see the control flow.

Later, you'll learn the `?` operator:

```rust
let value = something?;
```

It essentially lets errors propagate automatically.

Don't rush this. Understand the explicit `match` version first.

---

# 31. Empty Input

Be careful with code like:

```rust
items[0]
```

If the collection is empty, this panics.

Likewise:

```rust
map.get(key).unwrap()
```

can panic if the key doesn't exist.

And:

```rust
total as f32 / count as f32
```

can divide by zero if `count == 0`.

Before production-quality code, think about:

```text
What if input is empty?
What if a field is missing?
What if parsing fails?
What if the key doesn't exist?
What if there are duplicate records?
```

---

# 32. Practical Problem-Solving Checklist

When given a new Rust data-processing problem:

```text
1. Identify the input format
2. Decide whether a struct represents one record
3. Parse each line
4. Handle Option/Result safely
5. Store records in Vec
6. Identify aggregations
7. Use HashMap when grouping by a key
8. Use entry().or_insert() for counting/summing
9. Use get() when retrieving a value by key
10. Use iter()/enumerate() for scanning a Vec
11. Track index for max item in Vec
12. Track key for max item in HashMap
13. Check empty-input edge cases
14. Decide what the function should return
```

---

# 33. Concepts We Have NOT Covered Deeply Yet

These are good future topics:

```text
Ownership
Borrowing
Lifetimes
?
Iterators in depth
Option combinators
Result combinators
Struct methods / impl
Enums
Pattern matching
Traits
Generics
Closures
String slices in depth
Error types
File I/O
Command-line applications
Modules
Testing
```

Don't try to learn all of them at once.

Your current foundation is:

```text
struct
Vec
HashMap
Option
Result
match
iterators
parsing
ownership basics
borrowing basics
```

That is enough to start solving increasingly realistic Rust problems.

---

# 34. Current Learning Strategy

For practical exercises, try to solve in this order:

```text
Input
 ↓
What is one record?
 ↓
Create struct
 ↓
Parse safely
 ↓
Vec
 ↓
What needs grouping?
 ↓
HashMap
 ↓
What needs maximum/minimum?
 ↓
Scan Vec or HashMap
 ↓
Return Result
```

Don't optimize prematurely.

First make it:

```text
correct
 ↓
understandable
 ↓
idiomatic
 ↓
optimized
```

Understanding ownership and correctness is more important than eliminating every `.clone()` at this stage.
