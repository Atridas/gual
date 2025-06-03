use crate::{bulk_expansion, weight_expansion};

use super::{BivectorIt, VectorIt};

use gual::Expansion;

#[test]
fn expansion_vector_vector() {
    for v in VectorIt::new(50) {
        for v2 in VectorIt::new(50) {
            assert_eq!(v.bulk_expansion(&v2), bulk_expansion(v, v2));
            assert_eq!(v.weight_expansion(&v2), weight_expansion(v, v2));
        }
    }
}

#[test]
fn expansion_bivector_bivector() {
    for b in BivectorIt::new(50) {
        for b2 in BivectorIt::new(50) {
            assert_eq!(b.bulk_expansion(&b2), bulk_expansion(b, b2));
            assert_eq!(b.weight_expansion(&b2), weight_expansion(b, b2));
        }
    }
}
