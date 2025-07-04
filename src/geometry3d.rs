//! Module that contains the code of 3D geometric algebra. It has implementations for both
//! euclidean geometry and projective geometry (the later will actually create 2D objects!)
//!
//! The canonical basis of the algebra is: `x`, `y`, `z`, `yz`, `zx`, `xy`, `xyz`.
//!
//! ## Complements ##
//!
//! |                  |  1  | x  | y  | z  | yz | zx | xy | xyz |
//! | ---------------- | --- | -- | -- | -- | -- | -- | -- | --- |
//! | right complement | xyz | yz | zx | xy |  x |  y |  z |  1  |
//! | left complement  | xyz | yz | zx | xy |  x |  y |  z |  1  |
//!
//! ## Wedge product ##
//!
//! | `^` |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |  1  |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! |  x  |  x  |  0  | xy  | -zx | xyz |  0  |  0  |  0  |
//! |  y  |  y  | -xy |  0  |  yz |  0  | xyz |  0  |  0  |
//! |  z  |  z  |  zx | -yz |  0  |  0  |  0  | xyz |  0  |
//! |  yz |  yz | xyz |  0  |  0  |  0  |  0  |  0  |  0  |
//! |  zx |  zx |  0  | xyz |  0  |  0  |  0  |  0  |  0  |
//! |  xy |  xy |  0  |  0  | xyz |  0  |  0  |  0  |  0  |
//! | xyz | xyz |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//!
//! ## Antiwedge product ##
//!
//! | `v` |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |  1  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  1  |
//! |  x  |  0  |  0  |  0  |  0  |  1  |  0  |  0  |  x  |
//! |  y  |  0  |  0  |  0  |  0  |  0  |  1  |  0  |  y  |
//! |  z  |  0  |  0  |  0  |  0  |  0  |  0  |  1  |  z  |
//! |  yz |  0  |  1  |  0  |  0  |  0  |  z  | -y  |  yz |
//! |  zx |  0  |  0  |  1  |  0  | -z  |  0  |  x  |  zx |
//! |  xy |  0  |  0  |  0  |  1  |  y  | -x  |  0  |  xy |
//! | xyz |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//!
//! ## Projective metric ##
//!
//! * bulk: `1`, `x`, `y`, `xy`
//! * weight: `z`, `yz`, `zx`, `xyz`
//!
//! ## dual ##
//!
//!
//! |  euclidean  |  1  | x  | y  | z  | yz | zx | xy | xyz |
//! | ----------- | --- | -- | -- | -- | -- | -- | -- | --- |
//! | dual        | xyz | yz | zx | xy |  x |  y |  z |  1  |
//! | weight dual | xyz | yz | zx | xy |  x |  y |  z |  1  |
//!
//! |  projective |  1  | x  | y  | z  | yz | zx | xy | xyz |
//! | ----------- | --- | -- | -- | -- | -- | -- | -- | --- |
//! | dual        | xyz | yz | zx |  0 |  0 |  0 |  z |  0  |
//! | weight dual |  0  |  0 |  0 | xy |  x |  y |  0 |  1  |
//!
//!
//! ## Contractions ##
//!
//! |  euclidean |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | ---------- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |    1       |  1  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |    x       |  x  |  1  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |    y       |  y  |  0  |  1  |  0  |  0  |  0  |  0  |  0  |
//! |    z       |  z  |  0  |  0  |  1  |  0  |  0  |  0  |  0  |
//! |   yz       |  yz |  0  |  z  | -y  |  1  |  0  |  0  |  0  |
//! |   zx       |  zx | -z  |  0  |  x  |  0  |  1  |  0  |  0  |
//! |   xy       |  xy |  y  | -x  |  0  |  0  |  0  |  1  |  0  |
//! |  xyz       | xyz |  yz |  zx |  xy |  x  |  y  |  z  |  1  |
//!
//! | projective bulk |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | --------------- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |        1        |  1  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |        x        |  x  |  1  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |        y        |  y  |  0  |  1  |  0  |  0  |  0  |  0  |  0  |
//! |        z        |  z  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |        yz       |  yz |  0  |  z  |  0  |  0  |  0  |  0  |  0  |
//! |        zx       |  zx | -z  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |        xy       |  xy |  y  | -x  |  0  |  0  |  0  |  1  |  0  |
//! |       xyz       | xyz |  yz |  zx |  0  |  0  |  0  |  z  |  0  |
//!
//! | projective weight |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | ----------------- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |          1        |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |          x        |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |          y        |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |          z        |  0  |  0  |  0  |  1  |  0  |  0  |  0  |  0  |
//! |          yz       |  0  |  0  |  0  | -y  |  1  |  0  |  0  |  0  |
//! |          zx       |  0  |  0  |  0  |  x  |  0  |  1  |  0  |  0  |
//! |          xy       |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |         xyz       |  0  |  0  |  0  |  xy |  x  |  y  |  0  |  0  |
//!
//! ## Expansions
//!
//! |  euclidean |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | ---------- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |    1       | xyz |  yz |  zx |  xy |  x  |  y  |  z  |  1  |
//! |    x       |  0  | xyz |  0  |  0  |  0  | xy  | -zx |  x  |
//! |    y       |  0  |  0  | xyz |  0  | -xy |  0  |  yz |  y  |
//! |    z       |  0  |  0  |  0  | xyz |  zx | -yz |  0  |  z  |
//! |   yz       |  0  |  0  |  0  |  0  | xyz |  0  |  0  |  yz |
//! |   zx       |  0  |  0  |  0  |  0  |  0  | xyz |  0  |  zx |
//! |   xy       |  0  |  0  |  0  |  0  |  0  |  0  | xyz |  xy |
//! |  xyz       |  0  |  0  |  0  |  0  |  0  |  0  |  0  | xyz |
//!
//! |  projective bulk |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | ---------------- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |          1       | xyz |  yz |  zx |  0  |  0  |  0  |  z  |  0  |
//! |          x       |  0  | xyz |  0  |  0  |  0  |  0  | -zx |  0  |
//! |          y       |  0  |  0  | xyz |  0  |  0  |  0  |  yz |  0  |
//! |          z       |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |         yz       |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |         zx       |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |         xy       |  0  |  0  |  0  |  0  |  0  |  0  | xyz |  0  |
//! |        xyz       |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//!
//! | projective weight |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | ----------------- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |          1        |  0  |  0  |  0  |  xy |  x  |  y  |  0  |  1  |
//! |          x        |  0  |  0  |  0  |  0  |  0  | xy  |  0  |  x  |
//! |          y        |  0  |  0  |  0  |  0  | -xy |  0  |  0  |  y  |
//! |          z        |  0  |  0  |  0  | xyz |  zx | -yz |  0  |  z  |
//! |          yz       |  0  |  0  |  0  |  0  | xyz |  0  |  0  |  yz |
//! |          zx       |  0  |  0  |  0  |  0  |  0  | xyz |  0  |  zx |
//! |          xy       |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  xy |
//! |         xyz       |  0  |  0  |  0  |  0  |  0  |  0  |  0  | xyz |
//!

use std::marker::PhantomData;

use crate::Euclidean;

mod angle_legacy;
mod scalar3d;

mod add;
mod angle;
mod antiwedge;
mod complement;
mod contraction;
mod copyclone;
mod div;
mod dot;
mod dual;
mod expansion;
mod geometric_product;
mod initialization;
mod metric;
mod mul;
mod neg;
mod norm;
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
