use gual::{
    AntiwedgeProduct, Bivector3D, KVector, Scalar3D, Trivector3D, Vector3D, WedgeProduct,
    antiwedge_reference,
};

#[test]
fn complement_scalar() {
    let i = Trivector3D { xyz: 1 };

    let s = Scalar3D(1);
    assert_eq!(s.wedge(s.right_complement()), i);
    assert_eq!(s.left_complement().wedge(s), i);
}

#[test]
fn complement_vector() {
    let i = Trivector3D { xyz: 1 };

    let v = Vector3D { x: 1, y: 0, z: 0 };
    assert_eq!(v.wedge(v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(v), i);

    let v = Vector3D { x: 0, y: 1, z: 0 };
    assert_eq!(v.wedge(v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(v), i);

    let v = Vector3D { x: 0, y: 0, z: 1 };
    assert_eq!(v.wedge(v.right_complement()), i);
    assert_eq!(v.left_complement().wedge(v), i);
}

#[test]
fn complement_bivector() {
    let i = Trivector3D { xyz: 1 };

    let b = Bivector3D {
        yz: 1,
        zx: 0,
        xy: 0,
    };
    assert_eq!(b.wedge(b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(b), i);

    let b = Bivector3D {
        yz: 0,
        zx: 1,
        xy: 0,
    };
    assert_eq!(b.wedge(b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(b), i);

    let b = Bivector3D {
        yz: 0,
        zx: 0,
        xy: 1,
    };
    assert_eq!(b.wedge(b.right_complement()), i);
    assert_eq!(b.left_complement().wedge(b), i);
}

#[test]
fn complement_trivector() {
    let i = Trivector3D { xyz: 1 };

    let t = Trivector3D { xyz: 1 };
    assert_eq!(t.wedge(t.right_complement()), i);
    assert_eq!(t.left_complement().wedge(t), i);
}

#[test]
fn wedge_scalar_scalar() {
    for s in 0..100 {
        let s = Scalar3D(s);
        for s2 in 0..100 {
            let s2 = Scalar3D(s2);

            // scalars always commute
            assert_eq!(s.wedge(s2), s2.wedge(s));
        }
    }
}

#[test]
fn wedge_scalar_vector() {
    for s in 0..100 {
        let s = Scalar3D(s);
        for x in 20..40 {
            for y in 20..40 {
                for z in 20..40 {
                    let v = Vector3D { x, y, z };

                    // scalars always commute
                    assert_eq!(s.wedge(v), v.wedge(s));
                }
            }
        }
    }
}

#[test]
fn wedge_scalar_bivector() {
    for s in 0..100 {
        let s = Scalar3D(s);
        for yz in 20..40 {
            for zx in 20..40 {
                for xy in 20..40 {
                    let b = Bivector3D { yz, zx, xy };

                    // scalars always commute
                    assert_eq!(s.wedge(b), b.wedge(s));
                }
            }
        }
    }
}

#[test]
fn wedge_vector_vector() {
    for x in 20..40 {
        for y in 20..40 {
            for z in 20..40 {
                let v = Vector3D { x, y, z };

                for x in 20..40 {
                    for y in 20..40 {
                        for z in 20..40 {
                            let v2 = Vector3D { x, y, z };

                            // vector - vector anticommute
                            assert_eq!(v.wedge(v2), -v2.wedge(v));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn wedge_vector_bivector() {
    for x in 20..40 {
        for y in 20..40 {
            for z in 20..40 {
                let v = Vector3D { x, y, z };

                for yz in 20..40 {
                    for zx in 20..40 {
                        for xy in 20..40 {
                            let b = Bivector3D { yz, zx, xy };

                            // bivectors always commute
                            assert_eq!(v.wedge(b), b.wedge(v));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn antiwedge_scalar_trivector() {
    for i in 0..100 {
        let s = Scalar3D(i);
        for xyz in 0..100 {
            let t = Trivector3D { xyz };

            // actual implementation matches definition
            assert_eq!(s.antiwedge(t), antiwedge_reference(s, t));
            assert_eq!(t.antiwedge(s), antiwedge_reference(t, s));
        }
    }
}

#[test]
fn antiwedge_vector_bivector() {
    for x in 20..40 {
        for y in 20..40 {
            for z in 20..40 {
                let v = Vector3D { x, y, z };

                for yz in 20..40 {
                    for zx in 20..40 {
                        for xy in 20..40 {
                            let b = Bivector3D { yz, zx, xy };
                            // actual implementation matches definition
                            assert_eq!(v.antiwedge(b), antiwedge_reference(v, b));
                            assert_eq!(b.antiwedge(v), antiwedge_reference(b, v));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn antiwedge_vector_trivector() {
    for x in 20..40 {
        for y in 20..40 {
            for z in 20..40 {
                let v = Vector3D { x, y, z };
                for xyz in 0..50 {
                    let t = Trivector3D { xyz };
                    // actual implementation matches definition
                    assert_eq!(v.antiwedge(t), antiwedge_reference(v, t));
                    assert_eq!(t.antiwedge(v), antiwedge_reference(t, v));
                }
            }
        }
    }
}

#[test]
fn antiwedge_bivector_bivector() {
    for yz in 20..40 {
        for zx in 20..40 {
            for xy in 20..40 {
                let b1 = Bivector3D { yz, zx, xy };

                for yz in 20..40 {
                    for zx in 20..40 {
                        for xy in 20..40 {
                            let b2 = Bivector3D { yz, zx, xy };
                            // actual implementation matches definition
                            assert_eq!(b1.antiwedge(b2), antiwedge_reference(b1, b2));
                            assert_eq!(b2.antiwedge(b1), antiwedge_reference(b2, b1));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn antiwedge_bivector_trivector() {
    for yz in 20..40 {
        for zx in 20..40 {
            for xy in 20..40 {
                let b = Bivector3D { yz, zx, xy };

                for xyz in 20..40 {
                    let t = Trivector3D { xyz };
                    // actual implementation matches definition
                    assert_eq!(b.antiwedge(t), antiwedge_reference(b, t));
                    assert_eq!(t.antiwedge(b), antiwedge_reference(t, b));
                }
            }
        }
    }
}

#[test]
fn antiwedge_trivector_trivector() {
    for xyz in 20..40 {
        let t1 = Trivector3D { xyz };

        for xyz in 20..40 {
            let t2 = Trivector3D { xyz };
            // actual implementation matches definition
            assert_eq!(t1.antiwedge(t2), antiwedge_reference(t1, t2));
            assert_eq!(t2.antiwedge(t1), antiwedge_reference(t2, t1));
        }
    }
}
