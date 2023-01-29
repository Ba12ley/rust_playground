struct Vehicle {
    name: String,
    max_speed: u32,
}

trait HasEngine {
    fn has_engine(&self) -> bool;
    fn start(&self);
    fn stop(&self);
}

impl HasEngine for Vehicle {
    fn has_engine(&self) -> bool {
        true
    }
    fn start(&self) {
        println!("Starting {}", self.name);
    }

    fn stop(&self) {
        println!("Stopping {}", self.name);
    }
}