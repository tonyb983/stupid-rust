mod v1;
mod v2;

pub use v1::*;

pub mod rpc {
    include!(concat!(env!("OUT_DIR"), "/sdb.rpc.rs"));
}
