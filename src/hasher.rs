#[cfg(not(feature = "experimental"))]
use crate::sha512;

#[cfg(not(feature = "experimental"))]
#[derive(Clone)]
pub struct Hasher(sha512::Hash);

#[cfg(not(feature = "experimental"))]
impl Hasher {
    pub fn new() -> Self {
        Self(sha512::Hash::new())
    }

    pub fn update<T: AsRef<[u8]>>(&mut self, input: T) {
        self.0.update(input);
    }

    pub fn finalize(self) -> [u8; 64] {
        self.0.finalize()
    }

    pub fn hash<T: AsRef<[u8]>>(input: T) -> [u8; 64] {
        sha512::Hash::hash(input)
    }
}

#[cfg(feature = "experimental")]
#[derive(Clone)]
pub struct Hasher(blake3::Hasher);

#[cfg(feature = "experimental")]
impl Hasher {
    pub fn new() -> Self {
        Self(blake3::Hasher::new())
    }

    pub fn update<T: AsRef<[u8]>>(&mut self, input: T) {
        self.0.update(input.as_ref());
    }

    pub fn finalize(self) -> [u8; 64] {
        let mut buffer = [0u8; 64];
        self.0.finalize_xof()
            .fill(&mut buffer);
        buffer
    }

    pub fn hash<T: AsRef<[u8]>>(input: T) -> [u8; 64] {
        let mut hasher = Self::new();
        hasher.update(input);
        hasher.finalize()
    }
}