use std::ops::{Mul, Neg, Sub};

use crate::GeometricProduct;

use super::Evenvector;

impl<T> Sub for Evenvector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Evenvector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Evenvector {
            s: self.s - rhs.s,
            b: self.b - rhs.b,
        }
    }
}

impl<T> Neg for Evenvector<T>
where
    T: Neg<Output = T>,
{
    type Output = Evenvector<T>;
    fn neg(self) -> Self::Output {
        Evenvector {
            s: -self.s,
            b: -self.b,
        }
    }
}

impl<T, M> Mul<Evenvector<T, M>> for Evenvector<T, M>
where
    Evenvector<T, M>: GeometricProduct<Evenvector<T, M>, Output = Evenvector<T, M>>,
{
    type Output = Evenvector<T, M>;

    fn mul(self, rhs: Evenvector<T, M>) -> Self::Output {
        self.geometric_product(&rhs)
    }
}

// impl<T> GeometricProduct<Evenvector<T>> for Evenvector<T>
// where
//     T: Copy,
//     T: Add<T, Output = T>,
//     T: Sub<T, Output = T>,
//     T: Neg<Output = T>,
//     T: Mul<T, Output = T>,
// {
//     type Output = Evenvector<T>;

//     fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
//         self.b.geometric_product(&rhs.b)
//             + Evenvector {
//                 s: self.s * rhs.s,
//                 b: self.s * rhs.b + self.b * rhs.s,
//             }
//     }
// }
