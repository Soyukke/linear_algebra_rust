pub trait One {
    fn one() -> Self;
}

impl One for usize {
    fn one() -> Self {
        1
    }
}

impl One for i32 {
    fn one() -> Self{
        1
    }
}

impl One for i64{
    fn one() -> Self {
        1
    }
}


impl One for f32 {
    fn one() -> Self {
        1_f32
    }
}

impl One for f64 {
    fn one() -> Self {
        1_f64
    }
}

//pub trait Rand {
//   fn rand() -> Self;
//}
//
//impl Rand for usize {
//    fn rand() -> Self {
//        let mut rng = rand::thread_rng();
//        rng.gen()
//    }
//}
//
//impl Rand for i32 {
//    fn rand() -> Self{
//        let mut rng = rand::thread_rng();
//        rng.gen()
//    }
//}
//
//impl Rand for i64{
//    fn rand() -> Self {
//        let mut rng = rand::thread_rng();
//        rng.gen()
//    }
//}
//
//
//impl Rand for f32 {
//    fn rand() -> Self {
//        let mut rng = rand::thread_rng();
//        rng.gen()
//    }
//}
//
//impl Rand for f64 {
//    fn rand() -> Self {
//        let mut rng = rand::thread_rng();
//        rng.gen()
//    }
//}
//
//



