# LearnRust

1. To generate Model file:
        1. cargo install diesel_cli_ext
        2. diesel_ext --model > src/models.rs

2. Using let and const interchangeably can lead to error. Try changing to other if unexpected error thrown 
    eg. using non-constant variable and try to store update resulting value in "const" variable

