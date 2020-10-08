use euler;

/*
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

fn main() {
    let composite = 600851475143;
    let factors = euler::factor(composite);
    let largest = factors[factors.len()-1];

    println!("The max prime factor is {}", largest);
}
