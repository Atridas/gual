use std::{marker::PhantomData, ops::Mul};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{GeometricElement, Scalar};

use super::{Bivector, Evenvector, Multivector, Vector};

impl<T, M> Vector<T, M> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Bivector<T, M> {
    pub fn new(xy: T) -> Self {
        Self {
            xy,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Zero for Vector<T, M>
where
    T: Zero,
{
    fn zero() -> Self {
        Vector {
            x: T::zero(),
            y: T::zero(),
            _metric: PhantomData,
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T, M> Zero for Bivector<T, M>
where
    T: Zero,
{
    fn zero() -> Self {
        Bivector {
            xy: T::zero(),
            _metric: PhantomData,
        }
    }

    fn is_zero(&self) -> bool {
        self.xy.is_zero()
    }
}

impl<T, M> Zero for Evenvector<T, M>
where
    T: Zero,
{
    fn zero() -> Self {
        Evenvector {
            s: T::zero(),
            b: Bivector::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.s.is_zero() && self.b.is_zero()
    }
}

impl<T, M> Zero for Multivector<T, M>
where
    T: Zero,
{
    fn zero() -> Self {
        Multivector {
            s: T::zero(),
            v: Vector::zero(),
            b: Bivector::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.s.is_zero() && self.v.is_zero() && self.b.is_zero()
    }
}

impl<T, M> ConstZero for Vector<T, M>
where
    T: ConstZero,
{
    const ZERO: Self = Vector {
        x: T::ZERO,
        y: T::ZERO,
        _metric: PhantomData,
    };
}

impl<T, M> ConstZero for Bivector<T, M>
where
    T: ConstZero,
{
    const ZERO: Self = Bivector {
        xy: T::ZERO,
        _metric: PhantomData,
    };
}

impl<T, M> ConstZero for Evenvector<T, M>
where
    T: ConstZero,
{
    const ZERO: Self = Evenvector {
        s: T::ZERO,
        b: Bivector::ZERO,
    };
}

impl<T, M> ConstZero for Multivector<T, M>
where
    T: ConstZero,
{
    const ZERO: Self = Multivector {
        s: T::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZERO,
    };
}

impl<T, M> One for Evenvector<T, M>
where
    T: Zero,
    T: One,
    Evenvector<T, M>: Mul<Output = Evenvector<T, M>>, // TODO!
{
    fn one() -> Self {
        Evenvector {
            s: T::one(),
            b: Bivector::zero(),
        }
    }
}

impl<T, M> One for Multivector<T, M>
where
    T: Zero,
    T: One,
    Multivector<T, M>: Mul<Output = Multivector<T, M>>, // TODO!
{
    fn one() -> Self {
        Multivector {
            s: T::one(),
            v: Vector::zero(),
            b: Bivector::zero(),
        }
    }
}

impl<T, M> ConstOne for Evenvector<T, M>
where
    T: ConstZero,
    T: ConstOne,
    Evenvector<T, M>: Mul<Output = Evenvector<T, M>>, // TODO!
{
    const ONE: Self = Evenvector {
        s: T::ONE,
        b: Bivector::ZERO,
    };
}

impl<T, M> ConstOne for Multivector<T, M>
where
    T: ConstZero,
    T: ConstOne,
    Multivector<T, M>: Mul<Output = Multivector<T, M>>, // TODO!
{
    const ONE: Self = Multivector {
        s: T::ONE,
        v: Vector::ZERO,
        b: Bivector::ZERO,
    };
}

impl<T, M> Vector<T, M>
where
    T: ConstZero,
    T: ConstOne,
{
    /// Unit vector in the X direction
    pub const X: Self = Vector {
        x: T::ONE,
        y: T::ZERO,
        _metric: PhantomData,
    };

    /// Unit vector in the Y direction
    pub const Y: Self = Vector {
        x: T::ZERO,
        y: T::ONE,
        _metric: PhantomData,
    };
}

impl<T, M> Bivector<T, M>
where
    T: ConstOne,
{
    /// Unit bivector
    pub const XY: Self = Bivector {
        xy: T::ONE,
        _metric: PhantomData,
    };
}

impl<T, M> Evenvector<T, M>
where
    T: ConstZero,
    T: ConstOne,
{
    /// Unit bivector
    pub const XY: Self = Evenvector {
        s: T::ZERO,
        b: Bivector::XY,
    };
}

impl<T, M> Multivector<T, M>
where
    T: ConstZero,
    T: ConstOne,
{
    /// Unit vector in the X direction
    pub const X: Self = Multivector {
        s: T::ZERO,
        v: Vector::X,
        b: Bivector::ZERO,
    };

    /// Unit vector in the Y direction
    pub const Y: Self = Multivector {
        s: T::ZERO,
        v: Vector::Y,
        b: Bivector::ZERO,
    };

    /// Unit bivector
    pub const XY: Self = Multivector {
        s: T::ZERO,
        v: Vector::ZERO,
        b: Bivector::XY,
    };
}

impl<T, M> GeometricElement for Vector<T, M> {
    const ALGEBRA_DIMENSION: u32 = 2;
    const ELEMENT_DIMENSION: u32 = 1;
    type Field = T;
    type Metric = M;
    type Scalar = Scalar<2, T, M>;
}

impl<T, M> GeometricElement for Bivector<T, M> {
    const ALGEBRA_DIMENSION: u32 = 2;
    const ELEMENT_DIMENSION: u32 = 2;
    type Field = T;
    type Metric = M;
    type Scalar = Scalar<2, T, M>;
}
