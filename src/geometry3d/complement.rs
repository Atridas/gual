use std::marker::PhantomData;

use num::traits::ConstZero;

use crate::{
    Complement, Scalar,
    geometry3d::{Bivector, Evenvector, Multivector, Trivector, Vector},
};

impl<T, M> Complement for Scalar<3, T, M>
where
    T: Copy,
{
    type Output = Trivector<T, M>;

    fn right_complement(&self) -> Self::Output {
        Trivector {
            xyz: self.0,
            _metric: PhantomData,
        }
    }

    fn left_complement(&self) -> Self::Output {
        Trivector {
            xyz: self.0,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Complement for Vector<T, M>
where
    T: Copy,
{
    type Output = Bivector<T, M>;

    fn right_complement(&self) -> Self::Output {
        Bivector {
            yz: self.x,
            zx: self.y,
            xy: self.z,
            _metric: PhantomData,
        }
    }

    fn left_complement(&self) -> Self::Output {
        Bivector {
            yz: self.x,
            zx: self.y,
            xy: self.z,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy, M> Complement for Bivector<T, M>
where
    T: Copy,
{
    type Output = Vector<T, M>;

    fn right_complement(&self) -> Self::Output {
        Vector {
            x: self.yz,
            y: self.zx,
            z: self.xy,
            _metric: PhantomData,
        }
    }

    fn left_complement(&self) -> Self::Output {
        Vector {
            x: self.yz,
            y: self.zx,
            z: self.xy,
            _metric: PhantomData,
        }
    }
}

impl<T: Clone, M> Complement for Trivector<T, M> {
    type Output = T;

    fn right_complement(&self) -> Self::Output {
        self.xyz.clone()
    }

    fn left_complement(&self) -> Self::Output {
        self.xyz.clone()
    }
}

impl<T, M> Complement for Evenvector<T, M>
where
    T: Copy,
    T: ConstZero,
{
    type Output = Multivector<T, M>;

    fn right_complement(&self) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: self.b.right_complement(),
            b: Bivector::ZERO,
            t: Scalar::<3, T, M>::new(self.s).right_complement(),
        }
    }

    fn left_complement(&self) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: self.b.left_complement(),
            b: Bivector::ZERO,
            t: Scalar::<3, T, M>::new(self.s).left_complement(),
        }
    }
}

impl<T, M> Complement for Multivector<T, M>
where
    T: Copy,
    T: ConstZero,
{
    type Output = Multivector<T, M>;

    fn right_complement(&self) -> Self::Output {
        Multivector {
            s: self.t.right_complement(),
            v: self.b.right_complement(),
            b: self.v.right_complement(),
            t: Scalar::<3, T, M>::new(self.s).right_complement(),
        }
    }

    fn left_complement(&self) -> Self::Output {
        Multivector {
            s: self.t.left_complement(),
            v: self.b.left_complement(),
            b: self.v.left_complement(),
            t: Scalar::<3, T, M>::new(self.s).left_complement(),
        }
    }
}
