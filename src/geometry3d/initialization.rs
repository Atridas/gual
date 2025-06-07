use std::{marker::PhantomData, ops::Mul};

use num::{
    Float, One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{
    Epsilon, GeometricProduct,
    geometry3d::{
        Bivector, Evenvector, Multivector, Point, Trivector, UnitBivector, UnitVector, Vector,
    },
};

impl<T, M> Vector<T, M> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector {
            x,
            y,
            z,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Bivector<T, M> {
    pub fn new(yz: T, zx: T, xy: T) -> Self {
        Bivector {
            yz,
            zx,
            xy,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Trivector<T, M> {
    pub fn new(xyz: T) -> Self {
        Trivector {
            xyz,
            _metric: PhantomData,
        }
    }
}

impl<T> Point<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point(Vector::new(x, y, z))
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
            z: T::zero(),
            _metric: PhantomData,
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl<T, M> Zero for Bivector<T, M>
where
    T: Zero,
{
    fn zero() -> Self {
        Bivector {
            yz: T::zero(),
            zx: T::zero(),
            xy: T::zero(),
            _metric: PhantomData,
        }
    }

    fn is_zero(&self) -> bool {
        self.yz.is_zero() && self.zx.is_zero() && self.xy.is_zero()
    }
}

impl<T, M> Zero for Trivector<T, M>
where
    T: Zero,
{
    fn zero() -> Self {
        Trivector {
            xyz: T::zero(),
            _metric: PhantomData,
        }
    }

    fn is_zero(&self) -> bool {
        self.xyz.is_zero()
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
            t: Trivector::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.s.is_zero() && self.v.is_zero() && self.b.is_zero() && self.t.is_zero()
    }
}
impl<T, M> ConstZero for Vector<T, M>
where
    T: ConstZero,
{
    const ZERO: Self = Vector {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ZERO,
        _metric: PhantomData,
    };
}

impl<T, M> ConstZero for Bivector<T, M>
where
    T: ConstZero,
{
    const ZERO: Self = Bivector {
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ZERO,
        _metric: PhantomData,
    };
}

impl<T, M> ConstZero for Trivector<T, M>
where
    T: ConstZero,
{
    const ZERO: Self = Trivector {
        xyz: T::ZERO,
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
        t: Trivector::ZERO,
    };
}

impl<T, M> One for Evenvector<T, M>
where
    T: Zero,
    T: One,
    Evenvector<T, M>: GeometricProduct<Evenvector<T, M>, Output = Evenvector<T, M>>,
    Evenvector<T, M>: Mul<Evenvector<T>, Output = Evenvector<T, M>>,
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
            t: Trivector::zero(),
        }
    }
}

impl<T, M> ConstOne for Evenvector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: ConstOne,
    Evenvector<T, M>: One,
    Evenvector<T, M>: Mul<Evenvector<T>, Output = Evenvector<T>>,
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
        t: Trivector::ZERO,
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
        z: T::ZERO,
        _metric: PhantomData,
    };

    /// Unit vector in the Y direction
    pub const Y: Self = Vector {
        x: T::ZERO,
        y: T::ONE,
        z: T::ZERO,
        _metric: PhantomData,
    };

    /// Unit vector in the Z direction
    pub const Z: Self = Vector {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ONE,
        _metric: PhantomData,
    };
}

impl<T, M> Bivector<T, M>
where
    T: ConstZero,
    T: ConstOne,
{
    /// Unit bivector in the YZ plane
    pub const YZ: Self = Bivector {
        yz: T::ONE,
        zx: T::ZERO,
        xy: T::ZERO,
        _metric: PhantomData,
    };

    /// Unit bivector in the ZX plane
    pub const ZX: Self = Bivector {
        yz: T::ZERO,
        zx: T::ONE,
        xy: T::ZERO,
        _metric: PhantomData,
    };

    /// Unit bivector in the XY plane
    pub const XY: Self = Bivector {
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ONE,
        _metric: PhantomData,
    };
}

impl<T, M> Trivector<T, M>
where
    T: ConstOne,
{
    /// Unit trivector
    pub const XYZ: Self = Trivector {
        xyz: T::ONE,
        _metric: PhantomData,
    };
}

impl<T, M> Evenvector<T, M>
where
    T: ConstZero,
    T: ConstOne,
{
    /// Unit bivector in the YZ plane
    pub const YZ: Self = Evenvector {
        s: T::ZERO,
        b: Bivector::YZ,
    };

    /// Unit bivector in the ZX plane
    pub const ZX: Self = Evenvector {
        s: T::ZERO,
        b: Bivector::ZX,
    };

    /// Unit bivector in the XY plane
    pub const XY: Self = Evenvector {
        s: T::ZERO,
        b: Bivector::XY,
    };
}

impl<T> Multivector<T>
where
    T: ConstZero,
    T: ConstOne,
{
    /// Unit vector in the X direction
    pub const X: Self = Multivector {
        s: T::ZERO,
        v: Vector::X,
        b: Bivector::ZERO,
        t: Trivector::ZERO,
    };

    /// Unit vector in the Y direction
    pub const Y: Self = Multivector {
        s: T::ZERO,
        v: Vector::Y,
        b: Bivector::ZERO,
        t: Trivector::ZERO,
    };

    /// Unit vector in the Z direction
    pub const Z: Self = Multivector {
        s: T::ZERO,
        v: Vector::Z,
        b: Bivector::ZERO,
        t: Trivector::ZERO,
    };

    /// Unit bivector in the YZ plane
    pub const YZ: Self = Multivector {
        s: T::ZERO,
        v: Vector::ZERO,
        b: Bivector::YZ,
        t: Trivector::ZERO,
    };

    /// Unit bivector in the ZX plane
    pub const ZX: Self = Multivector {
        s: T::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZX,
        t: Trivector::ZERO,
    };

    /// Unit bivector in the XY plane
    pub const XY: Self = Multivector {
        s: T::ZERO,
        v: Vector::ZERO,
        b: Bivector::XY,
        t: Trivector::ZERO,
    };

    /// Unit trivector
    pub const XYZ: Self = Multivector {
        s: T::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZERO,
        t: Trivector::XYZ,
    };
}

impl<T> Point<T>
where
    T: ConstZero,
{
    /// Coordinate origin
    pub const ORIGIN: Self = Point(Vector::ZERO);
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

    /// Unit vector in the Z direction
    pub const Z: Self = UnitVector(Vector::Z);
}

impl<T> UnitBivector<T>
where
    T: ConstZero,
    T: ConstOne,
{
    /// Unit bivector in the YZ plane
    pub const YZ: Self = UnitBivector(Bivector::YZ);

    /// Unit bivector in the ZX plane
    pub const ZX: Self = UnitBivector(Bivector::ZX);

    /// Unit bivector in the XY plane
    pub const XY: Self = UnitBivector(Bivector::XY);
}

impl<T> From<UnitVector<T>> for Vector<T> {
    fn from(value: UnitVector<T>) -> Self {
        value.0
    }
}

impl<T> TryFrom<Vector<T>> for UnitVector<T>
where
    T: Float,
    T: Epsilon,
{
    type Error = ();
    fn try_from(value: Vector<T>) -> Result<Self, Self::Error> {
        let len2 = value.x * value.x + value.y * value.y + value.z * value.z;
        if len2.is_near_zero() {
            Err(())
        } else {
            let invlen = len2.sqrt().recip();
            Ok(UnitVector(value * invlen))
        }
    }
}

impl<T> From<UnitBivector<T>> for Bivector<T> {
    fn from(value: UnitBivector<T>) -> Self {
        value.0
    }
}

impl<T> TryFrom<Bivector<T>> for UnitBivector<T>
where
    T: Float,
    T: Epsilon,
{
    type Error = ();
    fn try_from(value: Bivector<T>) -> Result<Self, Self::Error> {
        let len2 = value.yz * value.yz + value.zx * value.zx + value.xy * value.xy;
        if len2.is_near_zero() {
            Err(())
        } else {
            let invlen = len2.sqrt().recip();
            Ok(UnitBivector(value * invlen))
        }
    }
}

impl<T> From<Point<T>> for Vector<T> {
    fn from(value: Point<T>) -> Self {
        value.0
    }
}

impl<T> From<Vector<T>> for Point<T> {
    fn from(value: Vector<T>) -> Self {
        Point(value)
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
