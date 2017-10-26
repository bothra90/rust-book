mod mybox;
mod coercion;

pub fn run() {
    // Test custom deref impl with MyBox.
    mybox::run();
    // Test deref coercion with MyBox.
    coercion::run();
}
