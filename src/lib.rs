#![deny(clippy::all)]
#![allow(clippy::new_without_default)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[cfg(all(
  not(target_family = "wasm"),
  not(all(target_env = "musl"))
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[napi]
#[repr(transparent)]
pub struct Blake3Hasher(blake3::Hasher);

#[napi]
impl Blake3Hasher {
  #[napi]
  pub fn derive_key(context: String, key_material: &[u8]) -> Result<Buffer> {
    Ok(
      blake3::derive_key(context.as_str(), key_material)
        .as_ref()
        .into(),
    )
  }

  #[napi(constructor)]
  pub fn new() -> Self {
    Self(blake3::Hasher::new())
  }

  #[napi(factory)]
  pub fn new_keyed(key: Buffer) -> Self {
    debug_assert!(key.len() == blake3::KEY_LEN);
    let mut target_key = [0; blake3::KEY_LEN];
    target_key.copy_from_slice(key.as_ref());
    Self(blake3::Hasher::new_keyed(&target_key))
  }

  #[napi(factory)]
  pub fn new_derive_key(input: String) -> Self {
    Self(blake3::Hasher::new_derive_key(input.as_str()))
  }

  #[napi]
  pub fn reset(&mut self) {
    self.0.reset();
  }

  #[napi]
  pub fn update(&mut self, input: Either3<String, &[u8], f64>) -> Result<&Self> {
    match input {
      Either3::A(a) => {
        self.0.update(a.as_bytes());
      }
      Either3::B(b) => {
        self.0.update(b);
      }
      Either3::C(c) => {
        let mut buffer = ryu::Buffer::default();
        self.0.update(buffer.format_finite(c).as_bytes());
      }
    }
    Ok(self)
  }

  #[napi]
  pub fn digest(&mut self) -> Buffer {
    self.0.finalize().as_bytes().to_vec().into()
  }
}

#[napi]
pub fn blake3(input: Either<String, &[u8]>) -> Result<Buffer> {
  Ok(match input {
    Either::A(a) => blake3::hash(a.as_bytes()).as_bytes().as_ref().into(),
    Either::B(b) => blake3::hash(b).as_bytes().as_ref().into(),
  })
}
