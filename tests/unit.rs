
#[allow(unused_imports)]
#[cfg(test)]
mod unit {
    use std::collections::HashSet;
    use std::hash::Hash;

    use pvault::*;

    #[test]
    fn handler_discriminator_check() {
        let discriminators = [
            pvault::InitializeProc::DISCRIMINATOR,
        ];

        assert!(is_unique(discriminators));
    }
    #[test]
    fn ha_discriminator_check() {
        let discriminators = [
            pvault::Vault::DISCRIMINATOR,
        ];

        assert!(is_unique(discriminators));
    }

    fn is_unique<T>(iter: T) -> bool where
        T: IntoIterator,
        T::Item: Eq + Hash, {
        let mut hash_set = HashSet::new();
        iter.into_iter().all(|d| hash_set.insert(d))
    }
}
