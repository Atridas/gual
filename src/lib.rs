mod multivector2d;
mod multivector3d;

pub use multivector2d::*;
//pub use multivector3d::Multivector3D;

pub trait Antiscalar {
    fn unit_volume() -> Self;
}

pub trait Multivector {
    type Scalar;
    type Vector;
    type Antivector;
    type Antiscalar: Antiscalar;

    fn scalar(&self) -> Self::Scalar;
    fn vector(&self) -> Self::Vector;
    fn antivector(&self) -> Self::Antivector;
    fn antiscalar(&self) -> Self::Antiscalar;

    fn unit_volume() -> Self::Antiscalar {
        Self::Antiscalar::unit_volume()
    }

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
    fn wedge(self, rhs: Rhs) -> Self::Output;
}
