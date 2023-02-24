pub mod gen {
    tonic::include_proto!("serdict");
}
pub use gen::*;

pub mod client;

pub const SERVICE_GROUP: &str = "platform";
pub const SERVICE_NAME: &str = "service_discovery";
