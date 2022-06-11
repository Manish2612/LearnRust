# Rust + Diesel + PostgreSQL

### To generate Model file for DB Table:
        cargo install diesel_cli_ext
        diesel_ext --model > src/models.rs

### Using let and const interchangeably
Using let and const interchangeably can lead to error. Try changing to other if unexpected error thrown 
eg. using non-constant variable and try to store resulting value in "const" variable will throw error

