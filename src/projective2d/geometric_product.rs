use std::ops::Mul;

use crate::GeometricProduct;
use crate::Projective;
use crate::geometry3d::{Bivector, Evenvector, Multivector, Trivector, Vector};
use crate::projective2d::{DirVector, Point, UnitVector};

// ----------------------------------------------------------------------------------------------------
// Macros
// ----------------------------------------------------------------------------------------------------

macro_rules! equivalent_vector_geometric_product {
    ($ht:ident<T, Projective>, $pt:ident<T>) => {
        impl<T: Copy> GeometricProduct<$pt<T>> for $ht<T, Projective>
        where
            $pt<T>: Into<Vector<T, Projective>>,
            $ht<T, Projective>: GeometricProduct<Vector<T, Projective>>,
        {
            type Output = <$ht<T, Projective> as GeometricProduct<Vector<T, Projective>>>::Output;

            fn geometric_product(&self, rhs: &$pt<T>) -> Self::Output {
                self.geometric_product(&(*rhs).into())
            }
        }
    };

    ($pt:ident<T>, $ht:ident<T, Projective>) => {
        impl<T: Copy> GeometricProduct<$ht<T, Projective>> for $pt<T>
        where
            $pt<T>: Into<Vector<T, Projective>>,
            Vector<T, Projective>: GeometricProduct<$ht<T, Projective>>,
        {
            type Output = <Vector<T, Projective> as GeometricProduct<$ht<T, Projective>>>::Output;

            fn geometric_product(&self, rhs: &$ht<T, Projective>) -> Self::Output {
                (*self).into().geometric_product(rhs)
            }
        }
    };

    ($lht:ident<T>, $rht:ident<T>) => {
        impl<T: Copy> GeometricProduct<$rht<T>> for $lht<T>
        where
            $lht<T>: Into<Vector<T, Projective>>,
            $rht<T>: Into<Vector<T, Projective>>,
            Vector<T, Projective>:
                GeometricProduct<Vector<T, Projective>, Output = Evenvector<T, Projective>>,
        {
            type Output = Evenvector<T, Projective>;
            fn geometric_product(&self, rhs: &$rht<T>) -> Self::Output {
                (*self).into().geometric_product(&(*rhs).into())
            }
        }
    };
}

// ----------------------------------------------------------------------------------------------------
// Points
// ----------------------------------------------------------------------------------------------------

impl<T: Copy> GeometricProduct<Point<T>> for T
where
    T: Mul<Output = T>,
{
    type Output = Point<T>;
    fn geometric_product(&self, rhs: &Point<T>) -> Self::Output {
        Point::new(*self * rhs.0.x, *self * rhs.0.y)
    }
}

equivalent_vector_geometric_product!(Point<T>, Vector<T, Projective>);
equivalent_vector_geometric_product!(Point<T>, Bivector<T, Projective>);
equivalent_vector_geometric_product!(Point<T>, Trivector<T, Projective>);
equivalent_vector_geometric_product!(Point<T>, Evenvector<T, Projective>);
equivalent_vector_geometric_product!(Point<T>, Multivector<T, Projective>);

equivalent_vector_geometric_product!(Vector<T, Projective>, Point<T>);
equivalent_vector_geometric_product!(Bivector<T, Projective>, Point<T>);
equivalent_vector_geometric_product!(Trivector<T, Projective>, Point<T>);
equivalent_vector_geometric_product!(Evenvector<T, Projective>, Point<T>);
equivalent_vector_geometric_product!(Multivector<T, Projective>, Point<T>);

equivalent_vector_geometric_product!(Point<T>, Point<T>);
equivalent_vector_geometric_product!(Point<T>, DirVector<T>);
equivalent_vector_geometric_product!(Point<T>, UnitVector<T>);

// ----------------------------------------------------------------------------------------------------
// DirVector
// ----------------------------------------------------------------------------------------------------

equivalent_vector_geometric_product!(DirVector<T>, Vector<T, Projective>);
equivalent_vector_geometric_product!(DirVector<T>, Bivector<T, Projective>);
equivalent_vector_geometric_product!(DirVector<T>, Trivector<T, Projective>);
equivalent_vector_geometric_product!(DirVector<T>, Evenvector<T, Projective>);
equivalent_vector_geometric_product!(DirVector<T>, Multivector<T, Projective>);

// equivalent_vector_geometric_product!(Vector<T, Projective>, DirVector<T>);
// equivalent_vector_geometric_product!(Bivector<T, Projective>, DirVector<T>);
// equivalent_vector_geometric_product!(Trivector<T, Projective>, DirVector<T>);
// equivalent_vector_geometric_product!(Evenvector<T, Projective>, DirVector<T>);
// equivalent_vector_geometric_product!(Multivector<T, Projective>, DirVector<T>);

equivalent_vector_geometric_product!(DirVector<T>, Point<T>);
// equivalent_vector_geometric_product!(DirVector<T>, DirVector<T>);
equivalent_vector_geometric_product!(DirVector<T>, UnitVector<T>);

// ----------------------------------------------------------------------------------------------------
// Unit Vector
// ----------------------------------------------------------------------------------------------------

impl<T: Copy> GeometricProduct<UnitVector<T>> for T
where
    T: Mul<Output = T>,
{
    type Output = DirVector<T>;
    fn geometric_product(&self, rhs: &UnitVector<T>) -> Self::Output {
        DirVector::new(*self * rhs.0.x, *self * rhs.0.y)
    }
}

equivalent_vector_geometric_product!(UnitVector<T>, Vector<T, Projective>);
equivalent_vector_geometric_product!(UnitVector<T>, Bivector<T, Projective>);
equivalent_vector_geometric_product!(UnitVector<T>, Trivector<T, Projective>);
equivalent_vector_geometric_product!(UnitVector<T>, Evenvector<T, Projective>);
equivalent_vector_geometric_product!(UnitVector<T>, Multivector<T, Projective>);

equivalent_vector_geometric_product!(Vector<T, Projective>, UnitVector<T>);
equivalent_vector_geometric_product!(Bivector<T, Projective>, UnitVector<T>);
equivalent_vector_geometric_product!(Trivector<T, Projective>, UnitVector<T>);
equivalent_vector_geometric_product!(Evenvector<T, Projective>, UnitVector<T>);
equivalent_vector_geometric_product!(Multivector<T, Projective>, UnitVector<T>);

equivalent_vector_geometric_product!(UnitVector<T>, Point<T>);
equivalent_vector_geometric_product!(UnitVector<T>, DirVector<T>);
equivalent_vector_geometric_product!(UnitVector<T>, UnitVector<T>);
