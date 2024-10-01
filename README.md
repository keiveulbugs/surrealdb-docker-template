A simple template in Rust for SurrealDB that allows for switching between type of database used with the help of feature flags.

```
// Does not run a database
cargo run

// Connects to a remote database
cargo run --features database

// Connects to a memory database
cargo run --features memdatabase

// Connects to a file database
cargo run --features filedatabase

```