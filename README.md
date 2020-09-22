# Rust Matrix Multiplication

Matrix multiplication implemented in Rust

```rust
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<i32>,
}

// fn main() {

let a = Matrix::new(2, 3, (0..6).collect()).unwrap();
let b = Matrix::new(3, 2, (0..6).collect()).unwrap();
assert_eq!(
    a * b,
    Matrix {
        rows: 2,
        cols: 2,
        data: [10, 13, 28, 40].to_vec(),
    }
);

// }
```

## Naïve, true to theory, traversal of a [tree of pythagorean triples](https://en.wikipedia.org/wiki/Tree_of_primitive_Pythagorean_triples)


* A Pythagorean triplet is a set of three natural numbers, {a, b, c}, for
which,

```text
a**2 + b**2 = c**2
```

and such that,

```text
a < b < c
```

For example,

```text
3**2 + 4**2 = 9 + 16 = 25 = 5**2.
```

There exists exactly one Pythagorean triplet for which a + b + c = 1000.

Finds the product a * b * c.

* Includes [`TernaryTree`](https://github.com/mkatychev/pythagorean_triplet/blob/master/src/ternary_tree.rs#L7-L12) implementation
