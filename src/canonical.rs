//! This module contains canonical implementations of the basic operations
//! This implementations are for testing purposes and are not supposed to be anywhere
//! near optimal implementations, they're just for reference

use std::ops::{Div, Mul};

use crate::{
    Antiscalar, AntiwedgeProduct, Complement, Dual, Epsilon, Expansion, Metric, Norm, WedgeProduct,
};

/// The algorithm for this is: `right_complement( left_complement(lhs) ^ left_complement(rhs) )`
pub fn canonical_antiwedge<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> <<<Lhs as Complement>::Output as WedgeProduct<<Rhs as Complement>::Output>>::Output as Complement>::Output
where
    Lhs: Complement,
    Rhs: Complement,
    <Lhs as Complement>::Output: WedgeProduct<<Rhs as Complement>::Output>,
    <<Lhs as Complement>::Output as WedgeProduct<<Rhs as Complement>::Output>>::Output: Complement,
{
    lhs.left_complement()
        .wedge(&rhs.left_complement())
        .right_complement()
}

/// The algorithm for this is: `right_complement( bulk( left_complement(a) ) )`
pub fn canonical_weight<T>(a: T) -> <<T as Complement>::Output as Complement>::Output
where
    T: Complement,
    <T as Complement>::Output: Metric,
    <T as Complement>::Output: Complement,
{
    a.left_complement().proper_bulk().right_complement()
}

/// The algorithm for this is: `right_complement( bulk(a) )`
pub fn canonical_right_bulk_dual<T>(a: T) -> <T as Complement>::Output
where
    T: Metric,
    T: Complement,
{
    a.proper_bulk().right_complement()
}

/// The algorithm for this is: `left_complement( bulk(a) )`
pub fn canonical_left_bulk_dual<T>(a: T) -> <T as Complement>::Output
where
    T: Metric,
    T: Complement,
{
    a.proper_bulk().left_complement()
}

/// The algorithm for this is: `right_complement( weight(a) )`
pub fn canonical_right_weight_dual<T>(a: T) -> <T as Complement>::Output
where
    T: Metric,
    T: Complement,
{
    a.proper_weight().right_complement()
}

/// The algorithm for this is: `left_complement( weight(a) )`
pub fn canonical_left_weight_dual<T>(a: T) -> <T as Complement>::Output
where
    T: Metric,
    T: Complement,
{
    a.proper_weight().left_complement()
}

/// The algorithm for this is: `lhs.antiwedge( rhs.bulk_dual() )`
pub fn canonical_bulk_contraction<Lhs, Rhs>(
    lhs: Lhs,
    rhs: Rhs,
) -> <Lhs as AntiwedgeProduct<<Rhs as Dual>::AntiKVector>>::Output
where
    Rhs: Dual,
    Lhs: AntiwedgeProduct<<Rhs as Dual>::AntiKVector>,
{
    lhs.antiwedge(&rhs.bulk_dual())
}

/// The algorithm for this is: `lhs.antiwedge( rhs.weight_dual() )`
pub fn canonical_weight_contraction<Lhs, Rhs>(
    lhs: Lhs,
    rhs: Rhs,
) -> <Lhs as AntiwedgeProduct<<Rhs as Dual>::AntiKVector>>::Output
where
    Rhs: Dual,
    Lhs: AntiwedgeProduct<<Rhs as Dual>::AntiKVector>,
{
    lhs.antiwedge(&rhs.weight_dual())
}

/// The algorithm for this is: `lhs.wedge( rhs.bulk_dual() )`
pub fn canonical_bulk_expansion<Lhs, Rhs>(
    lhs: Lhs,
    rhs: Rhs,
) -> <Lhs as WedgeProduct<<Rhs as Dual>::AntiKVector>>::Output
where
    Rhs: Dual,
    Lhs: WedgeProduct<<Rhs as Dual>::AntiKVector>,
{
    lhs.wedge(&rhs.bulk_dual())
}

/// The algorithm for this is: `lhs.wedge( rhs.weight_dual() )`
pub fn canonical_weight_expansion<Lhs, Rhs>(
    lhs: Lhs,
    rhs: Rhs,
) -> <Lhs as WedgeProduct<<Rhs as Dual>::AntiKVector>>::Output
where
    Rhs: Dual,
    Lhs: WedgeProduct<<Rhs as Dual>::AntiKVector>,
{
    lhs.wedge(&rhs.weight_dual())
}

pub fn canonical_geometric_cosine<Lhs, Rhs>(
    lhs: Lhs,
    rhs: Rhs,
) -> (
    <<Lhs as Expansion<Rhs>>::WeightOutput as Norm>::Scalar,
    <Lhs as Norm>::Antiscalar,
)
where
    Lhs: Expansion<Rhs>,
    Lhs: Norm,
    Rhs: Norm,
    <Lhs as Expansion<Rhs>>::WeightOutput: Norm,
    <Lhs as Norm>::Antiscalar: Antiscalar,
    <Rhs as Norm>::Antiscalar: Antiscalar,
    <<Lhs as Norm>::Antiscalar as Antiscalar>::T: Mul<
            <<Rhs as Norm>::Antiscalar as Antiscalar>::T,
            Output = <<Lhs as Norm>::Antiscalar as Antiscalar>::T,
        >,
{
    (
        lhs.weight_expansion(&rhs).bulk_norm(),
        <<Lhs as Norm>::Antiscalar as Antiscalar>::from_volume(
            lhs.weight_norm().volume() * rhs.weight_norm().volume(),
        ),
    )
}

pub fn canonical_cosine<Lhs, Rhs>(
    lhs: Lhs,
    rhs: Rhs,
) -> Option<<<Lhs as Norm>::Antiscalar as Antiscalar>::T>
where
    Lhs: Expansion<Rhs>,
    Lhs: Norm,
    Rhs: Norm,
    <Lhs as Expansion<Rhs>>::WeightOutput: Norm,
    <Lhs as Norm>::Antiscalar: Antiscalar,
    <Rhs as Norm>::Antiscalar: Antiscalar,
    <<Lhs as Norm>::Antiscalar as Antiscalar>::T: Mul<
            <<Rhs as Norm>::Antiscalar as Antiscalar>::T,
            Output = <<Lhs as Norm>::Antiscalar as Antiscalar>::T,
        >,
    <<Lhs as Norm>::Antiscalar as Antiscalar>::T: Epsilon,
    <<Lhs as Expansion<Rhs>>::WeightOutput as Norm>::Scalar: Div<
            <<Lhs as Norm>::Antiscalar as Antiscalar>::T,
            Output = <<Lhs as Norm>::Antiscalar as Antiscalar>::T,
        >,
{
    let a = lhs.weight_expansion(&rhs).bulk_norm();
    let b = lhs.weight_norm().volume() * rhs.weight_norm().volume();

    if b.is_near_zero() { None } else { Some(a / b) }
}

pub fn canonical_geometric_cosine_symetric<T>(
    lhs: T,
    rhs: T,
) -> (
    <<T as Norm>::Antiscalar as Antiscalar>::T,
    <T as Norm>::Antiscalar,
)
where
    T: Expansion<T, WeightOutput = <T as Norm>::Antiscalar>,
    T: Norm,
    <T as Norm>::Antiscalar: Antiscalar,
    <<T as Norm>::Antiscalar as Antiscalar>::T:
        Mul<Output = <<T as Norm>::Antiscalar as Antiscalar>::T>,
{
    (
        lhs.weight_expansion(&rhs).volume(),
        <<T as Norm>::Antiscalar as Antiscalar>::from_volume(
            lhs.weight_norm().volume() * rhs.weight_norm().volume(),
        ),
    )
}

pub fn canonical_cosine_symetric<T>(
    lhs: T,
    rhs: T,
) -> Option<<<T as Norm>::Antiscalar as Antiscalar>::T>
where
    T: Expansion<T, WeightOutput = <T as Norm>::Antiscalar>,
    T: Norm,
    <T as Norm>::Antiscalar: Antiscalar,
    <<T as Norm>::Antiscalar as Antiscalar>::T:
        Mul<Output = <<T as Norm>::Antiscalar as Antiscalar>::T>,
    <<T as Norm>::Antiscalar as Antiscalar>::T: Epsilon,
    <<T as Norm>::Antiscalar as Antiscalar>::T:
        Div<Output = <<T as Norm>::Antiscalar as Antiscalar>::T>,
{
    let a = lhs.weight_expansion(&rhs).volume();
    let b = lhs.weight_norm().volume() * rhs.weight_norm().volume();

    if b.is_near_zero() { None } else { Some(a / b) }
}
