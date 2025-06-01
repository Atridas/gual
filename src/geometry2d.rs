use std::marker::PhantomData;

use crate::Euclidean;

mod add;
mod antiwedge;
mod complement;
mod copyclone;
mod initialization;
mod mul;
mod neg;
mod sub;
mod vector_space;
mod wedge;

//mod bivector2d;
//mod multivector2d;
//mod scalar2d;
//mod vector2d;

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
