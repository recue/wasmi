#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate recue_wasmi;

fuzz_target!(|data: &[u8]| {
  // Just check if loading some arbitrary buffer doesn't panic.
  let _ = recue_wasmi::Module::from_buffer(data);
});
