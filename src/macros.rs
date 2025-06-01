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
    ($v:ident) => {
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
