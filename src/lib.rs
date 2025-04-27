use num::{Float, FromPrimitive};

pub mod geometry2d;
pub mod geometry3d;
pub mod geometry4d;

pub mod homogeneous3d;

pub trait Epsilon {
    fn eps() -> Self;
    fn is_near_zero(&self) -> bool;
}

pub trait Antiscalar {
    const UNIT_VOLUME: Self;
}

pub trait VectorSpace {
    type Scalar;
    type Vector;
    type Antivector;
    type Antiscalar: Antiscalar;

    const UNIT_VOLUME: Self::Antiscalar = Self::Antiscalar::UNIT_VOLUME;

    fn scalar(&self) -> Self::Scalar;
    fn vector(&self) -> Self::Vector;
    fn antivector(&self) -> Self::Antivector;
    fn antiscalar(&self) -> Self::Antiscalar;

    // u ^ right_complement(u) = antiscalar
    fn right_complement(&self) -> Self;
    // left_complement(u) ^ u = antiscalar
    fn left_complement(&self) -> Self;
}

pub trait KVector {
    type AntiKVector;

    // u ^ right_complement(u) = antiscalar
    fn right_complement(&self) -> Self::AntiKVector;
    // left_complement(u) ^ u = antiscalar
    fn left_complement(&self) -> Self::AntiKVector;
}

pub trait WedgeProduct<Rhs> {
    type Output;
    fn wedge(&self, rhs: Rhs) -> Self::Output;
}

pub trait AntiwedgeProduct<Rhs> {
    type Output;
    fn antiwedge(&self, rhs: Rhs) -> Self::Output;
}

pub trait Normalizable {
    type Output;
    fn normalize(&self) -> Option<Self::Output>;
}

pub trait Join<Rhs> {
    type Output;
    fn join(&self, rhs: Rhs) -> Self::Output;
}

pub trait Meet<Rhs> {
    type Output;
    fn meet(&self, rhs: Rhs) -> Self::Output;
}

pub fn antiwedge_reference<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> <<<Lhs as KVector>::AntiKVector as WedgeProduct<<Rhs as KVector>::AntiKVector>>::Output as KVector>::AntiKVector
where
    Lhs: KVector,
    Rhs: KVector,
    <Lhs as KVector>::AntiKVector: WedgeProduct<<Rhs as KVector>::AntiKVector>,
    <<Lhs as KVector>::AntiKVector as WedgeProduct<<Rhs as KVector>::AntiKVector>>::Output: KVector,
{
    lhs.left_complement()
        .wedge(rhs.left_complement())
        .right_complement()
}

#[macro_export]
macro_rules! reverse_mul {
    ($lht:ident, $rht:ident) => {
        impl<T> Mul<$rht<T>> for $lht<T>
        where
            $rht<T>: Mul<$lht<T>>,
        {
            type Output = <$rht<T> as Mul<$lht<T>>>::Output;

            fn mul(self, rhs: $rht<T>) -> Self::Output {
                rhs * self
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_wedge {
    ($lht:ident, $rht:ident) => {
        impl<T> WedgeProduct<$rht<T>> for $lht<T>
        where
            $lht<T>: Copy,
            $rht<T>: WedgeProduct<$lht<T>>,
        {
            type Output = <$rht<T> as WedgeProduct<$lht<T>>>::Output;

            fn wedge(&self, rhs: $rht<T>) -> Self::Output {
                rhs.wedge(*self)
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_antiwedge {
    ($lht:ident, $rht:ident) => {
        impl<T> AntiwedgeProduct<$rht<T>> for $lht<T>
        where
            $lht<T>: Copy,
            $rht<T>: AntiwedgeProduct<$lht<T>>,
        {
            type Output = <$rht<T> as AntiwedgeProduct<$lht<T>>>::Output;

            fn antiwedge(&self, rhs: $rht<T>) -> Self::Output {
                rhs.antiwedge(*self)
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_antiwedge_anticommutative {
    ($lht:ident, $rht:ident) => {
        impl<T> AntiwedgeProduct<$rht<T>> for $lht<T>
        where
            $rht<T>: AntiwedgeProduct<$lht<T>>,
            $lht<T>: Neg<Output = $lht<T>>,
        {
            type Output = <$rht<T> as AntiwedgeProduct<$lht<T>>>::Output;

            fn antiwedge(&self, rhs: $rht<T>) -> Self::Output {
                rhs.antiwedge(-self)
            }
        }
    };
}

impl<T: Float + FromPrimitive + Ord> Epsilon for T {
    #[inline(always)]
    fn eps() -> Self {
        T::from_f32(0.001).expect("expected T to be a floating point type")
    }

    #[inline(always)]
    fn is_near_zero(&self) -> bool {
        self.abs() < Self::eps()
    }
}
