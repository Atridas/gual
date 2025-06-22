use std::marker::PhantomData;

use crate::Euclidean;

mod add;
mod angle;
mod antiwedge;
mod complement;
mod contraction;
mod copyclone;
mod div;
mod dot;
mod dual;
mod geometric_product;
mod initialization;
mod metric;
mod mul;
mod neg;
mod norm;
mod sub;
mod unitize;
mod vector_space;
mod wedge;

#[derive(Debug, PartialEq, Eq)]
pub struct Vector<T, M = Euclidean> {
    pub x: T,
    pub y: T,
    _metric: PhantomData<M>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bivector<T, M = Euclidean> {
    pub xy: T,
    _metric: PhantomData<M>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Evenvector<T, M = Euclidean> {
    pub s: T,
    pub b: Bivector<T, M>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multivector<T, M = Euclidean> {
    pub s: T,
    pub v: Vector<T, M>,
    pub b: Bivector<T, M>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitVector<T>(Vector<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T>(pub Vector<T>);
