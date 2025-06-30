use gual::{Dot, Euclidean, Metric, Projective};

use crate::geometry3d::{BivectorIt, TrivectorIt, VectorIt};

#[test]
fn euclidean_dot_vector() {
    for v1 in VectorIt::<Euclidean>::new(15) {
        for v2 in VectorIt::<Euclidean>::new(15) {
            assert_eq!(v1.dot(&v2), v1.bulk().dot(&v2.bulk()));
            assert_eq!(v1.antidot(&v2), v1.weight().antidot(&v2.weight()));
        }
    }
}

#[test]
fn euclidean_dot_bivector() {
    for b1 in BivectorIt::<Euclidean>::new(15) {
        for b2 in BivectorIt::<Euclidean>::new(15) {
            assert_eq!(b1.dot(&b2), b1.bulk().dot(&b2.bulk()));
            assert_eq!(b1.antidot(&b2), b1.weight().antidot(&b2.weight()));
        }
    }
}

#[test]
fn euclidean_dot_trivector() {
    for b1 in TrivectorIt::<Euclidean>::new(50) {
        for b2 in TrivectorIt::<Euclidean>::new(50) {
            assert_eq!(b1.dot(&b2), b1.bulk().dot(&b2.bulk()));
            assert_eq!(b1.antidot(&b2), b1.weight().antidot(&b2.weight()));
        }
    }
}

#[test]
fn projective_dot_vector() {
    for v1 in VectorIt::<Projective>::new(15) {
        for v2 in VectorIt::<Projective>::new(15) {
            assert_eq!(v1.dot(&v2), v1.bulk().dot(&v2.bulk()));
            assert_eq!(
                v1.antidot(&v2),
                v1.proper_weight().antidot(&v2.proper_weight())
            );
        }
    }
}

#[test]
fn projective_dot_bivector() {
    for b1 in BivectorIt::<Projective>::new(15) {
        for b2 in BivectorIt::<Projective>::new(15) {
            assert_eq!(b1.dot(&b2), b1.bulk().dot(&b2.bulk()));
            assert_eq!(
                b1.antidot(&b2),
                b1.proper_weight().antidot(&b2.proper_weight())
            );
        }
    }
}

#[test]
fn projective_dot_trivector() {
    for b1 in TrivectorIt::<Projective>::new(50) {
        for b2 in TrivectorIt::<Projective>::new(50) {
            assert_eq!(b1.dot(&b2), b1.proper_bulk().dot(&b2.proper_bulk()));
            assert_eq!(
                b1.antidot(&b2),
                b1.proper_weight().antidot(&b2.proper_weight())
            );
        }
    }
}
