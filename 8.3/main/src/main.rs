pub mod plant_structures;

fn main() {
    println!("Hello, world!");

    plant_structures::roots::reftest();

    let mut f = plant_structures::roots::Fern{size: 1.0, growth_rate: 1.0};

    plant_structures::roots::run_simlation(&mut f,  10);

    println!("{}", f.size);

    //::plant_structures::roots::reftest(); failed to resolve: could not find `plant_structures` in the list of imported crates
}
