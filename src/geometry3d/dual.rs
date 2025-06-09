use std::marker::PhantomData;

use num::traits::ConstZero;

use crate::{
    Complement, Dual, Projective,
    geometry3d::{Bivector, Trivector, Vector},
};

// ----------------------------------------------------------------------------------------------------
// Euclidean metric
// ----------------------------------------------------------------------------------------------------

impl<T> Dual for Vector<T>
where
    Vector<T>: Complement,
{
    type AntiKVector = <Vector<T> as Complement>::Output;

    fn right_bulk_dual(&self) -> Self::AntiKVector {
        self.right_complement()
    }
    fn right_weight_dual(&self) -> Self::AntiKVector {
        self.right_complement()
    }
    fn left_bulk_dual(&self) -> Self::AntiKVector {
        self.left_complement()
    }
    fn left_weight_dual(&self) -> Self::AntiKVector {
        self.left_complement()
    }
}

impl<T> Dual for Bivector<T>
where
    Bivector<T>: Complement,
{
    type AntiKVector = <Bivector<T> as Complement>::Output;

    fn right_bulk_dual(&self) -> Self::AntiKVector {
        self.right_complement()
    }
    fn right_weight_dual(&self) -> Self::AntiKVector {
        self.right_complement()
    }
    fn left_bulk_dual(&self) -> Self::AntiKVector {
        self.left_complement()
    }
    fn left_weight_dual(&self) -> Self::AntiKVector {
        self.left_complement()
    }
}

impl<T> Dual for Trivector<T>
where
    Trivector<T>: Complement,
{
    type AntiKVector = <Trivector<T> as Complement>::Output;

    fn right_bulk_dual(&self) -> Self::AntiKVector {
        self.right_complement()
    }
    fn right_weight_dual(&self) -> Self::AntiKVector {
        self.right_complement()
    }
    fn left_bulk_dual(&self) -> Self::AntiKVector {
        self.left_complement()
    }
    fn left_weight_dual(&self) -> Self::AntiKVector {
        self.left_complement()
    }
}

// ----------------------------------------------------------------------------------------------------
// Projective metric
// ----------------------------------------------------------------------------------------------------

impl<T> Dual for Vector<T, Projective>
where
    T: Copy,
    T: ConstZero,
{
    type AntiKVector = Bivector<T, Projective>;

    fn right_bulk_dual(&self) -> Bivector<T, Projective> {
        Bivector {
            yz: self.x,
            zx: self.y,
            xy: T::ZERO,
            _metric: PhantomData,
        }
    }
    fn right_weight_dual(&self) -> Bivector<T, Projective> {
        Bivector {
            yz: T::ZERO,
            zx: T::ZERO,
            xy: self.z,
            _metric: PhantomData,
        }
    }
    fn left_bulk_dual(&self) -> Bivector<T, Projective> {
        Bivector {
            yz: self.x,
            zx: self.y,
            xy: T::ZERO,
            _metric: PhantomData,
        }
    }
    fn left_weight_dual(&self) -> Bivector<T, Projective> {
        Bivector {
            yz: T::ZERO,
            zx: T::ZERO,
            xy: self.z,
            _metric: PhantomData,
        }
    }
}

impl<T> Dual for Bivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
{
    type AntiKVector = Vector<T, Projective>;

    fn right_bulk_dual(&self) -> Vector<T, Projective> {
        Vector {
            x: T::ZERO,
            y: T::ZERO,
            z: self.xy,
            _metric: PhantomData,
        }
    }
    fn right_weight_dual(&self) -> Vector<T, Projective> {
        Vector {
            x: self.yz,
            y: self.zx,
            z: T::ZERO,
            _metric: PhantomData,
        }
    }
    fn left_bulk_dual(&self) -> Vector<T, Projective> {
        Vector {
            x: T::ZERO,
            y: T::ZERO,
            z: self.xy,
            _metric: PhantomData,
        }
    }
    fn left_weight_dual(&self) -> Vector<T, Projective> {
        Vector {
            x: self.yz,
            y: self.zx,
            z: T::ZERO,
            _metric: PhantomData,
        }
    }
}

impl<T> Dual for Trivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
{
    type AntiKVector = T;

    fn right_bulk_dual(&self) -> T {
        T::ZERO
    }
    fn right_weight_dual(&self) -> T {
        self.xyz
    }
    fn left_bulk_dual(&self) -> T {
        T::ZERO
    }
    fn left_weight_dual(&self) -> T {
        self.xyz
    }
}
