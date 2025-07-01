// @TODO(Atridas): replace the reverse_add with this
#[macro_export]
macro_rules! reverse_add_metric {
    ($lht:ident, $rht:ident) => {
        impl<T, M> Add<$rht<T, M>> for $lht<T, M>
        where
            $rht<T, M>: Add<$lht<T, M>>,
        {
            type Output = <$rht<T, M> as Add<$lht<T, M>>>::Output;

            fn add(self, rhs: $rht<T, M>) -> Self::Output {
                rhs + self
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_add_scalar_metric {
    ($rht:ident) => {
        impl<const D: u32, T, M> Add<$rht<T, M>> for Scalar<D, T, M>
        where
            $rht<T, M>: Add<Scalar<D, T, M>>,
        {
            type Output = <$rht<T, M> as Add<Scalar<D, T, M>>>::Output;

            fn add(self, rhs: $rht<T, M>) -> Self::Output {
                rhs + self
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_mul_scalar_metric {
    ($rht:ident) => {
        impl<const D: u32, T, M> Mul<$rht<T, M>> for Scalar<D, T, M>
        where
            $rht<T, M>: Mul<Scalar<D, T, M>>,
        {
            type Output = <$rht<T, M> as Mul<Scalar<D, T, M>>>::Output;

            fn mul(self, rhs: $rht<T, M>) -> Self::Output {
                rhs * self
            }
        }
    };
}

#[macro_export]
macro_rules! default_sub {
    ($lht:ident, $rht:ident) => {
        impl<T, M> Sub<$rht<T, M>> for $lht<T, M>
        where
            $lht<T, M>: Add<$rht<T, M>>,
            $rht<T, M>: Neg<Output = $rht<T, M>>,
        {
            type Output = <$lht<T, M> as Add<$rht<T, M>>>::Output;

            fn sub(self, rhs: $rht<T, M>) -> Self::Output {
                self + -rhs
            }
        }
    };
}

#[macro_export]
macro_rules! wedge_scalar {
    ($v:ident, $d:literal) => {
        impl<T, M> WedgeProduct<$v<T, M>> for T
        where
            T: Copy,
            $v<T, M>: Copy,
            $v<T, M>: Mul<T, Output = $v<T, M>>,
        {
            type Output = $v<T, M>;
            fn wedge(&self, rhs: &$v<T, M>) -> Self::Output {
                *rhs * *self
            }
        }

        impl<T, M> WedgeProduct<T> for $v<T, M>
        where
            T: Copy,
            $v<T, M>: Copy,
            $v<T, M>: Mul<T, Output = $v<T, M>>,
        {
            type Output = $v<T, M>;
            fn wedge(&self, rhs: &T) -> Self::Output {
                *self * *rhs
            }
        }

        impl<T, M> WedgeProduct<$v<T, M>> for Scalar<$d, T, M>
        where
            T: Copy,
            $v<T, M>: Copy,
            $v<T, M>: Mul<T, Output = $v<T, M>>,
        {
            type Output = $v<T, M>;
            fn wedge(&self, rhs: &$v<T, M>) -> Self::Output {
                *rhs * self.0
            }
        }

        impl<T, M> WedgeProduct<Scalar<$d, T, M>> for $v<T, M>
        where
            T: Copy,
            $v<T, M>: Copy,
            $v<T, M>: Mul<T, Output = $v<T, M>>,
        {
            type Output = $v<T, M>;
            fn wedge(&self, rhs: &Scalar<$d, T, M>) -> Self::Output {
                *self * rhs.0
            }
        }
    };
}

#[macro_export]
macro_rules! null_wedge {
    ($lht:ident, $rht:ident) => {
        impl<T, M> WedgeProduct<$rht<T, M>> for $lht<T, M> {
            type Output = ();
            fn wedge(&self, _: &$rht<T, M>) -> Self::Output {}
        }
    };
}

// @TODO(Atridas): replace the reverse_add with this
#[macro_export]
macro_rules! reverse_wedge_metric {
    ($lht:ident, $rht:ident) => {
        impl<T, M> WedgeProduct<$rht<T, M>> for $lht<T, M>
        where
            $lht<T, M>: Copy,
            $rht<T, M>: WedgeProduct<$lht<T, M>>,
        {
            type Output = <$rht<T, M> as WedgeProduct<$lht<T, M>>>::Output;

            fn wedge(&self, rhs: &$rht<T, M>) -> Self::Output {
                rhs.wedge(self)
            }
        }
    };
}

// @TODO(Atridas): replace the reverse_add with this
#[macro_export]
macro_rules! reverse_wedge_anticommutative_metric {
    ($lht:ident, $rht:ident) => {
        impl<T, M> WedgeProduct<$rht<T, M>> for $lht<T, M>
        where
            $rht<T, M>: WedgeProduct<$lht<T, M>>,
            $lht<T, M>: Neg<Output = $lht<T, M>>,
        {
            type Output = <$rht<T, M> as WedgeProduct<$lht<T, M>>>::Output;

            fn wedge(&self, rhs: $rht<T, M>) -> Self::Output {
                rhs.wedge(-self)
            }
        }
    };
}

#[macro_export]
macro_rules! null_antiwedge {
    ($lht:ident, $rht:ident) => {
        impl<T, M> AntiwedgeProduct<$rht<T, M>> for $lht<T, M> {
            type Output = ();
            fn antiwedge(&self, _: &$rht<T, M>) -> Self::Output {}
        }
    };
}

// @TODO(Atridas): replace the reverse_add with this
#[macro_export]
macro_rules! reverse_antiwedge_metric {
    ($lht:ident, $rht:ident) => {
        impl<T, M> AntiwedgeProduct<$rht<T, M>> for $lht<T, M>
        where
            $lht<T, M>: Copy,
            $rht<T, M>: AntiwedgeProduct<$lht<T, M>>,
        {
            type Output = <$rht<T, M> as AntiwedgeProduct<$lht<T, M>>>::Output;

            fn antiwedge(&self, rhs: &$rht<T, M>) -> Self::Output {
                rhs.antiwedge(self)
            }
        }
    };
}

// @TODO(Atridas): replace the reverse_add with this
#[macro_export]
macro_rules! reverse_antiwedge_anticommutative_metric {
    ($lht:ident, $rht:ident) => {
        impl<T, M> AntiwedgeProduct<$rht<T, M>> for $lht<T, M>
        where
            $rht<T, M>: AntiwedgeProduct<$lht<T, M>>,
            $lht<T, M>: Neg<Output = $lht<T, M>>,
        {
            type Output = <$rht<T, M> as AntiwedgeProduct<$lht<T, M>>>::Output;

            fn antiwedge(&self, rhs: $rht<T, M>) -> Self::Output {
                rhs.antiwedge(-self)
            }
        }
    };
}

#[macro_export]
macro_rules! geometric_with_scalar_metric {
    ($lht:ident) => {
        impl<T, M> GeometricProduct<T> for $lht<T, M>
        where
            T: GeometricProduct<$lht<T, M>, Output = $lht<T, M>>,
        {
            type Output = $lht<T, M>;

            fn geometric_product(&self, rhs: &T) -> Self::Output {
                rhs.geometric_product(self)
            }
        }
    };
}

#[macro_export]
macro_rules! geometric_with_scalar {
    ($lht:ident) => {
        impl<T> GeometricProduct<T> for $lht<T>
        where
            T: GeometricProduct<$lht<T>, Output = $lht<T>>,
        {
            type Output = $lht<T>;

            fn geometric_product(&self, rhs: &T) -> Self::Output {
                rhs.geometric_product(self)
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_geometric_metric {
    ($lht:ident, $rht:ident) => {
        impl<T, M> GeometricProduct<$rht<T, M>> for $lht<T, M>
        where
            $rht<T, M>: GeometricProduct<$lht<T, M>>,
        {
            type Output = <$rht<T, M> as GeometricProduct<$lht<T, M>>>::Output;

            fn geometric_product(&self, rhs: &$rht<T, M>) -> Self::Output {
                rhs.geometric_product(self)
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_geometric_anticommute_metric {
    ($lht:ident, $rht:ident) => {
        impl<T, M> GeometricProduct<rht<T, M>> for $lht<T, M>
        where
            $rht<T, M>: GeometricProduct<$lht<T, M>>,
            GeometricProduct<$lht<T, M>>::Output: Neg<Output = GeometricProduct<$lht<T, M>>>::Output>,
        {
            type Output = <$rht<T, M> as GeometricProduct<$lht<T, M>>>::Output;

            fn geometric_product(&self, rhs: &T) -> Self::Output {
                -rhs.geometric_product(self)
            }
        }
    };
}

#[macro_export]
macro_rules! reverse_angle {
    ($lht:ident<T>, $rht:ident<T>) => {
        impl<T> Angle<$rht<T>> for $lht<T>
        where
            $rht<T>: Angle<$lht<T>>,
        {
            type Scalar = <$rht<T> as Angle<$lht<T>>>::Scalar;
            type Antiscalar = <$rht<T> as Angle<$lht<T>>>::Antiscalar;

            fn geometric_cosine(&self, rhs: &$rht<T>) -> (Self::Scalar, Self::Antiscalar) {
                rhs.geometric_cosine(self)
            }

            fn cosine(&self, rhs: &$rht<T>) -> Option<Self::Scalar> {
                rhs.cosine(self)
            }
        }
    };

    ($lht:ident<T, $metric:ident>, $rht:ident<T>) => {
        impl<T> Angle<$rht<T>> for $lht<T, $metric>
        where
            $rht<T>: Angle<$lht<T, $metric>>,
        {
            type Scalar = <$rht<T> as Angle<$lht<T, $metric>>>::Scalar;
            type Antiscalar = <$rht<T> as Angle<$lht<T, $metric>>>::Antiscalar;

            fn geometric_cosine(&self, rhs: &$rht<T>) -> (Self::Scalar, Self::Antiscalar) {
                rhs.geometric_cosine(self)
            }

            fn cosine(&self, rhs: &$rht<T>) -> Option<Self::Scalar> {
                rhs.cosine(self)
            }
        }
    };

    ($lht:ident<T>, $rht:ident<T, $metric:ident>) => {
        impl<T> Angle<$rht<T, $metric>> for $lht<T>
        where
            $rht<T, $metric>: Angle<$lht<T>>,
        {
            type Scalar = <$rht<T, $metric> as Angle<$lht<T>>>::Scalar;
            type Antiscalar = <$rht<T, $metric> as Angle<$lht<T>>>::Antiscalar;

            fn geometric_cosine(&self, rhs: &$rht<T, $metric>) -> (Self::Scalar, Self::Antiscalar) {
                rhs.geometric_cosine(self)
            }

            fn cosine(&self, rhs: &$rht<T, $metric>) -> Option<Self::Scalar> {
                rhs.cosine(self)
            }
        }
    };

    ($lht:ident<T, $lhmetric:ident>, $rht:ident<T, $rhmetric:ident>) => {
        impl<T> Angle<$rht<T, $rhmetric>> for $lht<T, $lhmetric>
        where
            $rht<T, $rhmetric>: Angle<$lht<T, $lhmetric>>,
        {
            type Scalar = <$rht<T, $rhmetric> as Angle<$lht<T, $lhmetric>>>::Scalar;
            type Antiscalar = <$rht<T, $rhmetric> as Angle<$lht<T, $lhmetric>>>::Antiscalar;

            fn geometric_cosine(
                &self,
                rhs: &$rht<T, $rhmetric>,
            ) -> (Self::Scalar, Self::Antiscalar) {
                rhs.geometric_cosine(self)
            }

            fn cosine(&self, rhs: &$rht<T, $rhmetric>) -> Option<Self::Scalar> {
                rhs.cosine(self)
            }
        }
    };
}
