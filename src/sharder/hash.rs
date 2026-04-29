const FNV_PRIME: u64 = 1099511628211;
const FNV_OFFSET: u64 = 1469598103934665603;

pub fn hash(key: &[u8]) -> u64 {
    let mut hash: u64 = FNV_OFFSET;
    let prime = FNV_PRIME;

    for b in key {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(prime);
    }

    hash
}

mod tests {
    #[test]
    fn same_input_same_hash() {
        let a = super::hash(b"hello");
        let b = super::hash(b"hello");
        assert_eq!(a, b);
    }

    #[test]
    fn different_input_different_hash() {
        let a = super::hash(b"hello");
        let b = super::hash(b"world");
        assert_ne!(a, b);
    }
}
