#[macro_use]
extern crate error_chain;
extern crate url;

mod errors;
pub use errors::*;

pub fn write_file() -> std::io::Result<()> {
    loop {}
}

pub fn read_file() -> Result<String> {
    loop {}
}
