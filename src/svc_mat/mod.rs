pub mod gen {
    tonic::include_proto!("math");
}

/// Entrypoint service - parses and evaluates math expressions in a distributed manner
pub mod calc;

pub mod add;
pub mod div;
pub mod mul;
pub mod sub;

pub const SERVICE_GROUP: &str = "math";
