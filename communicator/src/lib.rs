pub mod network {
    // Modules can be nested inside other modules
    pub mod client {
        pub fn connect() {}
    }
    pub mod server {
        pub fn connect() {}
    }
    pub fn connect() {}
}

// Multiple modules can be defined in the same lib.rs
pub mod client {
    pub fn connect() {}
}

// module code can also be in a <name> direcotry with a mod.rs file.
pub mod networksep;


//  Modules can be defined here, but the code lies in another file.
pub mod netclient;

#[cfg(test)]
mod tests;
