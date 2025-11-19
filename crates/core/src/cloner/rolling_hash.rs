/// Rabin-Karp rolling hash implementation for efficient string/token matching
///
/// This uses polynomial rolling hash with a large prime modulus to minimize collisions
/// while allowing O(1) hash updates when the window slides.
pub struct RollingHash {
    /// Base for polynomial hash (using 257 for byte values + 1)
    base: u64,
    /// Large prime modulus to reduce collisions
    modulus: u64,
    /// Current hash value
    hash: u64,
    /// Window size
    window_size: usize,
    /// Precomputed base^(window_size-1) mod modulus for removing leftmost element
    base_power: u64,
}

impl RollingHash {
    /// Create a new rolling hash with the specified window size
    pub fn new(window_size: usize) -> Self {
        let base = 257u64;
        let modulus = 1_000_000_007u64;

        let mut base_power = 1u64;
        for _ in 0..window_size.saturating_sub(1) {
            base_power = Self::mul_mod(base_power, base, modulus);
        }

        Self { base, modulus, hash: 0, window_size, base_power }
    }

    /// Initialize hash with a sequence of values
    pub fn init(&mut self, values: &[u64]) {
        self.hash = 0;
        for &val in values.iter().take(self.window_size) {
            self.hash = Self::mul_mod(self.hash, self.base, self.modulus);
            self.hash = Self::add_mod(self.hash, val, self.modulus);
        }
    }

    /// Roll the window: remove the leftmost value and add a new rightmost value
    /// Returns the new hash value
    pub fn roll(&mut self, old_value: u64, new_value: u64) -> u64 {
        let old_contrib = Self::mul_mod(old_value, self.base_power, self.modulus);
        self.hash = Self::sub_mod(self.hash, old_contrib, self.modulus);

        self.hash = Self::mul_mod(self.hash, self.base, self.modulus);
        self.hash = Self::add_mod(self.hash, new_value, self.modulus);

        self.hash
    }

    /// Get current hash value
    pub fn get(&self) -> u64 {
        self.hash
    }

    /// Multiply with modular arithmetic to prevent overflow
    fn mul_mod(a: u64, b: u64, modulus: u64) -> u64 {
        ((a as u128 * b as u128) % modulus as u128) as u64
    }

    /// Add with modular arithmetic
    fn add_mod(a: u64, b: u64, modulus: u64) -> u64 {
        ((a as u128 + b as u128) % modulus as u128) as u64
    }

    /// Subtract with modular arithmetic (handles underflow)
    fn sub_mod(a: u64, b: u64, modulus: u64) -> u64 {
        if a >= b { (a - b) % modulus } else { (modulus - (b - a) % modulus) % modulus }
    }
}

/// Compute a hash value for a token string (for use in rolling hash)
pub fn token_hash(token: &str) -> u64 {
    let mut hash = 5381u64;
    for byte in token.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_hash_basic() {
        let window_size = 3;
        let mut rh = RollingHash::new(window_size);

        let values = [1, 2, 3, 4, 5];
        rh.init(&values[0..3]);

        let hash1 = rh.get();
        assert_ne!(hash1, 0);

        let hash2 = rh.roll(1, 4);
        assert_ne!(hash2, 0);
        assert_ne!(hash1, hash2);

        let hash3 = rh.roll(2, 5);
        assert_ne!(hash3, 0);
        assert_ne!(hash2, hash3);
    }

    #[test]
    fn test_same_sequence_same_hash() {
        let mut rh1 = RollingHash::new(4);
        let mut rh2 = RollingHash::new(4);

        let seq = vec![10, 20, 30, 40];
        rh1.init(&seq);
        rh2.init(&seq);

        assert_eq!(rh1.get(), rh2.get());
    }

    #[test]
    fn test_different_sequences_different_hash() {
        let mut rh1 = RollingHash::new(4);
        let mut rh2 = RollingHash::new(4);

        rh1.init(&[1, 2, 3, 4]);
        rh2.init(&[1, 2, 3, 5]);

        assert_ne!(rh1.get(), rh2.get());
    }

    #[test]
    fn test_token_hash() {
        let h1 = token_hash("if");
        let h2 = token_hash("else");
        let h3 = token_hash("if");

        assert_ne!(h1, h2);
        assert_eq!(h1, h3);
    }

    #[test]
    fn test_rolling_preserves_pattern() {
        let mut rh = RollingHash::new(3);
        let values = [1, 2, 3, 4, 5, 6, 1, 2, 3];

        rh.init(&values[0..3]);
        let first_hash = rh.get();

        let mut current_hash = first_hash;
        for i in 3..9 {
            current_hash = rh.roll(values[i - 3], values[i]);
        }

        assert_eq!(current_hash, first_hash);
    }
}
