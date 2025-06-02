use crate::{Complement, Dual};

use super::{Bivector, Vector};

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
