//! This module contains canonical implementations of the basic operations
//! This implementations are for testing purposes and are not supposed to be anywhere
//! near optimal implementations, they're just for reference

use crate::{AntiwedgeProduct, Complement, Dual, Metric, WedgeProduct};

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
