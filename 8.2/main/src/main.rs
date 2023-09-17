pub mod plant_structures;

fn main() {
    println!("Hello, world!");

    plant_structures::roots::reftest();
    //::plant_structures::roots::reftest(); failed to resolve: could not find `plant_structures` in the list of imported crates
}
