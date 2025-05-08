use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use num::Float;
use num::Zero;
use num::traits::ConstZero;

use crate::Epsilon;
use crate::Expansion;
use crate::geometry3d as d3;
use crate::geometry4d as d4;

use super::HomogeneusLine;
use super::HomogeneusPlane;
use super::HomogeneusPoint;
use super::HorizonLine;
use super::Line;
use super::Plane;

impl<T> Expansion<HomogeneusPoint<T>> for d4::Scalar<T>
where
    T: Copy,
    T: ConstZero,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusPlane<T>;
    type WeightOutput = HomogeneusPlane<T>;

    fn bulk_expansion(&self, rhs: &HomogeneusPoint<T>) -> Self::BulkOutput {
        HomogeneusPlane {
            wyz: self.0 * rhs.x,
            wzx: self.0 * rhs.y,
            wxy: self.0 * rhs.z,
            zyx: T::ZERO,
        }
    }

    fn weight_expansion(&self, rhs: &HomogeneusPoint<T>) -> Self::WeightOutput {
        HomogeneusPlane {
            wyz: T::ZERO,
            wzx: T::ZERO,
            wxy: T::ZERO,
            zyx: self.0 * rhs.w,
        }
    }
}

impl<T> Expansion<d3::Point<T>> for d4::Scalar<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<Plane<T>>;
    type WeightOutput = ();

    fn bulk_expansion(&self, rhs: &d3::Point<T>) -> Self::BulkOutput {
        if self.is_zero() {
            None
        } else {
            let len2 = rhs.0.x * rhs.0.x + rhs.0.y * rhs.0.y + rhs.0.z * rhs.0.z;
            if len2.is_near_zero() {
                None
            } else {
                let invlen = len2.sqrt().recip();
                Some(Plane(d4::Trivector {
                    wyz: rhs.0.x * invlen,
                    wzx: rhs.0.y * invlen,
                    wxy: rhs.0.z * invlen,
                    zyx: T::ZERO,
                }))
            }
        }
    }

    fn weight_expansion(&self, _rhs: &d3::Point<T>) -> Self::WeightOutput {}
}

impl<T> Expansion<d3::Vector<T>> for d4::Scalar<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<Plane<T>>;
    type WeightOutput = ();

    fn bulk_expansion(&self, rhs: &d3::Vector<T>) -> Self::BulkOutput {
        if self.is_zero() {
            None
        } else {
            let len2 = rhs.x * rhs.x + rhs.y * rhs.y + rhs.z * rhs.z;
            if len2.is_near_zero() {
                None
            } else {
                let invlen = len2.sqrt().recip();
                Some(Plane(d4::Trivector {
                    wyz: rhs.x * invlen,
                    wzx: rhs.y * invlen,
                    wxy: rhs.z * invlen,
                    zyx: T::ZERO,
                }))
            }
        }
    }

    fn weight_expansion(&self, _rhs: &d3::Vector<T>) -> Self::WeightOutput {}
}

impl<T> Expansion<HomogeneusLine<T>> for d4::Scalar<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusLine<T>;
    type WeightOutput = HomogeneusLine<T>;

    fn bulk_expansion(&self, rhs: &HomogeneusLine<T>) -> Self::BulkOutput {
        HomogeneusLine {
            wx: -self.0 * rhs.yz,
            wy: -self.0 * rhs.zx,
            wz: -self.0 * rhs.xy,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        }
    }

    fn weight_expansion(&self, rhs: &HomogeneusLine<T>) -> Self::WeightOutput {
        HomogeneusLine {
            wx: T::ZERO,
            wy: T::ZERO,
            wz: T::ZERO,
            yz: -self.0 * rhs.wx,
            zx: -self.0 * rhs.wy,
            xy: -self.0 * rhs.wz,
        }
    }
}

impl<T> Expansion<Line<T>> for d4::Scalar<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<Line<T>>;
    type WeightOutput = Option<HorizonLine<T>>;

    fn bulk_expansion(&self, rhs: &Line<T>) -> Self::BulkOutput {
        if self.0.is_zero() {
            None
        } else {
            let len2 = rhs.0.yz * rhs.0.yz + rhs.0.zx * rhs.0.zx + rhs.0.xy * rhs.0.xy;
            if len2.is_near_zero() {
                None
            } else {
                let mut invlen = len2.sqrt().recip();
                if self.0.is_sign_positive() {
                    invlen = -invlen;
                }
                Some(Line(HomogeneusLine {
                    wx: rhs.0.yz * invlen,
                    wy: rhs.0.zx * invlen,
                    wz: rhs.0.xy * invlen,
                    yz: T::ZERO,
                    zx: T::ZERO,
                    xy: T::ZERO,
                }))
            }
        }
    }

    fn weight_expansion(&self, rhs: &Line<T>) -> Self::WeightOutput {
        if self.0.is_zero() {
            None
        } else {
            if self.0.is_sign_positive() {
                Some(HorizonLine(d3::Bivector {
                    yz: -rhs.0.wx,
                    zx: -rhs.0.wy,
                    xy: -rhs.0.wz,
                }))
            } else {
                Some(HorizonLine(d3::Bivector {
                    yz: rhs.0.wx,
                    zx: rhs.0.wy,
                    xy: rhs.0.wz,
                }))
            }
        }
    }
}

impl<T> Expansion<HorizonLine<T>> for d4::Scalar<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
{
    type BulkOutput = Option<Line<T>>;
    type WeightOutput = ();

    fn bulk_expansion(&self, rhs: &HorizonLine<T>) -> Self::BulkOutput {
        if self.0.is_zero() {
            None
        } else {
            if self.0.is_sign_positive() {
                Some(Line(HomogeneusLine {
                    wx: -rhs.0.yz,
                    wy: -rhs.0.zx,
                    wz: -rhs.0.xy,
                    yz: T::ZERO,
                    zx: T::ZERO,
                    xy: T::ZERO,
                }))
            } else {
                Some(Line(HomogeneusLine {
                    wx: rhs.0.yz,
                    wy: rhs.0.zx,
                    wz: rhs.0.xy,
                    yz: T::ZERO,
                    zx: T::ZERO,
                    xy: T::ZERO,
                }))
            }
        }
    }

    fn weight_expansion(&self, _rhs: &HorizonLine<T>) -> Self::WeightOutput {}
}

impl<T> Expansion<HomogeneusPlane<T>> for d4::Scalar<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusPoint<T>;
    type WeightOutput = HomogeneusPoint<T>;

    fn bulk_expansion(&self, rhs: &HomogeneusPlane<T>) -> Self::BulkOutput {
        HomogeneusPoint {
            x: -self.0 * rhs.wyz,
            y: -self.0 * rhs.wzx,
            z: -self.0 * rhs.wxy,
            w: T::ZERO,
        }
    }

    fn weight_expansion(&self, rhs: &HomogeneusPlane<T>) -> Self::WeightOutput {
        HomogeneusPoint {
            x: T::ZERO,
            y: T::ZERO,
            z: T::ZERO,
            w: -self.0 * rhs.zyx,
        }
    }
}

impl<T> Expansion<Plane<T>> for d4::Scalar<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type BulkOutput = d3::Vector<T>;
    type WeightOutput = Option<d3::Point<T>>;

    fn bulk_expansion(&self, rhs: &Plane<T>) -> Self::BulkOutput {
        d3::Vector {
            x: -self.0 * rhs.0.wyz,
            y: -self.0 * rhs.0.wzx,
            z: -self.0 * rhs.0.wxy,
        }
    }

    fn weight_expansion(&self, _rhs: &Plane<T>) -> Self::WeightOutput {
        if self.is_zero() {
            None
        } else {
            Some(d3::Point::ZERO)
        }
    }
}

impl<T> Expansion<HomogeneusLine<T>> for HomogeneusPoint<T>
where
    T: Copy,
    T: ConstZero,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusPlane<T>;
    type WeightOutput = HomogeneusPlane<T>;

    fn bulk_expansion(&self, rhs: &HomogeneusLine<T>) -> Self::BulkOutput {
        HomogeneusPlane {
            wyz: self.y * rhs.xy - self.z * rhs.zx,
            wzx: self.z * rhs.yz - self.x * rhs.xy,
            wxy: self.x * rhs.zx - self.y * rhs.yz,
            zyx: T::ZERO,
        }
    }

    fn weight_expansion(&self, rhs: &HomogeneusLine<T>) -> Self::WeightOutput {
        HomogeneusPlane {
            wyz: -self.w * rhs.wx,
            wzx: -self.w * rhs.wy,
            wxy: -self.w * rhs.wz,
            zyx: self.x * rhs.wx + self.y * rhs.wy + self.z * rhs.wz,
        }
    }
}

impl<T> Expansion<Line<T>> for d3::Point<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<Plane<T>>;
    type WeightOutput = Plane<T>;

    fn bulk_expansion(&self, rhs: &Line<T>) -> Self::BulkOutput {
        let plane = HomogeneusPlane {
            wyz: self.0.y * rhs.0.xy - self.0.z * rhs.0.zx,
            wzx: self.0.z * rhs.0.yz - self.0.x * rhs.0.xy,
            wxy: self.0.x * rhs.0.zx - self.0.y * rhs.0.yz,
            zyx: T::ZERO,
        };

        let len2 = plane.wyz * plane.wyz + plane.wzx * plane.wzx + plane.wxy * plane.wxy;
        if len2.is_near_zero() {
            None
        } else {
            Some(Plane(plane * d4::Scalar(len2.sqrt().recip())))
        }
    }

    fn weight_expansion(&self, rhs: &Line<T>) -> Self::WeightOutput {
        Plane(d4::Trivector {
            wyz: -rhs.0.wx,
            wzx: -rhs.0.wy,
            wxy: -rhs.0.wz,
            zyx: self.0.x * rhs.0.wx + self.0.y * rhs.0.wy + self.0.z * rhs.0.wz,
        })
    }
}

impl<T> Expansion<HorizonLine<T>> for d3::Point<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<Plane<T>>;
    type WeightOutput = ();

    fn bulk_expansion(&self, rhs: &HorizonLine<T>) -> Self::BulkOutput {
        let plane = HomogeneusPlane {
            wyz: self.0.y * rhs.0.xy - self.0.z * rhs.0.zx,
            wzx: self.0.z * rhs.0.yz - self.0.x * rhs.0.xy,
            wxy: self.0.x * rhs.0.zx - self.0.y * rhs.0.yz,
            zyx: T::ZERO,
        };

        let len2 = plane.wyz * plane.wyz + plane.wzx * plane.wzx + plane.wxy * plane.wxy;
        if len2.is_near_zero() {
            None
        } else {
            Some(Plane(plane * d4::Scalar(len2.sqrt().recip())))
        }
    }

    fn weight_expansion(&self, _rhs: &HorizonLine<T>) -> Self::WeightOutput {}
}

impl<T> Expansion<Line<T>> for d3::Vector<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<Plane<T>>;
    type WeightOutput = ();

    fn bulk_expansion(&self, rhs: &Line<T>) -> Self::BulkOutput {
        let plane = HomogeneusPlane {
            wyz: self.y * rhs.0.xy - self.z * rhs.0.zx,
            wzx: self.z * rhs.0.yz - self.x * rhs.0.xy,
            wxy: self.x * rhs.0.zx - self.y * rhs.0.yz,
            zyx: T::ZERO,
        };

        let len2 = plane.wyz * plane.wyz + plane.wzx * plane.wzx + plane.wxy * plane.wxy;
        if len2.is_near_zero() {
            None
        } else {
            Some(Plane(plane * d4::Scalar(len2.sqrt().recip())))
        }
    }

    fn weight_expansion(&self, _rhs: &Line<T>) -> Self::WeightOutput {}
}

impl<T> Expansion<HorizonLine<T>> for d3::Vector<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<Plane<T>>;
    type WeightOutput = ();

    fn bulk_expansion(&self, rhs: &HorizonLine<T>) -> Self::BulkOutput {
        let plane = HomogeneusPlane {
            wyz: self.y * rhs.0.xy - self.z * rhs.0.zx,
            wzx: self.z * rhs.0.yz - self.x * rhs.0.xy,
            wxy: self.x * rhs.0.zx - self.y * rhs.0.yz,
            zyx: T::ZERO,
        };

        let len2 = plane.wyz * plane.wyz + plane.wzx * plane.wzx + plane.wxy * plane.wxy;
        if len2.is_near_zero() {
            None
        } else {
            Some(Plane(plane * d4::Scalar(len2.sqrt().recip())))
        }
    }

    fn weight_expansion(&self, _rhs: &HorizonLine<T>) -> Self::WeightOutput {}
}

impl<T> Expansion<HomogeneusPlane<T>> for HomogeneusPoint<T>
where
    T: Copy,
    T: ConstZero,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusLine<T>;
    type WeightOutput = HomogeneusLine<T>;

    fn bulk_expansion(&self, rhs: &HomogeneusPlane<T>) -> Self::BulkOutput {
        HomogeneusLine {
            wx: self.x * rhs.zyx,
            wy: self.y * rhs.zyx,
            wz: self.z * rhs.zyx,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        }
    }

    fn weight_expansion(&self, rhs: &HomogeneusPlane<T>) -> Self::WeightOutput {
        HomogeneusLine {
            wx: -self.w * rhs.wyz,
            wy: -self.w * rhs.wzx,
            wz: -self.w * rhs.wxy,
            yz: self.z * rhs.wzx - self.y * rhs.wxy,
            zx: self.x * rhs.wxy - self.z * rhs.wyz,
            xy: self.y * rhs.wyz - self.x * rhs.wzx,
        }
    }
}

impl<T> Expansion<Plane<T>> for d3::Point<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<Line<T>>;
    type WeightOutput = Line<T>;

    fn bulk_expansion(&self, rhs: &Plane<T>) -> Self::BulkOutput {
        if rhs.0.zyx.is_zero() {
            None
        } else {
            let len2 = self.0.x * self.0.x + self.0.y * self.0.y + self.0.z * self.0.z;
            if len2.is_near_zero() {
                None
            } else {
                let invlen = len2.sqrt().recip();
                Some(if rhs.0.zyx.is_sign_positive() {
                    Line(HomogeneusLine {
                        wx: self.0.x * invlen,
                        wy: self.0.y * invlen,
                        wz: self.0.z * invlen,
                        yz: T::ZERO,
                        zx: T::ZERO,
                        xy: T::ZERO,
                    })
                } else {
                    Line(HomogeneusLine {
                        wx: -invlen * self.0.x,
                        wy: -invlen * self.0.y,
                        wz: -invlen * self.0.z,
                        yz: T::ZERO,
                        zx: T::ZERO,
                        xy: T::ZERO,
                    })
                })
            }
        }
    }

    fn weight_expansion(&self, rhs: &Plane<T>) -> Self::WeightOutput {
        Line(HomogeneusLine {
            wx: -rhs.0.wyz,
            wy: -rhs.0.wzx,
            wz: -rhs.0.wxy,
            yz: self.0.z * rhs.0.wzx - self.0.y * rhs.0.wxy,
            zx: self.0.x * rhs.0.wxy - self.0.z * rhs.0.wyz,
            xy: self.0.y * rhs.0.wyz - self.0.x * rhs.0.wzx,
        })
    }
}

impl<T> Expansion<Plane<T>> for d3::Vector<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<Line<T>>;
    type WeightOutput = Option<HorizonLine<T>>;

    fn bulk_expansion(&self, rhs: &Plane<T>) -> Self::BulkOutput {
        if rhs.0.zyx.is_zero() {
            None
        } else {
            let len2 = self.x * self.x + self.y * self.y + self.z * self.z;
            if len2.is_near_zero() {
                None
            } else {
                let invlen = len2.sqrt().recip();
                Some(if rhs.0.zyx.is_sign_positive() {
                    Line(HomogeneusLine {
                        wx: self.x * invlen,
                        wy: self.y * invlen,
                        wz: self.z * invlen,
                        yz: T::ZERO,
                        zx: T::ZERO,
                        xy: T::ZERO,
                    })
                } else {
                    Line(HomogeneusLine {
                        wx: -invlen * self.x,
                        wy: -invlen * self.y,
                        wz: -invlen * self.z,
                        yz: T::ZERO,
                        zx: T::ZERO,
                        xy: T::ZERO,
                    })
                })
            }
        }
    }

    fn weight_expansion(&self, rhs: &Plane<T>) -> Self::WeightOutput {
        let bivector = d3::Bivector {
            yz: self.z * rhs.0.wzx - self.y * rhs.0.wxy,
            zx: self.x * rhs.0.wxy - self.z * rhs.0.wyz,
            xy: self.y * rhs.0.wyz - self.x * rhs.0.wzx,
        };
        let len2 =
            bivector.yz * bivector.yz + bivector.zx * bivector.zx + bivector.xy * bivector.xy;
        if len2.is_near_zero() {
            None
        } else {
            Some(HorizonLine(bivector * d3::Scalar(len2.sqrt().recip())))
        }
    }
}

impl<T> Expansion<d4::Quadvector<T>> for HomogeneusPoint<T>
where
    T: Copy,
    T: ConstZero,
    T: Mul<T, Output = T>,
{
    type BulkOutput = ();
    type WeightOutput = HomogeneusPoint<T>;

    fn bulk_expansion(&self, _rhs: &d4::Quadvector<T>) -> Self::BulkOutput {}

    fn weight_expansion(&self, rhs: &d4::Quadvector<T>) -> Self::WeightOutput {
        HomogeneusPoint {
            x: self.x * rhs.xyzw,
            y: self.y * rhs.xyzw,
            z: self.z * rhs.xyzw,
            w: self.w * rhs.xyzw,
        }
    }
}

impl<T> Expansion<d4::Quadvector<T>> for d3::Point<T>
where
    T: Copy,
    T: ConstZero,
{
    type BulkOutput = ();
    type WeightOutput = Option<d3::Point<T>>;

    fn bulk_expansion(&self, _rhs: &d4::Quadvector<T>) -> Self::BulkOutput {}

    fn weight_expansion(&self, rhs: &d4::Quadvector<T>) -> Self::WeightOutput {
        if rhs.is_zero() { None } else { Some(*self) }
    }
}

impl<T> Expansion<d4::Quadvector<T>> for d3::Vector<T>
where
    T: Copy,
    T: ConstZero,
    T: Mul<T, Output = T>,
{
    type BulkOutput = ();
    type WeightOutput = d3::Vector<T>;

    fn bulk_expansion(&self, _rhs: &d4::Quadvector<T>) -> Self::BulkOutput {}

    fn weight_expansion(&self, rhs: &d4::Quadvector<T>) -> Self::WeightOutput {
        d3::Vector {
            x: self.x * rhs.xyzw,
            y: self.y * rhs.xyzw,
            z: self.z * rhs.xyzw,
        }
    }
}

impl<T> Expansion<HomogeneusPlane<T>> for HomogeneusLine<T>
where
    T: Copy,
    T: ConstZero,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusPlane<T>;
    type WeightOutput = HomogeneusPlane<T>;

    fn bulk_expansion(&self, rhs: &HomogeneusPlane<T>) -> Self::BulkOutput {
        HomogeneusPlane {
            wyz: -self.yz * rhs.zyx,
            wzx: -self.zx * rhs.zyx,
            wxy: -self.xy * rhs.zyx,
            zyx: T::ZERO,
        }
    }

    fn weight_expansion(&self, rhs: &HomogeneusPlane<T>) -> Self::WeightOutput {
        HomogeneusPlane {
            wyz: self.wz * rhs.wzx - self.wy * rhs.wxy,
            wzx: self.wx * rhs.wxy - self.wz * rhs.wyz,
            wxy: self.wy * rhs.wyz - self.wx * rhs.wzx,
            zyx: self.yz * rhs.wyz + self.zx * rhs.wzx + self.xy * rhs.wxy,
        }
    }
}

impl<T> Expansion<Plane<T>> for Line<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<Plane<T>>;
    type WeightOutput = Option<Plane<T>>;

    fn bulk_expansion(&self, rhs: &Plane<T>) -> Self::BulkOutput {
        if rhs.0.zyx.is_zero() {
            None
        } else {
            let len2 = self.0.yz * self.0.yz + self.0.zx * self.0.zx + self.0.xy * self.0.xy;
            if len2.is_near_zero() {
                None
            } else {
                let mut invlen = len2.sqrt().recip();
                if rhs.0.zyx.is_sign_positive() {
                    invlen = -invlen;
                }
                Some(Plane(d4::Trivector {
                    wyz: self.0.yz * invlen,
                    wzx: self.0.zx * invlen,
                    wxy: self.0.xy * invlen,
                    zyx: T::ZERO,
                }))
            }
        }
    }

    fn weight_expansion(&self, rhs: &Plane<T>) -> Self::WeightOutput {
        let hp = HomogeneusPlane {
            wyz: self.0.wz * rhs.0.wzx - self.0.wy * rhs.0.wxy,
            wzx: self.0.wx * rhs.0.wxy - self.0.wz * rhs.0.wyz,
            wxy: self.0.wy * rhs.0.wyz - self.0.wx * rhs.0.wzx,
            zyx: self.0.yz * rhs.0.wyz + self.0.zx * rhs.0.wzx + self.0.xy * rhs.0.wxy,
        };

        let len2 = hp.wyz * hp.wyz + hp.wzx * hp.wzx + hp.wxy * hp.wxy;
        if len2.is_near_zero() {
            None
        } else {
            Some(Plane(hp * d4::Scalar(len2.sqrt().recip())))
        }
    }
}

impl<T> Expansion<Plane<T>> for HorizonLine<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<Plane<T>>;
    type WeightOutput = ();

    fn bulk_expansion(&self, rhs: &Plane<T>) -> Self::BulkOutput {
        if rhs.0.zyx.is_zero() {
            None
        } else {
            let len2 = self.0.yz * self.0.yz + self.0.zx * self.0.zx + self.0.xy * self.0.xy;
            if len2.is_near_zero() {
                None
            } else {
                let mut invlen = len2.sqrt().recip();
                if rhs.0.zyx.is_sign_positive() {
                    invlen = -invlen;
                }
                Some(Plane(d4::Trivector {
                    wyz: self.0.yz * invlen,
                    wzx: self.0.zx * invlen,
                    wxy: self.0.xy * invlen,
                    zyx: T::ZERO,
                }))
            }
        }
    }

    fn weight_expansion(&self, _rhs: &Plane<T>) -> Self::WeightOutput {}
}

impl<T> Expansion<d4::Quadvector<T>> for HomogeneusLine<T>
where
    T: Copy,
    T: ConstZero,
    T: Mul<T, Output = T>,
{
    type BulkOutput = ();
    type WeightOutput = HomogeneusLine<T>;

    fn bulk_expansion(&self, _rhs: &d4::Quadvector<T>) -> Self::BulkOutput {}

    fn weight_expansion(&self, rhs: &d4::Quadvector<T>) -> Self::WeightOutput {
        HomogeneusLine {
            wx: self.wx * rhs.xyzw,
            wy: self.wy * rhs.xyzw,
            wz: self.wz * rhs.xyzw,
            yz: self.yz * rhs.xyzw,
            zx: self.zx * rhs.xyzw,
            xy: self.xy * rhs.xyzw,
        }
    }
}

impl<T> Expansion<d4::Quadvector<T>> for Line<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
{
    type BulkOutput = ();
    type WeightOutput = Option<Line<T>>;

    fn bulk_expansion(&self, _rhs: &d4::Quadvector<T>) -> Self::BulkOutput {}

    fn weight_expansion(&self, rhs: &d4::Quadvector<T>) -> Self::WeightOutput {
        if rhs.is_zero() {
            None
        } else if rhs.xyzw.is_sign_positive() {
            Some(*self)
        } else {
            Some(Line(HomogeneusLine {
                wx: -self.0.wx,
                wy: -self.0.wy,
                wz: -self.0.wz,
                yz: -self.0.yz,
                zx: -self.0.zx,
                xy: -self.0.xy,
            }))
        }
    }
}

impl<T> Expansion<d4::Quadvector<T>> for HorizonLine<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
{
    type BulkOutput = ();
    type WeightOutput = Option<HorizonLine<T>>;

    fn bulk_expansion(&self, _rhs: &d4::Quadvector<T>) -> Self::BulkOutput {}

    fn weight_expansion(&self, rhs: &d4::Quadvector<T>) -> Self::WeightOutput {
        if rhs.is_zero() {
            None
        } else if rhs.xyzw.is_sign_positive() {
            Some(*self)
        } else {
            Some(HorizonLine(d3::Bivector {
                yz: -self.0.yz,
                zx: -self.0.zx,
                xy: -self.0.xy,
            }))
        }
    }
}

impl<T> Expansion<d4::Quadvector<T>> for HomogeneusPlane<T>
where
    T: Copy,
    T: ConstZero,
    T: Mul<T, Output = T>,
{
    type BulkOutput = ();
    type WeightOutput = HomogeneusPlane<T>;

    fn bulk_expansion(&self, _rhs: &d4::Quadvector<T>) -> Self::BulkOutput {}

    fn weight_expansion(&self, rhs: &d4::Quadvector<T>) -> Self::WeightOutput {
        HomogeneusPlane {
            wyz: self.wyz * rhs.xyzw,
            wzx: self.wzx * rhs.xyzw,
            wxy: self.wxy * rhs.xyzw,
            zyx: self.zyx * rhs.xyzw,
        }
    }
}

impl<T> Expansion<d4::Quadvector<T>> for Plane<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
{
    type BulkOutput = ();
    type WeightOutput = Option<Plane<T>>;

    fn bulk_expansion(&self, _rhs: &d4::Quadvector<T>) -> Self::BulkOutput {}

    fn weight_expansion(&self, rhs: &d4::Quadvector<T>) -> Self::WeightOutput {
        if rhs.is_zero() {
            None
        } else if rhs.xyzw.is_sign_positive() {
            Some(*self)
        } else {
            Some(Plane(d4::Trivector {
                wyz: -self.0.wyz,
                wzx: -self.0.wzx,
                wxy: -self.0.wxy,
                zyx: -self.0.zyx,
            }))
        }
    }
}

impl<T> HomogeneusPoint<T>
where
    T: Copy,
    T: ConstZero,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    pub fn line_throught_orthogonal_to(&self, plane: &HomogeneusPlane<T>) -> HomogeneusLine<T> {
        self.weight_expansion(plane)
    }

    pub fn plane_throught_orthogonal_to(&self, line: &HomogeneusLine<T>) -> HomogeneusPlane<T> {
        self.weight_expansion(line)
    }
}

impl<T> d3::Point<T>
where
    T: Copy,
    T: ConstZero,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    pub fn line_throught_orthogonal_to(&self, plane: &Plane<T>) -> Line<T> {
        Line(HomogeneusLine {
            wx: -plane.0.wyz,
            wy: -plane.0.wzx,
            wz: -plane.0.wxy,
            yz: self.0.z * plane.0.wzx - self.0.y * plane.0.wxy,
            zx: self.0.x * plane.0.wxy - self.0.z * plane.0.wyz,
            xy: self.0.y * plane.0.wyz - self.0.x * plane.0.wzx,
        })
    }

    pub fn plane_throught_orthogonal_to(&self, line: &Line<T>) -> Plane<T> {
        Plane(d4::Trivector {
            wyz: -line.0.wx,
            wzx: -line.0.wy,
            wxy: -line.0.wz,
            zyx: self.0.x * line.0.wx + self.0.y * line.0.wy + self.0.z * line.0.wz,
        })
    }
}

impl<T> HomogeneusLine<T>
where
    T: Copy,
    T: ConstZero,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    pub fn plane_throught_orthogonal_to(&self, line: &HomogeneusPlane<T>) -> HomogeneusPlane<T> {
        self.weight_expansion(line)
    }
}

impl<T> Line<T>
where
    T: Copy,
    T: ConstZero,
    T: Float,
    T: Epsilon,
{
    pub fn plane_throught_orthogonal_to(&self, line: &Plane<T>) -> Option<Plane<T>> {
        self.weight_expansion(line)
    }
}
