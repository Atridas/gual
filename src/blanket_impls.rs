use num::{Float, FromPrimitive};

use crate::{Antiscalar, Attitude, Complement, Contraction, Dot, Epsilon, Expansion, Metric};

impl<T> Contraction<T> for T
where
    T: Dot,
    <T as Dot>::Antiscalar: Complement<Output = <T as Dot>::Scalar>,
{
    type BulkOutput = <T as Dot>::Scalar;
    type WeightOutput = <T as Dot>::Scalar;

    fn bulk_contraction(&self, rhs: &T) -> Self::BulkOutput {
        self.dot(rhs)
    }

    fn weight_contraction(&self, rhs: &T) -> Self::WeightOutput {
        self.antidot(rhs).right_complement()
    }
}

impl<T> Expansion<T> for T
where
    T: Dot,
    <T as Dot>::Antiscalar: Antiscalar<T = <T as Dot>::Scalar>,
{
    type BulkOutput = <T as Dot>::Antiscalar;
    type WeightOutput = <T as Dot>::Antiscalar;

    fn bulk_expansion(&self, rhs: &T) -> Self::BulkOutput {
        <T as Dot>::Antiscalar::from_volume(self.dot(rhs))
    }

    fn weight_expansion(&self, rhs: &T) -> Self::WeightOutput {
        self.antidot(rhs)
    }
}

impl<T> Attitude for T
where
    T: Metric,
{
    type Output = <T as Metric>::Weight;

    fn attitude(&self) -> Self::Output {
        self.weight()
    }
}

impl<T: Float + FromPrimitive + PartialOrd> Epsilon for T {
    #[inline(always)]
    fn eps() -> Self {
        T::from_f32(0.001).expect("expected T to be a floating point type")
    }

    #[inline(always)]
    fn is_near_zero(&self) -> bool {
        self.abs() < Self::eps()
    }
}
