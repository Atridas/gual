use std::marker::PhantomData;

pub mod canonical;
pub mod geometry2d;
pub mod geometry3d;
pub mod geometry4d;
pub mod projective2d;

mod blanket_impls;
mod macros;
mod scalar;

pub mod homogeneous3d;

/// Marker type used to define an Euclidean metric.
///
/// Euclidean metrics are those whose all basis vectors square to 1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Euclidean {}

/// Marker type used to define a Projective metric.
///
/// A Projective metric is build on top of an euclidean metric, with an additional vector
/// that squares to 0. This is used to build projective geometries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Projective {}

/// Wrapper type for scalars so we can overwrite + and * (and other) operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scalar<const D: u32, T, M = Euclidean>(pub T, PhantomData<M>);

/// Helper trait to avoid division by (nearly) zero
pub trait Epsilon {
    /// Returns an arbitrarily small positive number
    fn eps() -> Self;
    /// Returns true if the absolute value of the number is smaller than [`Epsilon::eps`]
    fn is_near_zero(&self) -> bool;
}

/// Marks the element with the largest dimensionality of a geometry.
pub trait Antiscalar {
    /// Element representing the unit volume
    const UNIT_VOLUME: Self;

    /// Usually the type that represents the scalar of the geometry
    type T;

    /// Gets the volume value as a scalar
    fn volume(&self) -> Self::T;

    /// Constructs the volume element from a scalar
    fn from_volume(volume: Self::T) -> Self;
}

pub trait VectorSpace {
    type Scalar;
    type Vector;
    type Antivector;
    type Antiscalar: Antiscalar;

    const UNIT_VOLUME: Self::Antiscalar = Self::Antiscalar::UNIT_VOLUME;

    fn scalar(&self) -> Self::Scalar;
    fn vector(&self) -> Self::Vector;
    fn antivector(&self) -> Self::Antivector;
    fn antiscalar(&self) -> Self::Antiscalar;

    // u ^ right_complement(u) = antiscalar
    fn right_complement(&self) -> Self;
    // left_complement(u) ^ u = antiscalar
    fn left_complement(&self) -> Self;
}

pub trait GeometricElement {
    const ALGEBRA_DIMENSION: u32;
    const ELEMENT_DIMENSION: u32;
    type Field;
    type Metric;

    type Scalar; // = Scalar<ALGEBRA_DIMENSION, Field, Metric>;
}

/// Complement operation for a geometric algebra.
///
/// This is one of the "arbitrary" operations for a geometric algebra, it needs
/// a canonical unit volume to be defined (Usually through [`Antiscalar`] and [`VectorSpace`]).
/// Then all "anti" operations should be consistent with this.
///
/// A right complement is usually written with a line over the element: u̅
///
/// It is defined so that: `u ^ u̅` = positive volume
///
/// A right complement is usually written with a line over the element: u̲
///
/// It is defined so that: `u̲ ^ u` = positive volume
///
/// Each operation has a dual counterpart, it's defined with this identity: `a antiop b = left_component(a̅ op b̅) = right_component(a̲ op b̲)`
pub trait Complement {
    type Output;

    /// Computes the right complement operation: u̅
    ///
    /// It is defined so that: u ^ u̅ = positive volume
    fn right_complement(&self) -> Self::Output;

    /// Computes the left complement operation: u̲
    ///
    /// It is defined so that: u̲ ^ u = positive volume
    fn left_complement(&self) -> Self::Output;
}

pub trait KVector {
    type AntiKVector;

    // u ^ right_complement(u) = antiscalar
    fn right_complement(&self) -> Self::AntiKVector;
    // left_complement(u) ^ u = antiscalar
    fn left_complement(&self) -> Self::AntiKVector;
}

/// Geometric algebra Wedge product.
///
/// This operation joints the dimensionality of 2 elements, so 2 vectors create a bivector,
/// a vector and a bivector create a trivector, etc. It's usually denoted with a wedge `^` symbol.
pub trait WedgeProduct<Rhs> {
    /// Resulting type of the wedge operation
    type Output;

    /// Wedges 2 operators
    fn wedge(&self, rhs: &Rhs) -> Self::Output;
}

/// Dual operation of the [`WedgeProduct`]
///
/// This operation makes the union or intersection of 2 elements, so 2 bivectors give a vector. It's usually
/// denoted with an inverse wedge `v` symbol
///
/// Being a dual operation, it is defined with respect a volume element [`Antiscalar`] and the corresponding
/// [`Complement`] operations
pub trait AntiwedgeProduct<Rhs> {
    type Output;
    fn antiwedge(&self, rhs: &Rhs) -> Self::Output;
}

pub trait Join<Rhs> {
    type Output;
    fn join(&self, rhs: &Rhs) -> Self::Output;
}

pub trait Meet<Rhs> {
    type Output;
    fn meet(&self, rhs: &Rhs) -> Self::Output;
}

/// Basic operation from the metric.
///
/// Toguether with [`Antiscalar`] this defines the behaviour of the algebra.
///
/// The metric defines a scalar for each pair of same-dimenstion vectors. This must be
/// a bilinear operation. If the result of the dot product between two elements is zero they are said
/// to be orthogonal. In euclidean space all basis vectors `dot` with themseleves to `1`, but they can
/// give `0` in some metrics (projective) or even `-1` in minkowski metrics.
///
/// The dual of the metric is the antimetric and is created with a process similar to that described
/// in [`Complement`] for dual operations. It is the same as the metric in euclidean space, but swaps the
/// roles of null and non-null vectors in the other metric spaces.
pub trait Dot {
    /// Type used as a scalar in the algebra
    type Scalar;

    /// Type that represents the full space in the algebra
    type Antiscalar;

    /// Full dot operation
    fn dot(&self, rhs: &Self) -> Self::Scalar;

    /// Dual dot operation
    fn antidot(&self, rhs: &Self) -> Self::Antiscalar;

    /// Returns the addition of the dot and the antidot. This is useful in non-euclidean algebras.
    fn geometric_dot(&self, rhs: &Self) -> (Self::Scalar, Self::Antiscalar) {
        (self.dot(rhs), self.antidot(rhs))
    }
}

/// Defines a metric and divides the vector elements in a bulk and a weight.
///
/// This needs a metric consistent with [`Dot`]. This is not an interesting operation in
/// euclidean space (both bulk and weight return the full input) but becomes useful in other spaces.
///
/// In projective space the bulk is the part of the element whose dot product with themself whould not become
/// `0`, and the weight the part that whould become `0`.
pub trait Metric {
    /// Part left after applying the Metric
    type Bulk;

    /// Part left after applying the Antimetric
    type Weight;

    /// Part left after applying the Metric
    ///
    /// Usually denoted with a star `a*`
    fn bulk(&self) -> Self::Bulk;

    /// Part left after applying the Antimetric
    ///
    /// Whould be denoted with an empty star but standard ascii does
    /// not have that
    fn weight(&self) -> Self::Weight;

    /// Constructs an element from its bulk
    fn from_bulk(bulk: &Self::Bulk) -> Self;

    /// Constructs an element from its weight
    fn from_weight(weight: &Self::Weight) -> Self;

    /// Constructs an element from its bulk and weight
    fn from_bulk_and_weight(bulk: &Self::Bulk, weight: &Self::Weight) -> Self;

    /// Implementation of the bulk that works with Self
    fn proper_bulk(&self) -> Self;

    /// Implementation of the weight that works with Self
    fn proper_weight(&self) -> Self;
}

/// Computes the different norms available in an element
///
/// A norm is the square root of the element dotted with itself. The bulk norm uses the
/// [`Dot::dot`] and the weight norm uses the [`Dot::antidot`]
pub trait Norm {
    /// Type used as a scalar in the algebra
    type Scalar;

    /// Type that represents the full space in the algebra
    type Antiscalar: Antiscalar<T = Self::Scalar>;

    /// square of the [`Norm::bulk_norm`]
    fn bulk_norm_squared(&self) -> Self::Scalar;

    /// square of the [`Norm::weight_norm`]
    fn weight_norm_squared(&self) -> Self::Antiscalar;

    /// norm of the bulk
    fn bulk_norm(&self) -> Self::Scalar;

    /// norm of the weight
    fn weight_norm(&self) -> Self::Antiscalar;

    /// addition of the bulk and weight norms
    fn geometric_norm(&self) -> (Self::Scalar, Self::Antiscalar) {
        (self.bulk_norm(), self.weight_norm())
    }

    /// equivalent to the [`Norm::bulk_norm`]
    fn norm(&self) -> Self::Scalar {
        self.bulk_norm()
    }

    /// equivalent to the [`Norm::bulk_norm_squared`]
    fn norm_squared(&self) -> Self::Scalar {
        self.bulk_norm_squared()
    }
}

/// Makes the weight of an element `1`
///
/// Also known as "normalizing" something, it is always in reference to the weight of it.
///
/// In euclidean space it gives rise to unit vectors, etc. In projective space is what gives the point
/// a given vector represents (you take the bulk of a unitized vector to get it).
///
/// Sometimes special types can be leveraged that always contain a unitized element and can do some operations
/// more optimally.
pub trait Unitizable {
    type Output;

    /// Gives a version of itself whose weight norm is equal to `1`.
    /// Returns null if the original weight is too close to `0`.
    fn unitize(&self) -> Option<Self::Output>;
}

/// Dual operations.
///
/// This combines the [`Complement`] operation with the [`Metric`] operation in one. It gives 4 combinations
/// (left & right complements and metric (bulks) and antimetric (weights)). Remember that with euclidean
/// metrics there is no difference between the bulk and the weight.
///
/// We arbitrarily elevate the right variations to be the "default" ones (the only difference between the
/// left and right ones is a sign at most) that we'll use to create other operations (Like the [`Contraction`]
/// and the [`Expansion`]), we also define a default [`Dual::dual`] operation to be the bulk dual as a default
/// choice.
pub trait Dual {
    /// Complementary vector space
    type AntiKVector;

    /// The result of applying the metric and the right complement
    ///
    ///
    fn right_bulk_dual(&self) -> Self::AntiKVector;
    /// The result of applying the metric and the left complement
    fn left_bulk_dual(&self) -> Self::AntiKVector;
    /// The result of applying the antimetric and the right complement
    fn right_weight_dual(&self) -> Self::AntiKVector;
    /// The result of applying the antimetric and the left complement
    fn left_weight_dual(&self) -> Self::AntiKVector;

    /// Equivalent to the [`Dual::right_bulk_dual`]
    fn dual(&self) -> Self::AntiKVector {
        self.right_bulk_dual()
    }
    /// Equivalent to the [`Dual::right_bulk_dual`]
    fn bulk_dual(&self) -> Self::AntiKVector {
        self.right_bulk_dual()
    }
    /// Equivalent to the [`Dual::right_weight_dual`]
    fn weight_dual(&self) -> Self::AntiKVector {
        self.right_weight_dual()
    }
}

/// Contraction operation: `a v b*`
///
/// Comes with 2 variants: bulk and weight one. In euclidean space there is no difference.
///
/// It conceptually "substracts" one element from another, so `a.contraction(b)` will have the dimension
/// of `dimension(a) - dimension(b)`. If both have the same dimensions the result is a scalar equivalent to
/// the dot product.
///
/// The resulting object is contained within the first object and orthogonal to the second one.
pub trait Contraction<Rhs> {
    type BulkOutput;
    type WeightOutput;

    /// Substracts the bulk of `rhs` from `self`.
    ///
    /// The resulting object is contained within `self` and orthogonal to the bulk of `rhs`
    ///
    /// Equivalent to `self.antiwedge(rhs.right_bulk_dual())`
    fn bulk_contraction(&self, rhs: &Rhs) -> Self::BulkOutput;

    /// Substracts the weigth of `rhs` from `self`.
    ///
    /// The resulting object is contained within `self` and orthogonal to the weigth of `rhs`
    ///
    /// Equivalent to `self.antiwedge(rhs.right_weight_dual())`
    fn weight_contraction(&self, rhs: &Rhs) -> Self::WeightOutput;

    /// Equivalent to [`Contraction::bulk_contraction`]
    fn contraction(&self, rhs: &Rhs) -> Self::BulkOutput {
        self.bulk_contraction(rhs)
    }
}

/// Expansion operation: `a v b*`
///
/// Comes with 2 variants: bulk and weight one. In euclidean space there is no difference.
///
/// It conceptually "adds" the missing dimensions of the second object to the first object, so
/// `a.expansion(b)` will have the dimension of `dimension(a) + antidimension(b)`. If both have the
/// same dimensions the result is a scalar equivalent to the dot product.
///
/// The result will contain the first input and be orthogonal to the second input.
///
/// An example whould be expanding a vector using a bivector in 3 dimensions: the result will be another
/// bivector with the original vector in it, that is orthogonal to the bivector input.
pub trait Expansion<Rhs> {
    type BulkOutput;
    type WeightOutput;

    /// Adds the dual of the bulk of `rhs` from `self`.
    ///
    /// The resulting object contains `self` and is orthogonal to the bulk of `rhs`
    ///
    /// Equivalent to `self.wedge(rhs.right_bulk_dual())`
    fn bulk_expansion(&self, rhs: &Rhs) -> Self::BulkOutput;

    /// Adds the dual of the weight of `rhs` from `self`.
    ///
    /// The resulting object contains `self` and is orthogonal to the weight of `rhs`
    ///
    /// Equivalent to `self.wedge(rhs.right_weight_dual())`
    fn weight_expansion(&self, rhs: &Rhs) -> Self::WeightOutput;

    /// Equivalent to [`Expansion::bulk_expansion`]
    fn expansion(&self, rhs: &Rhs) -> Self::BulkOutput {
        self.bulk_expansion(rhs)
    }
}

pub trait OrthogonalProjection<Rhs> {
    type Output;
    fn orthogonal_projection(&self, rhs: &Rhs) -> Self::Output;
}

pub trait CentralProjection<Rhs> {
    type Output;
    fn central_projection(&self, rhs: &Rhs) -> Self::Output;
}

pub trait OrthogonalAntiprojection<Rhs> {
    type Output;
    fn orthogonal_antiprojection(&self, rhs: &Rhs) -> Self::Output;
}

pub trait CentralAntiprojection<Rhs> {
    type Output;
    fn central_antiprojection(&self, rhs: &Rhs) -> Self::Output;
}

pub trait GeometricProduct<Rhs> {
    type Output;
    fn geometric_product(&self, rhs: &Rhs) -> Self::Output;
}

pub trait GeometricAntiproduct<Rhs> {
    type Output;
    fn geometric_antiproduct(&self, rhs: &Rhs) -> Self::Output;
}

/// Computes the angle between 2 primitives
///
/// The definition for the geometric cosine is: `bulk_norm(weight_expansion(a, b)) + weight_norm(a) * weight_norm(b)`
///
/// This gives an scalar and an antiscalar. The actual cosine can be retrieved by
/// dividing the scalar by the antiscalar values.
///
/// There is a specialization in case both primitives have the same dimension: we "skip" the bulk_norm. This gives us
/// a signed value, so we can have larger than right angles as a result (wich whould be meaningless in primitives of
/// different dimensions)
pub trait Angle<Rhs> {
    type Scalar;
    type Antiscalar;

    fn geometric_cosine(&self, rhs: &Rhs) -> (Self::Scalar, Self::Antiscalar);
    fn cosine(&self, rhs: &Rhs) -> Option<Self::Scalar>;
}

pub trait Attitude {
    type Output;

    fn attitude(&self) -> Self::Output;
}

pub trait Distance<Rhs> {
    type Scalar;
    type Antiscalar;

    fn geometric_distance(&self, rhs: &Rhs) -> (Self::Scalar, Self::Antiscalar);
    fn distance(&self, rhs: &Rhs) -> Self::Scalar;
}

pub trait Support {
    type Point;

    fn support(&self) -> Self::Point;
}

pub trait Antisupport {
    type Plane;

    fn antisupport(&self) -> Self::Plane;
}

impl<const D: u32, T, M> GeometricElement for Scalar<D, T, M> {
    const ALGEBRA_DIMENSION: u32 = D;
    const ELEMENT_DIMENSION: u32 = 0;
    type Field = T;
    type Metric = M;
    type Scalar = Self;
}

#[macro_export]
macro_rules! reverse_add {
    ($lht:ident, $rht:ident) => {
        impl<T> Add<$rht<T>> for $lht<T>
        where
            $rht<T>: Add<$lht<T>>,
        {
            type Output = <$rht<T> as Add<$lht<T>>>::Output;

            fn add(self, rhs: $rht<T>) -> Self::Output {
                rhs + self
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_mul {
    ($lht:ident, $rht:ident) => {
        impl<T> Mul<$rht<T>> for $lht<T>
        where
            $rht<T>: Mul<$lht<T>>,
        {
            type Output = <$rht<T> as Mul<$lht<T>>>::Output;

            fn mul(self, rhs: $rht<T>) -> Self::Output {
                rhs * self
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_wedge {
    ($lht:ident, $rht:ident) => {
        impl<T> WedgeProduct<$rht<T>> for $lht<T>
        where
            $lht<T>: Copy,
            $rht<T>: WedgeProduct<$lht<T>>,
        {
            type Output = <$rht<T> as WedgeProduct<$lht<T>>>::Output;

            fn wedge(&self, rhs: &$rht<T>) -> Self::Output {
                rhs.wedge(self)
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_antiwedge {
    ($lht:ident, $rht:ident) => {
        impl<T> AntiwedgeProduct<$rht<T>> for $lht<T>
        where
            $lht<T>: Copy,
            $rht<T>: AntiwedgeProduct<$lht<T>>,
        {
            type Output = <$rht<T> as AntiwedgeProduct<$lht<T>>>::Output;

            fn antiwedge(&self, rhs: &$rht<T>) -> Self::Output {
                rhs.antiwedge(self)
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_antiwedge_anticommutative {
    ($lht:ident, $rht:ident) => {
        impl<T> AntiwedgeProduct<$rht<T>> for $lht<T>
        where
            $rht<T>: AntiwedgeProduct<$lht<T>>,
            $lht<T>: Neg<Output = $lht<T>>,
        {
            type Output = <$rht<T> as AntiwedgeProduct<$lht<T>>>::Output;

            fn antiwedge(&self, rhs: $rht<T>) -> Self::Output {
                rhs.antiwedge(-self)
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_geometric {
    ($lht:ident, $rht:ident) => {
        impl<T> GeometricProduct<$rht<T>> for $lht<T>
        where
            $lht<T>: Copy,
            $rht<T>: GeometricProduct<$lht<T>>,
        {
            type Output = <$rht<T> as GeometricProduct<$lht<T>>>::Output;

            fn geometric_product(&self, rhs: &$rht<T>) -> Self::Output {
                rhs.geometric_product(self)
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_antigeometric {
    ($lht:ident, $rht:ident) => {
        impl<T> GeometricAntiproduct<$rht<T>> for $lht<T>
        where
            $lht<T>: Copy,
            $rht<T>: GeometricAntiproduct<$lht<T>>,
        {
            type Output = <$rht<T> as GeometricAntiproduct<$lht<T>>>::Output;

            fn geometric_antiproduct(&self, rhs: &$rht<T>) -> Self::Output {
                rhs.geometric_antiproduct(self)
            }
        }
    };
}
