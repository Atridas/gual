use std::marker::PhantomData;

use super::{Bivector, Vector};

impl<T: Clone, M> Clone for Vector<T, M> {
    fn clone(&self) -> Self {
        Vector {
            x: self.x.clone(),
            y: self.y.clone(),
            _metric: PhantomData,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.x.clone_from(&source.x);
        self.y.clone_from(&source.y);
    }
}

impl<T: Clone, M> Clone for Bivector<T, M> {
    fn clone(&self) -> Self {
        Bivector {
            xy: self.xy.clone(),
            _metric: PhantomData,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.xy.clone_from(&source.xy);
    }
}

impl<T: Copy, M> Copy for Vector<T, M> {}
impl<T: Copy, M> Copy for Bivector<T, M> {}
