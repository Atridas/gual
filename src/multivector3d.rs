// use std::ops::{Add, Mul, Sub};

// use crate::Multivector;

// #[derive(Debug, Clone, Copy)]
// pub struct Multivector3D<T> {
//     pub s: T,
//     pub x: T,
//     pub y: T,
//     pub z: T,
//     pub yz: T,
//     pub zx: T,
//     pub xy: T,
//     pub xyz: T,
// }

// impl<T: Copy> Multivector for Multivector3D<T>
// where
//     T: Add<T, Output = T>,
//     T: Sub<T, Output = T>,
//     T: Mul<T, Output = T>,
// {
//     type Scalar = T;

//     fn scalar(&self) -> Self::Scalar {
//         self.s
//     }

//     fn antiscalar(&self) -> Self::Scalar {
//         self.xyz
//     }

//     fn right_complement(&self) -> Self {
//         unimplemented!();
//     }

//     fn left_complement(&self) -> Self {
//         unimplemented!();
//     }

//     fn wedge(self, rhs: Self) -> Self {
//         Multivector3D {
//             s: self.s * rhs.s,
//             x: self.s * rhs.x + self.x * rhs.s,
//             y: self.s * rhs.y + self.y * rhs.s,
//             z: self.s * rhs.z + self.z * rhs.s,
//             yz: self.y * rhs.z - self.z * rhs.y,
//             zx: self.z * rhs.x - self.x * rhs.z,
//             xy: self.x * rhs.y - self.y * rhs.x,
//             xyz: self.s * rhs.xyz
//                 + self.xyz * rhs.s
//                 + self.x * rhs.yz
//                 + self.yz * rhs.x
//                 + self.y * rhs.zx
//                 + self.zx * rhs.y
//                 + self.z * rhs.xy
//                 + self.xy * rhs.z,
//         }
//     }
// }

// impl<T: Copy> Add for Multivector3D<T>
// where
//     T: Add<T, Output = T>,
// {
//     type Output = Multivector3D<T>;
//     fn add(self, rhs: Self) -> Self::Output {
//         Multivector3D {
//             s: self.s + rhs.s,
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z,
//             yz: self.yz + rhs.yz,
//             zx: self.zx + rhs.zx,
//             xy: self.xy + rhs.xy,
//             xyz: self.xyz + rhs.xyz,
//         }
//     }
// }

// impl<T> Sub for Multivector3D<T>
// where
//     T: Sub<T, Output = T>,
// {
//     type Output = Multivector3D<T>;
//     fn sub(self, rhs: Self) -> Self::Output {
//         Multivector3D {
//             s: self.s - rhs.s,
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//             z: self.z - rhs.z,
//             yz: self.yz - rhs.yz,
//             zx: self.zx - rhs.zx,
//             xy: self.xy - rhs.xy,
//             xyz: self.xyz - rhs.xyz,
//         }
//     }
// }
