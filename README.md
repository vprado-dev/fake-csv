# Create csv with fake data with Rust
## To run:
```bash
git clone
cargo run src
```

## If you want to change or add some data
### Change this struct
```rust
struct Person {
  //Add or remove fields
  nome: String,
  email: String,
}
```

### Change this for loop
```rust
for _ in 0..10000 {
  writer.serialize(Person {
    //Add or remove the fields that you specied in previous struct
    nome: Name(EN).fake(),
    email: SafeEmail(EN).fake(),
  })?;
}
```
