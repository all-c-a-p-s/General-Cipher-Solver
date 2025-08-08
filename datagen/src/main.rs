pub mod ciphers;
pub mod datagen;
pub mod features;

fn main() {
    let generated = datagen::generate_once();
    println!("{:?}", generated);
}
