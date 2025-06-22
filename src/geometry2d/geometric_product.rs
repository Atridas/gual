use std::{
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

use crate::{
    GeometricProduct, geometric_with_scalar_metric, reverse_geometric, reverse_geometric_metric,
};

use super::{Bivector, Evenvector, Multivector, UnitVector, Vector};

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
            xy: *self * rhs.xy,
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
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Vector
// ----------------------------------------------------------------------------------------------------

geometric_with_scalar_metric!(Vector);

impl<T: Copy> GeometricProduct<Vector<T>> for Vector<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T>;
    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        Evenvector {
            s: self.x * rhs.x + self.y * rhs.y,
            b: Bivector {
                xy: self.x * rhs.y - self.y * rhs.x,
                _metric: PhantomData,
            },
        }
    }
}

impl<T: Copy> GeometricProduct<UnitVector<T>> for Vector<T>
where
    Vector<T>: GeometricProduct<Vector<T>, Output = Evenvector<T>>,
{
    type Output = Evenvector<T>;
    fn geometric_product(&self, rhs: &UnitVector<T>) -> Self::Output {
        self.geometric_product(&rhs.0)
    }
}

impl<T: Copy> GeometricProduct<Bivector<T>> for Vector<T>
where
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Vector<T>;
    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        Vector {
            x: -(self.y * rhs.xy),
            y: self.x * rhs.xy,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy> GeometricProduct<Evenvector<T>> for Vector<T>
where
    T: Add<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Vector<T>;
    fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
        *self * rhs.s + self.geometric_product(&rhs.b)
    }
}

impl<T: Copy> GeometricProduct<Multivector<T>> for Vector<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;
    fn geometric_product(&self, rhs: &Multivector<T>) -> Self::Output {
        let ev = self.geometric_product(&rhs.v);
        Multivector {
            s: ev.s,
            v: *self * rhs.s + self.geometric_product(&rhs.b),
            b: ev.b,
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

reverse_geometric_metric!(Bivector, Vector);

reverse_geometric!(Bivector, UnitVector);

impl<T: Copy> GeometricProduct<Bivector<T>> for Bivector<T>
where
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = T;
    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        -(self.xy * rhs.xy)
    }
}

impl<T: Copy> GeometricProduct<Evenvector<T>> for Bivector<T>
where
    T: Add<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T>;
    fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
        *self * rhs.s + self.geometric_product(&rhs.b)
    }
}

impl<T: Copy> GeometricProduct<Multivector<T>> for Bivector<T>
where
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;
    fn geometric_product(&self, rhs: &Multivector<T>) -> Self::Output {
        Multivector {
            s: self.geometric_product(&rhs.b),
            v: self.geometric_product(&rhs.v),
            b: *self * rhs.s,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Evenvector
// ----------------------------------------------------------------------------------------------------

geometric_with_scalar_metric!(Evenvector);

reverse_geometric_metric!(Evenvector, Vector);

reverse_geometric!(Evenvector, UnitVector);

reverse_geometric_metric!(Evenvector, Bivector);

impl<T: Copy> GeometricProduct<Evenvector<T>> for Evenvector<T>
where
    T: Add<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T>;
    fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
        *self * rhs.s + self.geometric_product(&rhs.b)
    }
}

impl<T: Copy> GeometricProduct<Multivector<T>> for Evenvector<T>
where
    T: Add<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;
    fn geometric_product(&self, rhs: &Multivector<T>) -> Self::Output {
        self.s.geometric_product(rhs) + self.b.geometric_product(rhs)
    }
}

// ----------------------------------------------------------------------------------------------------
// Multivector
// ----------------------------------------------------------------------------------------------------

geometric_with_scalar_metric!(Multivector);

impl<T: Copy> GeometricProduct<Vector<T>> for Multivector<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;
    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        let ev = self.v.geometric_product(rhs);
        Multivector {
            s: ev.s,
            v: self.s.geometric_product(rhs) + self.b.geometric_product(rhs),
            b: ev.b,
        }
    }
}

impl<T: Copy> GeometricProduct<UnitVector<T>> for Multivector<T>
where
    Multivector<T>: GeometricProduct<Vector<T>, Output = Multivector<T>>,
{
    type Output = Multivector<T>;
    fn geometric_product(&self, rhs: &UnitVector<T>) -> Self::Output {
        self.geometric_product(&rhs.0)
    }
}

reverse_geometric_metric!(Multivector, Bivector);

reverse_geometric_metric!(Multivector, Evenvector);

impl<T: Copy> GeometricProduct<Multivector<T>> for Multivector<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T>;
    fn geometric_product(&self, rhs: &Multivector<T>) -> Self::Output {
        self.s.geometric_product(rhs)
            + self.v.geometric_product(rhs)
            + self.b.geometric_product(rhs)
    }
}
