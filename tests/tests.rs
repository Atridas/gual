use gual::{AntiwedgeProduct, Complement, Dual, WedgeProduct};
mod geometry2d;

pub fn antiwedge<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> <<<Lhs as Complement>::Output as WedgeProduct<<Rhs as Complement>::Output>>::Output as Complement>::Output
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

pub fn bulk_contraction<Lhs, Rhs>(
    lhs: Lhs,
    rhs: Rhs,
) -> <Lhs as AntiwedgeProduct<<Rhs as Dual>::AntiKVector>>::Output
where
    Rhs: Dual,
    Lhs: AntiwedgeProduct<<Rhs as Dual>::AntiKVector>,
{
    lhs.antiwedge(&rhs.bulk_dual())
}

pub fn weight_contraction<Lhs, Rhs>(
    lhs: Lhs,
    rhs: Rhs,
) -> <Lhs as AntiwedgeProduct<<Rhs as Dual>::AntiKVector>>::Output
where
    Rhs: Dual,
    Lhs: AntiwedgeProduct<<Rhs as Dual>::AntiKVector>,
{
    lhs.antiwedge(&rhs.weight_dual())
}

pub fn bulk_expansion<Lhs, Rhs>(
    lhs: Lhs,
    rhs: Rhs,
) -> <Lhs as WedgeProduct<<Rhs as Dual>::AntiKVector>>::Output
where
    Rhs: Dual,
    Lhs: WedgeProduct<<Rhs as Dual>::AntiKVector>,
{
    lhs.wedge(&rhs.bulk_dual())
}

pub fn weight_expansion<Lhs, Rhs>(
    lhs: Lhs,
    rhs: Rhs,
) -> <Lhs as WedgeProduct<<Rhs as Dual>::AntiKVector>>::Output
where
    Rhs: Dual,
    Lhs: WedgeProduct<<Rhs as Dual>::AntiKVector>,
{
    lhs.wedge(&rhs.weight_dual())
}
