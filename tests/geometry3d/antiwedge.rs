use gual::{
    AntiwedgeProduct, Complement, Euclidean, Scalar, canonical::Antiwedge, geometry3d::Trivector,
};

use super::ScalarIt;

type VectorIt = crate::geometry3d::VectorIt<Euclidean>;
type BivectorIt = crate::geometry3d::BivectorIt<Euclidean>;
type TrivectorIt = crate::geometry3d::TrivectorIt<Euclidean>;

#[test]
fn antiwedge_vector_bivector() {
    for v in VectorIt::new(20) {
        for b in BivectorIt::new(20) {
            // actual implementation matches definition
            assert_eq!(v.antiwedge(&b), v.canonical_antiwedge(&b));
            assert_eq!(b.antiwedge(&v), b.canonical_antiwedge(&v));
        }
    }
}

#[test]
fn antiwedge_vector_trivector() {
    for v in VectorIt::new(20) {
        for t in TrivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(v.antiwedge(&t), v.canonical_antiwedge(&t));
            assert_eq!(t.antiwedge(&v), t.canonical_antiwedge(&v));
        }
    }
}

#[test]
fn antiwedge_bivector_bivector() {
    for b1 in BivectorIt::new(20) {
        for b2 in BivectorIt::new(20) {
            // actual implementation matches definition
            assert_eq!(b1.antiwedge(&b2), b1.canonical_antiwedge(&b2));
            assert_eq!(b2.antiwedge(&b1), b2.canonical_antiwedge(&b1));
        }
    }
}

#[test]
fn antiwedge_bivector_trivector() {
    for b in BivectorIt::new(20) {
        for t in TrivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(b.antiwedge(&t), b.canonical_antiwedge(&t));
            assert_eq!(t.antiwedge(&b), t.canonical_antiwedge(&b));
        }
    }
}

#[test]
fn antiwedge_trivector_trivector() {
    let antiwedge = |a: Trivector<i32>, b: Trivector<i32>| {
        let s = a.left_complement() * b.left_complement();
        Scalar::<3, _>::new(s).right_complement()
    };

    for t1 in TrivectorIt::new(100) {
        for t2 in TrivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(t1.antiwedge(&t2), antiwedge(t1, t2));
            assert_eq!(t2.antiwedge(&t1), antiwedge(t2, t1));
        }
    }
}

#[test]
fn antiwedge_scalar_trivector() {
    for s in ScalarIt::new(100) {
        let sc: Scalar<3, _> = Scalar::new(s);
        for t in TrivectorIt::new(100) {
            // actual implementation matches definition
            assert_eq!(s.antiwedge(&t), sc.canonical_antiwedge(&t));
            assert_eq!(t.antiwedge(&s), t.canonical_antiwedge(&sc));
        }
    }
}
