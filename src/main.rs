use gual::{Multivector, Multivector2D, Vector2D, WedgeProduct};
use num::zero;

fn main() {
    let v2 = Multivector2D {
        s: zero(),
        v: Vector2D { x: 1, y: 2 },
        a: zero(),
    };

    println!("v2: {:?}", v2);
    println!("volume: {:?}", Multivector2D::<i32>::unit_volume());
    println!("lc(v2): {:?}", v2.left_complement());
    println!("rc(v2): {:?}", v2.right_complement());
    println!("v2 ^ rc(v2): {:?}", v2.wedge(v2.right_complement()));
    println!("lc(v2) ^ v2: {:?}", v2.left_complement().wedge(v2));

    // let m3 = Multivector3D {
    //     s: 0,
    //     x: 1,
    //     y: 2,
    //     z: 3,
    //     yz: 0,
    //     zx: 0,
    //     xy: 0,
    //     xyz: 0,
    // };
    // let n3 = Multivector3D {
    //     s: 0,
    //     x: 4,
    //     y: 5,
    //     z: 6,
    //     yz: 0,
    //     zx: 0,
    //     xy: 0,
    //     xyz: 0,
    // };

    // println!("{:?}", m3 + n3);
    // println!("{:?}", m3.wedge(n3));
}
