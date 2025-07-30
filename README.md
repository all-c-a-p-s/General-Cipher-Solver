# General Cipher Solver
Applying the genetic algorithm to solve any cipher. 

Instead of having to write a whole solver for a cipher, you only have to write functions to:
- [ ] randomly initialise a key
- [ ] decode the text given the key
- [ ] randomly mutate the key
- [ ] crossover two keys (optional but improves performance)

# Usage
- [install Rust](https://www.rust-lang.org/)
- clone the repo: ```git clone https://github.com/all-c-a-p-s/General-Cipher-Solver```
- create new file corresponding to the cipher you want, and add ```pub mod file_name;``` to main.rs
- update function call in main.rs (remember to set generic parameter appropriately depending on crossover)
- optionally change GA parameters

Once this is done, sit back and watch the [BLAZINGLY FAST](https://programmerhumor.io/rust-memes/rust-is-blazingly-fast-and-we-wont-shut-up-about-it-f24q) parallelised solver 🚀🚀🚀
