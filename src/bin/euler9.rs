/*
Problem
---
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

    a**2 + b**2 = c**2

For example, 3**2 + 4**2 = 9 + 16 = 25 = 5**2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.

Research
---
https://en.wikipedia.org/wiki/Pythagorean_triple

You can generate a triple using from an arbitrary set of integers m, n, and k,
where m > n > 0, using the formula:

a = k(m^2 - n^2)
b = k(2mn)
c = k(m^2 + n^2)

We want a, b, and c where a + b + c = s. Using substition and simplifying,
we get:

k(m^2 - n^2) + k(2mn) + k(m^2 + n^2) = s
k(m^2 - n^2 + 2mn + m^2 + n^2) = s
k(2m^2 + 2mn) = s
k(2)(m^2 + mn)) = s
k(m^2 + mn) = s/2

So we need to find m, n, and k where m > n > 0 and k(m^2 + mn) == s/2
using k = 1, n = 1, m has an upper bound of floor(sqrt(s)).
using m = 2, n = 1, k has an upper bound of floor(s/12).

*/

fn main() {
    let f: f64 = 1000.0;
    let ubm = f.sqrt().floor() as u64;
    let ubk = (f/12.0).floor() as u64;
    let s = f as u64;

    for m in 2..=ubm {
        for n in 1..m {
            for k in 1..ubk {
                let val = k*(m*m + m*n);
                if val > s/2 { break; }
                if val == s/2 {
                    let a = k*(m*m - n*n);
                    let b = k*2*m*n;
                    let c = k*(m*m + n*n);

                    println!("{} * {} * {} = {}", a, b, c, a*b*c);
                } 
            }
        }
    }
}
