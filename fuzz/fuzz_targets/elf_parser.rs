#![no_main]

use libfuzzer_sys::fuzz_target;
use solana_rbpf::elf::{Executable};
use solana_rbpf::vm::{BuiltinProgram, Config, TestContextObject};
use solana_rbpf::verifier::{TautologyVerifier};
use std::{
    sync::Arc,
};
#[derive(arbitrary::Arbitrary, Debug)]
struct FuzzData {
    elf: Vec<u8>,
}

fuzz_target!(|data: FuzzData| {
    let loader = BuiltinProgram::<TestContextObject>::new_loader(Config::default());
    let t = Executable::<TautologyVerifier, TestContextObject>::load(&data.elf, Arc::new(loader));
    
});
