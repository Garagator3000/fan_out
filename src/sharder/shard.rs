use super::hash;

pub trait Sharder {
    fn shard(&self, key: &[u8]) -> usize;
}

pub struct HashSharder {
    pub shards: usize,
}

impl HashSharder {
    pub fn new_hash_sharder(shards: u8) -> HashSharder {
        HashSharder {
            shards: usize::from(shards)
        }
    }
}

impl Sharder for HashSharder {
    fn shard(&self, key: &[u8]) -> usize {
        (hash::hash(key) as usize) % self.shards
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        let s = HashSharder { shards: 3 };

        let a = s.shard(b"hello");
        let b = s.shard(b"hello");

        assert_eq!(a, b);
    }

    #[test]
    fn within_bounds() {
        let s = HashSharder { shards: 10 };

        for i in 0..1000 {
            let key = format!("key-{i}");
            let shard = s.shard(key.as_bytes());

            assert!(shard < 10);
        }
    }

    #[test]
    fn distribution_is_reasonable() {
        let shards = 5;
        let s = HashSharder { shards };

        let mut buckets = vec![0usize; shards];

        let total = 100_000;

        for i in 0..total {
            let key = format!("key-{i}");
            let shard = s.shard(key.as_bytes());

            buckets[shard] += 1;
        }

        let expected = total / shards;

        for count in buckets {
            // допускаем отклонение 20%
            let lower = expected * 80 / 100;
            let upper = expected * 120 / 100;

            assert!(
                count >= lower && count <= upper,
                "bad distribution: {count}"
            );
        }
    }
}
