use std::ops::Mul;

use crate::GeometricProduct;
use crate::Projective;
use crate::geometry3d::{Bivector, Evenvector, Multivector, Trivector, Vector};
use crate::projective2d::{DirVector, Line, Point, UnitLine, UnitVector};

// ----------------------------------------------------------------------------------------------------
// Macros
// ----------------------------------------------------------------------------------------------------

macro_rules! equivalent_vector_geometric_product {
    ($ht:ident, $pt:ident : $base:ident) => {
        impl<T: Copy> GeometricProduct<$pt<T>> for $ht<T, Projective>
        where
            $pt<T>: Into<$base<T, Projective>>,
            $ht<T, Projective>: GeometricProduct<$base<T, Projective>>,
        {
            type Output = <$ht<T, Projective> as GeometricProduct<$base<T, Projective>>>::Output;

            fn geometric_product(&self, rhs: &$pt<T>) -> Self::Output {
                self.geometric_product(&(*rhs).into())
            }
        }
    };

    ($pt:ident : $base:ident, $ht:ident) => {
        impl<T: Copy> GeometricProduct<$ht<T, Projective>> for $pt<T>
        where
            $pt<T>: Into<$base<T, Projective>>,
            $base<T, Projective>: GeometricProduct<$ht<T, Projective>>,
        {
            type Output = <$base<T, Projective> as GeometricProduct<$ht<T, Projective>>>::Output;

            fn geometric_product(&self, rhs: &$ht<T, Projective>) -> Self::Output {
                (*self).into().geometric_product(rhs)
            }
        }
    };

    ($lht:ident : $lbase:ident, $rht:ident : $rbase:ident) => {
        impl<T: Copy> GeometricProduct<$rht<T>> for $lht<T>
        where
            $lht<T>: Into<$lbase<T, Projective>>,
            $rht<T>: Into<$rbase<T, Projective>>,
            $lbase<T, Projective>: GeometricProduct<$rbase<T, Projective>>,
        {
            type Output =
                <$lbase<T, Projective> as GeometricProduct<$rbase<T, Projective>>>::Output;
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

equivalent_vector_geometric_product!(Point : Vector, Vector);
equivalent_vector_geometric_product!(Point : Vector, Bivector);
equivalent_vector_geometric_product!(Point : Vector, Trivector);
equivalent_vector_geometric_product!(Point : Vector, Evenvector);
equivalent_vector_geometric_product!(Point : Vector, Multivector);

equivalent_vector_geometric_product!(Vector, Point : Vector);
equivalent_vector_geometric_product!(Bivector, Point : Vector);
equivalent_vector_geometric_product!(Trivector, Point : Vector);
equivalent_vector_geometric_product!(Evenvector, Point : Vector);
equivalent_vector_geometric_product!(Multivector, Point : Vector);

equivalent_vector_geometric_product!(Point : Vector, Point : Vector);
equivalent_vector_geometric_product!(Point : Vector, DirVector : Vector);
equivalent_vector_geometric_product!(Point : Vector, UnitVector : Vector);
equivalent_vector_geometric_product!(Point : Vector, UnitLine : Bivector);

// ----------------------------------------------------------------------------------------------------
// DirVector
// ----------------------------------------------------------------------------------------------------

equivalent_vector_geometric_product!(DirVector : Vector, Vector);
equivalent_vector_geometric_product!(DirVector : Vector, Bivector);
equivalent_vector_geometric_product!(DirVector : Vector, Trivector);
equivalent_vector_geometric_product!(DirVector : Vector, Evenvector);
equivalent_vector_geometric_product!(DirVector : Vector, Multivector);

// equivalent_vector_geometric_product!(Vector, DirVector : Vector);
// equivalent_vector_geometric_product!(Bivector, DirVector : Vector);
// equivalent_vector_geometric_product!(Trivector, DirVector : Vector);
// equivalent_vector_geometric_product!(Evenvector, DirVector : Vector);
// equivalent_vector_geometric_product!(Multivector, DirVector : Vector);

equivalent_vector_geometric_product!(DirVector : Vector, Point : Vector);
// equivalent_vector_geometric_product!(DirVector : Vector, DirVector : Vector);
equivalent_vector_geometric_product!(DirVector : Vector, UnitVector : Vector);
equivalent_vector_geometric_product!(DirVector : Vector, UnitLine : Bivector);

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

equivalent_vector_geometric_product!(UnitVector : Vector, Vector);
equivalent_vector_geometric_product!(UnitVector : Vector, Bivector);
equivalent_vector_geometric_product!(UnitVector : Vector, Trivector);
equivalent_vector_geometric_product!(UnitVector : Vector, Evenvector);
equivalent_vector_geometric_product!(UnitVector : Vector, Multivector);

equivalent_vector_geometric_product!(Vector, UnitVector : Vector);
equivalent_vector_geometric_product!(Bivector, UnitVector : Vector);
equivalent_vector_geometric_product!(Trivector, UnitVector : Vector);
equivalent_vector_geometric_product!(Evenvector, UnitVector : Vector);
equivalent_vector_geometric_product!(Multivector, UnitVector : Vector);

equivalent_vector_geometric_product!(UnitVector : Vector, Point : Vector);
equivalent_vector_geometric_product!(UnitVector : Vector, DirVector : Vector);
equivalent_vector_geometric_product!(UnitVector : Vector, UnitVector : Vector);
equivalent_vector_geometric_product!(UnitVector : Vector, UnitLine : Bivector);

// ----------------------------------------------------------------------------------------------------
// Unit Line
// ----------------------------------------------------------------------------------------------------

impl<T: Copy> GeometricProduct<UnitLine<T>> for T
where
    T: Mul<Output = T>,
{
    type Output = Line<T>;
    fn geometric_product(&self, rhs: &UnitLine<T>) -> Self::Output {
        Line::new(*self * rhs.0.yz, *self * rhs.0.zx, *self * rhs.0.xy)
    }
}

equivalent_vector_geometric_product!(UnitLine : Bivector, Vector);
equivalent_vector_geometric_product!(UnitLine : Bivector, Bivector);
equivalent_vector_geometric_product!(UnitLine : Bivector, Trivector);
equivalent_vector_geometric_product!(UnitLine : Bivector, Evenvector);
equivalent_vector_geometric_product!(UnitLine : Bivector, Multivector);

equivalent_vector_geometric_product!(Vector, UnitLine : Bivector);
equivalent_vector_geometric_product!(Bivector, UnitLine : Bivector);
equivalent_vector_geometric_product!(Trivector, UnitLine : Bivector);
equivalent_vector_geometric_product!(Evenvector, UnitLine : Bivector);
equivalent_vector_geometric_product!(Multivector, UnitLine : Bivector);

equivalent_vector_geometric_product!(UnitLine : Bivector, Point : Vector);
equivalent_vector_geometric_product!(UnitLine : Bivector, DirVector : Vector);
equivalent_vector_geometric_product!(UnitLine : Bivector, UnitVector : Vector);
equivalent_vector_geometric_product!(UnitLine : Bivector, UnitLine : Bivector);
