## Linear Algebra Rust Implementation

## Usage

See test functions in src/lib.rs.

### Examples

```
=> Rust Code
use crate::matrix::{*};
use crate::basic_trait::One;

let n = Matrix::<f64, 3, 3>::one();
println!("{}", n);

=> Result
3x3 Matrix
    1     0     0 
    0     1     0 
    0     0     1 
```


### Example CUDA

```rust
use crate::cublas_ffi::*;
let x = VMatrix::<f32>::new(3, 3, 2f32);
let y = VMatrix::<f32>::new(3, 3, 3f32);
println!("cpu::x: {}", x);
println!("cpu::x: {}", y);
let cx = x.gpu();
let cy = y.gpu();
let cz = cx * cy;
let cpuz = cz.unwrap().cpu();
println!("cpu::z: {}", cpuz);

=> Result
cpu::x: 3x3 Matrix
    2     2     2 
    2     2     2 
    2     2     2 

cpu::x: 3x3 Matrix
    3     3     3 
    3     3     3 
    3     3     3 

cpu::z: 3x3 Matrix
   18    18    18 
   18    18    18 
   18    18    18 

```
