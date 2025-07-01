use gual::{
    Angle, Epsilon, Euclidean, Projective,
    canonical::{Angle as CanonicalAngle, SymetricAngle},
};

use crate::geometry3d::{BivectorIt, ToF32, VectorIt};

#[test]
fn euclidean_angle_vector_vector() {
    for a in VectorIt::<Euclidean>::new(10) {
        for b in VectorIt::<Euclidean>::new(10) {
            let a = a.to_f32();
            let b = b.to_f32();

            let v = a.geometric_cosine(&b);
            let r = a.canonical_geometric_cosine_symetric(&b);

            assert!((v.0 - r.0).is_near_zero());
            assert!((v.1.xyz - r.1.xyz).is_near_zero());

            let v = a.cosine(&b);
            let r = a.canonical_cosine_symetric(&b);

            match (v, r) {
                (Some(v), Some(r)) => assert!((v - r).is_near_zero()),
                (None, None) => {}
                (Some(_), _) => panic!("implementation succeded while reference failed!"),
                (_, Some(_)) => panic!("reference succeded while implementation failed!"),
            }
        }
    }
}

#[test]
fn euclidean_angle_vector_bivector() {
    for a in VectorIt::<Euclidean>::new(10) {
        for b in BivectorIt::<Euclidean>::new(10) {
            let a = a.to_f32();
            let b = b.to_f32();

            let v = a.geometric_cosine(&b);
            let r = a.canonical_geometric_cosine(&b);

            assert!((v.0 - r.0).is_near_zero());
            assert!((v.1.xyz - r.1.xyz).is_near_zero());

            let v = a.cosine(&b);
            let r = a.canonical_cosine(&b);

            match (v, r) {
                (Some(v), Some(r)) => assert!((v - r).is_near_zero()),
                (None, None) => {}
                (Some(_), _) => panic!("implementation succeded while reference failed!"),
                (_, Some(_)) => panic!("reference succeded while implementation failed!"),
            }
        }
    }
}

#[test]
fn euclidean_angle_bivector_vector() {
    for a in BivectorIt::<Euclidean>::new(10) {
        for b in VectorIt::<Euclidean>::new(10) {
            let a = a.to_f32();
            let b = b.to_f32();

            let v = a.geometric_cosine(&b);
            let r = a.canonical_geometric_cosine(&a);

            assert!((v.0 - r.0).is_near_zero());
            assert!((v.1.xyz - r.1.xyz).is_near_zero());

            let v = a.cosine(&b);
            let r = a.canonical_cosine(&a);

            match (v, r) {
                (Some(v), Some(r)) => assert!((v - r).is_near_zero()),
                (None, None) => {}
                (Some(_), _) => panic!("implementation succeded while reference failed!"),
                (_, Some(_)) => panic!("reference succeded while implementation failed!"),
            }
        }
    }
}

#[test]
fn euclidean_angle_bivector_bivector() {
    for a in BivectorIt::<Euclidean>::new(10) {
        for b in BivectorIt::<Euclidean>::new(10) {
            let a = a.to_f32();
            let b = b.to_f32();

            let v = a.geometric_cosine(&b);
            let r = a.canonical_geometric_cosine_symetric(&b);

            assert!((v.0 - r.0).is_near_zero());
            assert!((v.1.xyz - r.1.xyz).is_near_zero());

            let v = a.cosine(&b);
            let r = a.canonical_cosine_symetric(&b);

            match (v, r) {
                (Some(v), Some(r)) => assert!((v - r).is_near_zero()),
                (None, None) => {}
                (Some(_), _) => panic!("implementation succeded while reference failed!"),
                (_, Some(_)) => panic!("reference succeded while implementation failed!"),
            }
        }
    }
}

#[test]
fn projective_angle_bivector_bivector() {
    for a in BivectorIt::<Projective>::new(10) {
        for b in BivectorIt::<Projective>::new(10) {
            let a = a.to_f32();
            let b = b.to_f32();

            let v = a.geometric_cosine(&b);
            let r = a.canonical_geometric_cosine_symetric(&b);

            assert!((v.0 - r.0).is_near_zero());
            assert!((v.1.xyz - r.1.xyz).is_near_zero());

            let v = a.cosine(&b);
            let r = a.canonical_cosine_symetric(&b);

            match (v, r) {
                (Some(v), Some(r)) => assert!((v - r).is_near_zero()),
                (None, None) => {}
                (Some(_), _) => panic!("implementation succeded while reference failed!"),
                (_, Some(_)) => panic!("reference succeded while implementation failed!"),
            }
        }
    }
}
