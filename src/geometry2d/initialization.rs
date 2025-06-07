use std::{marker::PhantomData, ops::Mul};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{Unitizable, geometry2d::Point};

use super::{Bivector, Evenvector, Multivector, UnitVector, Vector};

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

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self(Vector::new(x, y))
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

impl<T> UnitVector<T>
where
    T: ConstZero,
    T: ConstOne,
{
    /// Unit vector in the X direction
    pub const X: Self = UnitVector(Vector::X);

    /// Unit vector in the Y direction
    pub const Y: Self = UnitVector(Vector::Y);
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

impl<T> Point<T>
where
    T: ConstZero,
{
    /// Coordinate origin
    pub const ORIGIN: Self = Point(Vector::ZERO);
}

impl<T> From<UnitVector<T>> for Vector<T> {
    fn from(value: UnitVector<T>) -> Self {
        value.0
    }
}

impl<T> TryFrom<Vector<T>> for UnitVector<T>
where
    Vector<T>: Unitizable<Output = UnitVector<T>>,
{
    type Error = ();
    fn try_from(value: Vector<T>) -> Result<Self, Self::Error> {
        match value.unitize() {
            Some(unit) => Ok(unit),
            None => Err(()),
        }
    }
}

impl<T> UnitVector<T> {
    /// Creates a new unit vector.
    ///
    /// It is marked unsafe even though it is not "rust unsafe" to use,
    /// you should really initialize it with only a unit vector if you
    /// want other operations to have a meaningful result.
    pub unsafe fn new(value: Vector<T>) -> Self {
        UnitVector(value)
    }
}
