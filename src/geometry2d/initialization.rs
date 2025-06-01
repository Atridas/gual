use std::{marker::PhantomData, ops::Mul};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

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

impl<T, M> Evenvector<T, M> {
    pub fn new(s: T, b: Bivector<T, M>) -> Self {
        Self { s, b }
    }
}

impl<T, M> Multivector<T, M> {
    pub fn new(s: T, v: Vector<T, M>, b: Bivector<T, M>) -> Self {
        Self { s, v, b }
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
    pub const X: Self = Vector {
        x: T::ONE,
        y: T::ZERO,
        _metric: PhantomData,
    };

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
    pub const X: Self = Multivector {
        s: T::ZERO,
        v: Vector::X,
        b: Bivector::ZERO,
    };

    pub const Y: Self = Multivector {
        s: T::ZERO,
        v: Vector::Y,
        b: Bivector::ZERO,
    };

    pub const XY: Self = Multivector {
        s: T::ZERO,
        v: Vector::ZERO,
        b: Bivector::XY,
    };
}
