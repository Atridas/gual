//! This module contains canonical implementations of the basic operations
//! This implementations are for testing purposes and are not supposed to be anywhere
//! near optimal implementations, they're just for reference

use std::ops::{Div, Mul};

use crate::{Antiscalar, AntiwedgeProduct, Complement, Dual, Epsilon, Norm, WedgeProduct};

use crate::Expansion as CrateExpansion;
use crate::Metric as CrateMetric;

/// The algorithm for this is: `right_complement( left_complement(lhs) ^ left_complement(rhs) )`
pub trait Antiwedge<Rht> {
    type Output;
    fn canonical_antiwedge(&self, rhs: &Rht) -> Self::Output;
}

impl<Lht, Rht> Antiwedge<Rht> for Lht
where
    Lht: Complement,
    Rht: Complement,
    <Lht as Complement>::Output: WedgeProduct<<Rht as Complement>::Output>,
    <<Lht as Complement>::Output as WedgeProduct<<Rht as Complement>::Output>>::Output: Complement,
{
    type Output = <<<Lht as Complement>::Output as WedgeProduct<<Rht as Complement>::Output>>::Output as Complement>::Output;

    fn canonical_antiwedge(&self, rhs: &Rht) -> Self::Output {
        self.left_complement()
            .wedge(&rhs.left_complement())
            .right_complement()
    }
}

pub trait Metric {
    type DualOutput;

    /// The algorithm for this is: `right_complement( bulk( left_complement(a) ) )`
    fn canonical_weight(&self) -> Self;

    /// The algorithm for this is: `right_complement( bulk(a) )`
    fn canonical_right_bulk_dual(&self) -> Self::DualOutput;
    /// The algorithm for this is: `right_complement( weight(a) )`
    fn canonical_right_weight_dual(&self) -> Self::DualOutput;
    /// The algorithm for this is: `left_complement( bulk(a) )`
    fn canonical_left_bulk_dual(&self) -> Self::DualOutput;
    /// The algorithm for this is: `left_complement( weight(a) )`
    fn canonical_left_weight_dual(&self) -> Self::DualOutput;
}

impl<T> Metric for T
where
    T: Complement,
    T: CrateMetric,
    <T as Complement>::Output: CrateMetric,
    <T as Complement>::Output: Complement<Output = T>,
{
    type DualOutput = <T as Complement>::Output;

    fn canonical_weight(&self) -> Self {
        self.left_complement().proper_bulk().right_complement()
    }

    fn canonical_right_bulk_dual(&self) -> Self::DualOutput {
        self.proper_bulk().right_complement()
    }

    fn canonical_left_bulk_dual(&self) -> Self::DualOutput {
        self.proper_bulk().left_complement()
    }

    fn canonical_right_weight_dual(&self) -> Self::DualOutput {
        self.proper_weight().right_complement()
    }

    fn canonical_left_weight_dual(&self) -> Self::DualOutput {
        self.proper_weight().left_complement()
    }
}

pub trait Contraction<Rht> {
    type Output;

    /// The algorithm for this is: `lhs.antiwedge( rhs.bulk_dual() )`
    fn canonical_bulk_contraction(&self, rhs: &Rht) -> Self::Output;
    /// The algorithm for this is: `lhs.antiwedge( rhs.weight_dual() )`
    fn canonical_weight_contraction(&self, rhs: &Rht) -> Self::Output;
}

pub trait Expansion<Rht> {
    type Output;

    /// The algorithm for this is: `lhs.wedge( rhs.bulk_dual() )`
    fn canonical_bulk_expansion(&self, rhs: &Rht) -> Self::Output;
    /// The algorithm for this is: `lhs.wedge( rhs.weight_dual() )`
    fn canonical_weight_expansion(&self, rhs: &Rht) -> Self::Output;
}

impl<Lht, Rht> Contraction<Rht> for Lht
where
    Rht: Dual,
    Lht: AntiwedgeProduct<<Rht as Dual>::AntiKVector>,
{
    type Output = <Lht as AntiwedgeProduct<<Rht as Dual>::AntiKVector>>::Output;
    fn canonical_bulk_contraction(&self, rhs: &Rht) -> Self::Output {
        self.antiwedge(&rhs.bulk_dual())
    }
    fn canonical_weight_contraction(&self, rhs: &Rht) -> Self::Output {
        self.antiwedge(&rhs.weight_dual())
    }
}

impl<Lht, Rht> Expansion<Rht> for Lht
where
    Rht: Dual,
    Lht: WedgeProduct<<Rht as Dual>::AntiKVector>,
{
    type Output = <Lht as WedgeProduct<<Rht as Dual>::AntiKVector>>::Output;
    fn canonical_bulk_expansion(&self, rhs: &Rht) -> Self::Output {
        self.wedge(&rhs.bulk_dual())
    }
    fn canonical_weight_expansion(&self, rhs: &Rht) -> Self::Output {
        self.wedge(&rhs.weight_dual())
    }
}

pub trait Angle<Rht> {
    type Scalar;
    type Antiscalar;

    fn canonical_geometric_cosine(&self, rhs: &Rht) -> (Self::Scalar, Self::Antiscalar);
    fn canonical_cosine(&self, rhs: &Rht) -> Option<Self::Scalar>;
}

impl<Lht, Rht> Angle<Rht> for Lht
where
    Lht: CrateExpansion<Rht>,
    Lht: Norm,
    Rht: Norm,
    <Lht as CrateExpansion<Rht>>::WeightOutput: Norm<Scalar = <Lht as Norm>::Scalar>,
    <Lht as Norm>::Antiscalar: Antiscalar<T = <Lht as Norm>::Scalar>,
    <Rht as Norm>::Antiscalar: Antiscalar<T = <Lht as Norm>::Scalar>,
    <Lht as Norm>::Scalar: Epsilon,
    <Lht as Norm>::Scalar: Mul<<Lht as Norm>::Scalar, Output = <Lht as Norm>::Scalar>,
    <Lht as Norm>::Scalar: Div<<Lht as Norm>::Scalar, Output = <Lht as Norm>::Scalar>,
{
    type Scalar = <<Lht as CrateExpansion<Rht>>::WeightOutput as Norm>::Scalar;
    type Antiscalar = <Lht as Norm>::Antiscalar;

    fn canonical_geometric_cosine(&self, rhs: &Rht) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.weight_expansion(&rhs).bulk_norm(),
            <<Lht as Norm>::Antiscalar as Antiscalar>::from_volume(
                self.weight_norm().volume() * rhs.weight_norm().volume(),
            ),
        )
    }

    fn canonical_cosine(&self, rhs: &Rht) -> Option<Self::Scalar> {
        let geometric_cosine = self.canonical_geometric_cosine(&rhs);

        if geometric_cosine.1.volume().is_near_zero() {
            None
        } else {
            Some(geometric_cosine.0 / geometric_cosine.1.volume())
        }
    }
}

pub trait SymetricAngle {
    type Scalar;
    type Antiscalar;

    fn canonical_geometric_cosine_symetric(&self, rhs: &Self) -> (Self::Scalar, Self::Antiscalar);
    fn canonical_cosine_symetric(&self, rhs: &Self) -> Option<Self::Scalar>;
}

impl<T> SymetricAngle for T
where
    T: CrateExpansion<T, WeightOutput = <T as Norm>::Antiscalar>,
    T: Norm,
    <T as Norm>::Antiscalar: Antiscalar<T = <T as Norm>::Scalar>,
    <T as Norm>::Scalar: Epsilon,
    <T as Norm>::Scalar: Mul<Output = <T as Norm>::Scalar>,
    <T as Norm>::Scalar: Div<Output = <T as Norm>::Scalar>,
{
    type Scalar = <<T as Norm>::Antiscalar as Antiscalar>::T;
    type Antiscalar = <T as Norm>::Antiscalar;

    fn canonical_geometric_cosine_symetric(&self, rhs: &Self) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.weight_expansion(&rhs).volume(),
            <<T as Norm>::Antiscalar as Antiscalar>::from_volume(
                self.weight_norm().volume() * rhs.weight_norm().volume(),
            ),
        )
    }

    fn canonical_cosine_symetric(&self, rhs: &Self) -> Option<Self::Scalar> {
        let geometric_cosine = self.canonical_geometric_cosine_symetric(&rhs);

        if geometric_cosine.1.volume().is_near_zero() {
            None
        } else {
            Some(geometric_cosine.0 / geometric_cosine.1.volume())
        }
    }
}
