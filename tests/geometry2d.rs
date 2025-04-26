use gual::{
    AntiwedgeProduct, Bivector2D, KVector, Scalar2D, Vector2D, WedgeProduct, antiwedge_reference,
};

#[test]
fn complement_scalar() {
    let i = Bivector2D { xy: 1 };

    let s = Scalar2D(1);
    assert_eq!(s.wedge(s.right_complement()), i);
    assert_eq!(s.left_complement().wedge(s), i);
}

#[test]
fn complement_vector() {
    let i = Bivector2D { xy: 1 };

    let v = Vector2D { x: 1, y: 0 };
    assert_eq!(v.wedge(v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(v), i);

    let v = Vector2D { x: 0, y: 1 };
    assert_eq!(v.wedge(v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(v), i);
}

#[test]
fn complement_bivector() {
    let i = Bivector2D { xy: 1 };

    let b = Bivector2D { xy: 1 };
    assert_eq!(b.wedge(b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(b), i);
}

#[test]
fn wedge_scalar_scalar() {
    for s in 0..100 {
        let s = Scalar2D(s);
        for s2 in 0..100 {
            let s2 = Scalar2D(s2);

            // scalars always commute
            assert_eq!(s.wedge(s2), s2.wedge(s));
        }
    }
}

#[test]
fn wedge_scalar_vector() {
    for s in 0..100 {
        let s = Scalar2D(s);
        for x in 0..50 {
            for y in 0..50 {
                let v = Vector2D { x, y };

                // scalars always commute
                assert_eq!(s.wedge(v), v.wedge(s));
            }
        }
    }
}

#[test]
fn wedge_scalar_bivector() {
    for s in 0..100 {
        let s = Scalar2D(s);
        for xy in 0..100 {
            let b = Bivector2D { xy };

            // scalars always commute
            assert_eq!(s.wedge(b), b.wedge(s));
        }
    }
}

#[test]
fn wedge_vector_bivector() {
    for x in 0..50 {
        for y in 0..50 {
            let v = Vector2D { x, y };

            for x in 0..50 {
                for y in 0..50 {
                    let v2 = Vector2D { x, y };

                    // vector - vector anticommute
                    assert_eq!(v.wedge(v2), -v2.wedge(v));
                }
            }
        }
    }
}

#[test]
fn antiwedge_scalar_bivector() {
    for i in 0..100 {
        let s = Scalar2D(i);
        for xy in 0..100 {
            let b = Bivector2D { xy };

            // actual implementation matches definition
            assert_eq!(s.antiwedge(b), antiwedge_reference(s, b));
            assert_eq!(b.antiwedge(s), antiwedge_reference(b, s));
        }
    }
}

#[test]
fn antiwedge_vector_vector() {
    for x in 0..50 {
        for y in 0..50 {
            let a = Vector2D { x, y };
            for x in 0..50 {
                for y in 0..50 {
                    let b = Vector2D { x, y };
                    // actual implementation matches definition
                    assert_eq!(a.antiwedge(b), antiwedge_reference(a, b));
                    assert_eq!(b.antiwedge(a), antiwedge_reference(b, a));
                }
            }
        }
    }
}

#[test]
fn antiwedge_vector_bivector() {
    for x in 0..50 {
        for y in 0..50 {
            let a = Vector2D { x, y };
            for xy in 0..50 {
                let b = Bivector2D { xy };
                // actual implementation matches definition
                assert_eq!(a.antiwedge(b), antiwedge_reference(a, b));
                assert_eq!(b.antiwedge(a), antiwedge_reference(b, a));
            }
        }
    }
}

#[test]
fn antiwedge_bivector_bivector() {
    for xy in 0..50 {
        let a = Bivector2D { xy };
        for xy in 0..50 {
            let b = Bivector2D { xy };
            // actual implementation matches definition
            assert_eq!(a.antiwedge(b), antiwedge_reference(a, b));
            assert_eq!(b.antiwedge(a), antiwedge_reference(b, a));
        }
    }
}
