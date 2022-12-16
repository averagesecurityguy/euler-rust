/*
Starting in the top left corner of a 2×2 grid, and only being able to move to
the right and down, there are exactly 6 routes to the bottom right corner.

    R -> D
    D -> R

    R -> R -> D -> D
    R -> D -> R -> D
    R -> D -> D -> R
    D -> R -> R -> D
    D -> R -> D -> R
    D -> D -> R -> R

How many such routes are there through a 20×20 grid? 
*/


fn main() {
    let val = euler::pascal::pascal_kth(40, 20);

    println!("Number of paths {}", val.unwrap());
}
