use std::marker::PhantomData;

use crate::Euclidean;

mod angle3d;
mod scalar3d;

mod add;
mod antiwedge;
mod complement;
mod copyclone;
mod div;
mod dual;
mod geometric_product;
mod initialization;
mod metric;
mod mul;
mod neg;
mod projective_geometric_product;
mod sub;
mod vector_space;
mod wedge;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar<T>(pub T);

#[derive(Debug, PartialEq, Eq)]
pub struct Vector<T, M = Euclidean> {
    pub x: T,
    pub y: T,
    pub z: T,
    _metric: PhantomData<M>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T>(pub Vector<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitVector<T>(pub(super) Vector<T>);

#[derive(Debug, PartialEq, Eq)]
pub struct Bivector<T, M = Euclidean> {
    pub yz: T,
    pub zx: T,
    pub xy: T,
    _metric: PhantomData<M>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitBivector<T>(pub(super) Bivector<T>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Trivector<T, M = Euclidean> {
    pub xyz: T,
    _metric: PhantomData<M>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multivector<T, M = Euclidean> {
    pub s: T,
    pub v: Vector<T, M>,
    pub b: Bivector<T, M>,
    pub t: Trivector<T, M>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Evenvector<T, M = Euclidean> {
    pub s: T,
    pub b: Bivector<T, M>,
}
