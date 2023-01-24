pub mod gen {
    tonic::include_proto!("math");
}

pub mod add;
pub mod div;
pub mod mul;
pub mod sub;

pub const SERVICE_GROUP: &str = "math";