# General Cipher Solver
Applying the genetic algorithm to solve any cipher<sup>[1](#footnote1)</sup>.

 <sub><a id="footnote1">1</a>:See 'Limitations'</sub>

Instead of having to write a whole solver for a cipher, you only have to write functions to:
- [ ] randomly initialise a key
- [ ] decode the text given the key
- [ ] randomly mutate the key
- [ ] crossover two keys (optional but improves performance)

Depending on your use case, these functions may have already been written for you<sup>[2](#footnote2)</sup>!

 <sub><a id="footnote2">2</a>:See 'Utils'</sub>

## Usage
- [install Rust](https://www.rust-lang.org/)
- clone the repo: ```git clone https://github.com/all-c-a-p-s/General-Cipher-Solver```
- create new file corresponding to the cipher you want, and add ```pub mod file_name;``` to main.rs
- update function call in main.rs (remember to set generic parameter appropriately depending on crossover)
- optionally change GA parameters

Once this is done, sit back and watch the [BLAZINGLY FAST](https://programmerhumor.io/rust-memes/rust-is-blazingly-fast-and-we-wont-shut-up-about-it-f24q) parallelised solver 🚀🚀🚀

## Utils
- The keys of many ciphers can be represented using arrays of the form ```[T; N]```
- For these cases, you can find functions to randomise, crossover, and mutate such arrays

## Examples
Some examples (all well under 100 lines of code) can be found for:
- a simple monoalphabetic substitution cipher
- the four-square cipher, a bigram substitution cipher with a larger key
- the columnar transposition cipher<sup>[1](#footnote1)</sup>

## Limitations
There are some limitations of this project, which may or may not be adressed in the future:
- You first need to identify the cipher used. Some good advice for this can be found [here](https://github.com/themaddoctor/BritishNationalCipherChallenge/tree/master/guides).
- Even after identifying the cipher, in some cases, such as the columnar transposition cipher, you may still have to do some extra work such as determining the key length.
- Ciphers with very large keys (e.g. over 100 elements) may be impractical to solve.
- Compound ciphers may be impractical to solve.
- The genetic algorithm is not well suited to 'chaotic' ciphers (ciphers in which similar keys do not produce similar plaintexts).
