use std::{
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

use num::traits::ConstZero;

use crate::{GeometricProduct, WedgeProduct, geometric_with_scalar_metric, geometry3d::Trivector};

use super::{Bivector, Evenvector, Multivector, UnitVector, Vector};

// ----------------------------------------------------------------------------------------------------
// Macros
// ----------------------------------------------------------------------------------------------------

macro_rules! unit_vector_geometric {
    ($lht:ident) => {
        impl<T> GeometricProduct<UnitVector<T>> for $lht<T>
        where
            $lht<T>: GeometricProduct<Vector<T>>,
        {
            type Output = <$lht<T> as GeometricProduct<Vector<T>>>::Output;

            fn geometric_product(&self, rhs: &UnitVector<T>) -> Self::Output {
                self.geometric_product(&rhs.0)
            }
        }
    };
}

// ----------------------------------------------------------------------------------------------------
// Scalar
// ----------------------------------------------------------------------------------------------------

impl<T: Copy, M> GeometricProduct<Vector<T, M>> for T
where
    T: Mul<Output = T>,
{
    type Output = Vector<T, M>;
    fn geometric_product(&self, rhs: &Vector<T, M>) -> Self::Output {
        Vector {
            x: *self * rhs.x,
            y: *self * rhs.y,
            z: *self * rhs.z,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy> GeometricProduct<UnitVector<T>> for T
where
    T: Mul<Output = T>,
{
    type Output = Vector<T>;
    fn geometric_product(&self, rhs: &UnitVector<T>) -> Self::Output {
        Vector {
            x: *self * rhs.0.x,
            y: *self * rhs.0.y,
            z: *self * rhs.0.z,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy, M> GeometricProduct<Bivector<T, M>> for T
where
    T: Mul<Output = T>,
{
    type Output = Bivector<T, M>;
    fn geometric_product(&self, rhs: &Bivector<T, M>) -> Self::Output {
        Bivector {
            yz: *self * rhs.yz,
            zx: *self * rhs.zx,
            xy: *self * rhs.xy,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy, M> GeometricProduct<Trivector<T, M>> for T
where
    T: Mul<Output = T>,
{
    type Output = Trivector<T, M>;
    fn geometric_product(&self, rhs: &Trivector<T, M>) -> Self::Output {
        Trivector {
            xyz: *self * rhs.xyz,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy, M> GeometricProduct<Evenvector<T, M>> for T
where
    T: Mul<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn geometric_product(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        Evenvector {
            s: *self * rhs.s,
            b: self.geometric_product(&rhs.b),
        }
    }
}

impl<T: Copy, M> GeometricProduct<Multivector<T, M>> for T
where
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;
    fn geometric_product(&self, rhs: &Multivector<T, M>) -> Self::Output {
        Multivector {
            s: *self * rhs.s,
            v: self.geometric_product(&rhs.v),
            b: self.geometric_product(&rhs.b),
            t: self.geometric_product(&rhs.t),
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Vector
// ----------------------------------------------------------------------------------------------------

geometric_with_scalar_metric!(Vector);

impl<T> GeometricProduct<Vector<T>> for Vector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T>;

    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        Evenvector {
            s: self.x * rhs.x + self.y * rhs.y + self.z * rhs.z,
            b: self.wedge(rhs),
        }
    }
}

unit_vector_geometric!(Vector);

impl<T> GeometricProduct<Bivector<T>> for Vector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: Vector {
                x: self.z * rhs.zx - self.y * rhs.xy,
                y: self.x * rhs.xy - self.z * rhs.yz,
                z: self.y * rhs.yz - self.x * rhs.zx,
                _metric: PhantomData,
            },
            b: Bivector::ZERO,
            t: self.wedge(rhs),
        }
    }
}

impl<T> GeometricProduct<Trivector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Bivector<T>;

    fn geometric_product(&self, rhs: &Trivector<T>) -> Self::Output {
        Bivector {
            yz: self.x * rhs.xyz,
            zx: self.y * rhs.xyz,
            xy: self.z * rhs.xyz,
            _metric: PhantomData,
        }
    }
}

impl<T> GeometricProduct<Evenvector<T>> for Vector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
        let a = self.geometric_product(&rhs.s);
        let c = Vector {
            x: self.z * rhs.b.zx - self.y * rhs.b.xy,
            y: self.x * rhs.b.xy - self.z * rhs.b.yz,
            z: self.y * rhs.b.yz - self.x * rhs.b.zx,
            _metric: PhantomData,
        };
        let d = self.wedge(&rhs.b);

        Multivector {
            s: T::ZERO,
            v: a + c,
            b: Bivector::ZERO,
            t: d,
        }
    }
}

impl<T> GeometricProduct<Multivector<T>> for Vector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Multivector<T>) -> Self::Output {
        let a = self.geometric_product(&rhs.s);
        let b = self.geometric_product(&rhs.v);
        let c = Vector {
            x: self.z * rhs.b.zx - self.y * rhs.b.xy,
            y: self.x * rhs.b.xy - self.z * rhs.b.yz,
            z: self.y * rhs.b.yz - self.x * rhs.b.zx,
            _metric: PhantomData,
        };
        let d = self.wedge(&rhs.b);
        let e = self.geometric_product(&rhs.t);

        Multivector {
            s: b.s,
            v: a + c,
            b: b.b + e,
            t: d,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// UnitVector
// ----------------------------------------------------------------------------------------------------

impl<T, Rhs> GeometricProduct<Rhs> for UnitVector<T>
where
    Vector<T>: GeometricProduct<Rhs>,
{
    type Output = <Vector<T> as GeometricProduct<Rhs>>::Output;

    fn geometric_product(&self, rhs: &Rhs) -> Self::Output {
        self.0.geometric_product(rhs)
    }
}

// ----------------------------------------------------------------------------------------------------
// Bivector
// ----------------------------------------------------------------------------------------------------

geometric_with_scalar_metric!(Bivector);

impl<T> GeometricProduct<Vector<T>> for Bivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: Vector {
                x: self.xy * rhs.y - self.zx * rhs.z,
                y: self.yz * rhs.z - self.xy * rhs.x,
                z: self.zx * rhs.x - self.yz * rhs.y,
                _metric: PhantomData,
            },
            b: Bivector::ZERO,
            t: self.wedge(rhs),
        }
    }
}

unit_vector_geometric!(Bivector);

impl<T> GeometricProduct<Bivector<T>> for Bivector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T>;

    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        Evenvector {
            s: -(self.yz * rhs.yz + self.zx * rhs.zx + self.xy * rhs.xy),
            b: Bivector {
                yz: self.xy * rhs.zx - self.zx * rhs.xy,
                zx: self.yz * rhs.xy - self.xy * rhs.yz,
                xy: self.zx * rhs.yz - self.yz * rhs.zx,
                _metric: PhantomData,
            },
        }
    }
}

impl<T> GeometricProduct<Trivector<T>> for Bivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Vector<T>;

    fn geometric_product(&self, rhs: &Trivector<T>) -> Self::Output {
        Vector {
            x: self.yz * -rhs.xyz,
            y: self.zx * -rhs.xyz,
            z: self.xy * -rhs.xyz,
            _metric: PhantomData,
        }
    }
}

impl<T> GeometricProduct<Evenvector<T>> for Bivector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T>;

    fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
        self.geometric_product(&rhs.s) + self.geometric_product(&rhs.b)
    }
}

impl<T> GeometricProduct<Multivector<T>> for Bivector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Multivector<T>) -> Self::Output {
        let a = self.geometric_product(&rhs.s);
        let b = Vector {
            x: self.xy * rhs.v.y - self.zx * rhs.v.z,
            y: self.yz * rhs.v.z - self.xy * rhs.v.x,
            z: self.zx * rhs.v.x - self.yz * rhs.v.y,
            _metric: PhantomData,
        };
        let c = self.wedge(&rhs.v);
        let d = self.geometric_product(&rhs.b);
        let e = self.geometric_product(&rhs.t);

        Multivector {
            s: d.s,
            v: b + e,
            b: a + d.b,
            t: c,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Trivector
// ----------------------------------------------------------------------------------------------------

geometric_with_scalar_metric!(Trivector);

impl<T> GeometricProduct<Vector<T>> for Trivector<T>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Bivector<T>;

    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        rhs.geometric_product(self)
    }
}

unit_vector_geometric!(Trivector);

impl<T> GeometricProduct<Bivector<T>> for Trivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Vector<T>;

    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        rhs.geometric_product(self)
    }
}

impl<T> GeometricProduct<Trivector<T>> for Trivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = T;

    fn geometric_product(&self, rhs: &Trivector<T>) -> Self::Output {
        -self.xyz * rhs.xyz
    }
}

impl<T> GeometricProduct<Evenvector<T>> for Trivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: self.geometric_product(&rhs.b),
            b: Bivector::ZERO,
            t: self.geometric_product(&rhs.s),
        }
    }
}

impl<T> GeometricProduct<Multivector<T>> for Trivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Multivector<T>) -> Self::Output {
        Multivector {
            s: self.geometric_product(&rhs.t),
            v: self.geometric_product(&rhs.b),
            b: self.geometric_product(&rhs.v),
            t: self.geometric_product(&rhs.s),
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Evenvector
// ----------------------------------------------------------------------------------------------------

geometric_with_scalar_metric!(Evenvector);

impl<T> GeometricProduct<Vector<T>> for Evenvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        let a = self.s.geometric_product(rhs);
        let b = Vector {
            x: self.b.xy * rhs.y - self.b.zx * rhs.z,
            y: self.b.yz * rhs.z - self.b.xy * rhs.x,
            z: self.b.zx * rhs.x - self.b.yz * rhs.y,
            _metric: PhantomData,
        };
        let c = self.b.wedge(rhs);

        Multivector {
            s: T::ZERO,
            v: a + b,
            b: Bivector::ZERO,
            t: c,
        }
    }
}

unit_vector_geometric!(Evenvector);

impl<T> GeometricProduct<Bivector<T>> for Evenvector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T>;

    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        self.s.geometric_product(rhs) + self.b.geometric_product(rhs)
    }
}

impl<T> GeometricProduct<Trivector<T>> for Evenvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Trivector<T>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: self.b.geometric_product(rhs),
            b: Bivector::ZERO,
            t: self.s.geometric_product(rhs),
        }
    }
}

impl<T> GeometricProduct<Multivector<T>> for Evenvector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Neg<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Multivector<T>) -> Self::Output {
        let a = self.s.geometric_product(rhs);
        let b = self.b.geometric_product(&rhs.s);
        let c = Vector {
            x: self.b.xy * rhs.v.y - self.b.zx * rhs.v.z,
            y: self.b.yz * rhs.v.z - self.b.xy * rhs.v.x,
            z: self.b.zx * rhs.v.x - self.b.yz * rhs.v.y,
            _metric: PhantomData,
        };
        let d = self.b.wedge(&rhs.v);
        let e = self.b.geometric_product(&rhs.t);

        Multivector {
            s: a.s,
            v: a.v + c + e,
            b: a.b + b,
            t: a.t + d,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Multivector
// ----------------------------------------------------------------------------------------------------

geometric_with_scalar_metric!(Multivector);

impl<T> GeometricProduct<Vector<T>> for Multivector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        let a = self.s.geometric_product(rhs);
        let b = self.v.geometric_product(rhs);
        let c = Vector {
            x: self.b.xy * rhs.y - self.b.zx * rhs.z,
            y: self.b.yz * rhs.z - self.b.xy * rhs.x,
            z: self.b.zx * rhs.x - self.b.yz * rhs.y,
            _metric: PhantomData,
        };
        let d = self.b.wedge(rhs);
        let e = self.t.geometric_product(rhs);

        Multivector {
            s: b.s,
            v: a + c,
            b: b.b + e,
            t: d,
        }
    }
}

unit_vector_geometric!(Multivector);

impl<T> GeometricProduct<Bivector<T>> for Multivector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        let a = self.s.geometric_product(rhs);
        let b = Vector {
            x: self.v.z * rhs.zx - self.v.y * rhs.xy,
            y: self.v.x * rhs.xy - self.v.z * rhs.yz,
            z: self.v.y * rhs.yz - self.v.x * rhs.zx,
            _metric: PhantomData,
        };
        let c = self.v.wedge(rhs);
        let d = self.b.geometric_product(rhs);
        let e = self.t.geometric_product(rhs);

        Multivector {
            s: d.s,
            v: b + e,
            b: a + d.b,
            t: c,
        }
    }
}

impl<T> GeometricProduct<Trivector<T>> for Multivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Trivector<T>) -> Self::Output {
        Multivector {
            s: self.t.geometric_product(rhs),
            v: self.b.geometric_product(rhs),
            b: self.v.geometric_product(rhs),
            t: self.s.geometric_product(rhs),
        }
    }
}

impl<T> GeometricProduct<Evenvector<T>> for Multivector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Neg<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
        let a = self.s.geometric_product(rhs);
        let b = self.geometric_product(&rhs.s);
        let c = self.v.wedge(&rhs.b);
        let d = Vector {
            x: self.v.z * rhs.b.zx - self.v.y * rhs.b.xy,
            y: self.v.x * rhs.b.xy - self.v.z * rhs.b.yz,
            z: self.v.y * rhs.b.yz - self.v.x * rhs.b.zx,
            _metric: PhantomData,
        };
        let e = self.b.geometric_product(&rhs.b);
        let f = self.t.geometric_product(&rhs.b);

        Multivector {
            s: a.s + b.s + e.s,
            v: b.v + d + f,
            b: a.b + b.b + e.b,
            t: b.t + c,
        }
    }
}

impl<T> GeometricProduct<Multivector<T>> for Multivector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Neg<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Multivector<T>) -> Self::Output {
        self.geometric_product(&rhs.s)
            + self.geometric_product(&rhs.v)
            + self.geometric_product(&rhs.b)
            + self.geometric_product(&rhs.t)
    }
}
