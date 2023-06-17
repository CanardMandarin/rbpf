#[macro_use]
extern crate afl;

use solana_rbpf::elf::{Executable};
use solana_rbpf::vm::{BuiltinProgram, Config, TestContextObject};
use solana_rbpf::verifier::{TautologyVerifier};
use std::{
    sync::Arc,
};

fn main() {
    fuzz!(|data: &[u8]| {
        let loader = BuiltinProgram::<TestContextObject>::new_loader(Config::default());
        let t = Executable::<TautologyVerifier, TestContextObject>::load(&data.to_vec(), Arc::new(loader));
    });
}
