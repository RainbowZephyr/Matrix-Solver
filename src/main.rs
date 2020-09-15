// extern crate arrayfire as af;

// use af::*;
use std::env;
use std::path::PathBuf;
use nalgebra;
use rayon::prelude::*;
use cpuprofiler::PROFILER;
use nalgebra_glm;

#[allow(unused_variables)]
#[allow(unused_must_use)]
fn main() {
    // set_device(1);

    let z = 2573;
    // let d = af::Dim4::new(&[z, 0, 0, 0]);
    // let r: af::Array<u32> = af::range(d, 0);

    let m1 = 2057;
    let m2= 2573;
    let M: nalgebra::Matrix4x1<i32> = nalgebra::Matrix4x1::new(29, 800, 0, 1);
    let m: nalgebra::Matrix3x1<i32> = nalgebra::Matrix3x1::new(m1.clone(), m2.clone(), 1);
    let v: Vec<i32> = (0..z).collect();


    // for a in 0..z {
    //     for b in 0..z {
    //         for c in 0..z {
    //             for d in 0..z {
    //                 for e in 0..z {
    //                     for f in 0..z {
    //                         for g in 0..z {
    //                             for h in 0..z {
    //                                 for i in 0..z {
    //                                     for j in 0..z {
    //                                         for k in 0..z {
    //                                             for l in (0..z).step_by(5) {
    //                                                 let p = nalgebra::Matrix3x4::new(a.clone(), b.clone(), c.clone(), d.clone(),
    //                                                                                  e.clone(), f.clone(), g.clone(), h.clone(),
    //                                                                                  i.clone(), j.clone(), k.clone(), l.clone());
    //
    //                                                 let r = p * M;
    //                                                 if r == m {
    //                                                     println!("{}", p)
    //                                                 }
    //                                             }
    //                                         }
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    let t: nalgebra::Matrix1<i32> = nalgebra::Matrix1::new(m1.clone());
    // println!("{}", t);
    // PROFILER.lock().unwrap().start("./bin.prof").expect("err");


    v.into_par_iter().for_each(|a| {
        // println!("B: {}", b);
        for b in 0..z {
            // for c in 0..z {
                for d in 0..z {
                    let p1: nalgebra::Matrix1x4<i32> = nalgebra::Matrix1x4::new(a.clone(), b.clone() , 0, d.clone());

                    let r: nalgebra::Matrix1<i32> = p1 * M;
                    // println!("result: {}", nalgebra_glm::floor(&r));


                    if r == t {
                        println!("P1: {}", p1);
                    }
                }
            // }
        }
    });

    let u: nalgebra::Matrix1<i32> = nalgebra::Matrix1::new(m2.clone());
    let v: Vec<i32> = (0..z).collect();

    v.into_par_iter().for_each(|a| {
        // println!("B: {}", b);
        for b in 0..z {
            // for c in 0..z {
                for d in 0..z {
                    let p2: nalgebra::Matrix1x4<i32> = nalgebra::Matrix1x4::new(a.clone(), b.clone() , 0, d.clone());

                    let r: nalgebra::Matrix1<i32> = p2 * M;
                    // println!("result: {}", nalgebra_glm::floor(&r));


                    if r == u {
                        println!("P2: {}", p2);
                    }
                }
            // }
        }
    });



    let w: nalgebra::Matrix1<i32> = nalgebra::Matrix1::new(1);

    let v: Vec<i32> = (0..z).collect();

    v.into_par_iter().for_each(|a| {
        // println!("B: {}", b);
        for b in 0..z {
            // for c in 0..z {
                for d in 0..z {
                    let p3: nalgebra::Matrix1x4<i32> = nalgebra::Matrix1x4::new(a.clone(), b.clone() , 0, d.clone());

                    let r: nalgebra::Matrix1<i32> = p3 * M;
                    // println!("result: {}", nalgebra_glm::floor(&r));


                    if r == w {
                        println!("P3: {}", p3);
                    }
                }
            // }
        }
    });
}
