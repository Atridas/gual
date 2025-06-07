use num::Float;

use crate::{
    Angle, Dot, Epsilon, Norm,
    geometry2d::{Bivector, Vector},
};

impl<T> Angle<Vector<T>> for Vector<T>
where
    T: Float,
    T: Epsilon,
    Vector<T>: Dot<Scalar = T>,
    Vector<T>: Norm<Scalar = T>,
{
    type Scalar = T;
    type Antiscalar = Bivector<T>;

    fn cosine(&self, rhs: &Vector<T>) -> Option<T> {
        let div = self.norm_squared() * rhs.norm_squared();
        if div.is_near_zero() {
            None
        } else {
            Some(self.dot(rhs) / div.sqrt())
        }
    }

    fn geometric_cosine(&self, rhs: &Vector<T>) -> (T, Bivector<T>) {
        (
            self.dot(rhs),
            Bivector::new((self.norm_squared() * rhs.norm_squared()).sqrt()),
        )
    }
}
