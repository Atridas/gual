use gual::{Dot, Metric};

use crate::geometry2d::{BivectorIt, VectorIt};

#[test]
fn dot_vector() {
    for v1 in VectorIt::new(50) {
        for v2 in VectorIt::new(50) {
            assert_eq!(v1.dot(&v2), v1.bulk().dot(&v2.bulk()));
            assert_eq!(v1.antidot(&v2), v1.weight().antidot(&v2.weight()));
        }
    }
}

#[test]
fn dot_bivector() {
    for b1 in BivectorIt::new(50) {
        for b2 in BivectorIt::new(50) {
            assert_eq!(b1.dot(&b2), b1.bulk().dot(&b2.bulk()));
            assert_eq!(b1.antidot(&b2), b1.weight().antidot(&b2.weight()));
        }
    }
}
