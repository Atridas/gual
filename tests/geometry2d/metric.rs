use gual::{Complement, Metric, Scalar, canonical_weight};

use super::{BivectorIt, VectorIt};

#[test]
fn metric_vector() {
    for v in VectorIt::new(50) {
        assert_eq!(v.proper_bulk(), v.bulk());
        assert_eq!(v.proper_weight(), v.weight());

        assert_eq!(v.proper_weight(), canonical_weight(v));
    }
}

#[test]
fn metric_bivector() {
    for b in BivectorIt::new(50) {
        assert_eq!(b.proper_bulk(), b.bulk());
        assert_eq!(b.proper_weight(), b.weight());

        assert_eq!(
            b.proper_weight(),
            Scalar::<2, _>::new(b.left_complement())
                .proper_bulk()
                .right_complement()
        );
    }
}
