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
