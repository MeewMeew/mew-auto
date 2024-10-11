use anyhow::Result;

pub fn hash32(data: &[u8]) -> Result<u32> {
  const FNV_OFFSET_BASIS: u32 = 0x811C9DC5;
  const FNV_32_PRIME: u32 = 0x01000193;

  let mut hash = FNV_OFFSET_BASIS;

  for &byte in data {
    hash ^= byte as u32;
    hash = hash.wrapping_mul(FNV_32_PRIME);
  }

  Ok(hash)
}
