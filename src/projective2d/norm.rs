use num::traits::ConstOne;

use crate::{
    Antiscalar, Norm, Projective,
    geometry3d::Trivector,
    projective2d::{UnitLine, UnitVector},
};

impl<T> Norm for UnitVector<T>
where
    T: Clone,
    T: ConstOne,
{
    type Scalar = T;
    type Antiscalar = Trivector<T, Projective>;

    fn bulk_norm_squared(&self) -> T {
        T::ONE
    }

    fn weight_norm_squared(&self) -> Trivector<T, Projective> {
        Trivector::UNIT_VOLUME
    }

    fn bulk_norm(&self) -> T {
        T::ONE
    }

    fn weight_norm(&self) -> Trivector<T, Projective> {
        Trivector::UNIT_VOLUME
    }
}

impl<T> Norm for UnitLine<T>
where
    T: Clone,
    T: ConstOne,
{
    type Scalar = T;
    type Antiscalar = Trivector<T, Projective>;

    fn bulk_norm_squared(&self) -> T {
        T::ONE
    }

    fn weight_norm_squared(&self) -> Trivector<T, Projective> {
        Trivector::UNIT_VOLUME
    }

    fn bulk_norm(&self) -> T {
        T::ONE
    }

    fn weight_norm(&self) -> Trivector<T, Projective> {
        Trivector::UNIT_VOLUME
    }
}
