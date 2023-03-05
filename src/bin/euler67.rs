/*
By starting at the top of the triangle below and moving to adjacent numbers on
the row below, the maximum total from top to bottom is 23.

   3
  7 4
 2 4 6
8 5 9 3

That is, 3 + 7 + 4 + 9 = 23.

Find the maximum total from top to bottom of the triangle below:

                            75
                          95  64
                        17  47  82
                      18  35  87  10
                    20  04  82  47  65
                  19  01  23  75  03  34
                88  02  77  73  07  63  67
              99  65  04  28  06  16  70  92
            41  41  26  56  83  40  80  70  33
          41  48  72  33  47  32  37  16  94  29
        53  71  44  65  25  43  91  52  97  51  14
      70  11  33  28  77  73  17  78  39  68  17  57
    91  71  52  38  17  14  91  43  58  50  27  29  48
  63  66  04  68  89  53  67  30  73  16  69  87  40  31
04  62  98  27  23  09  70  98  73  93  38  53  60  04  23

NOTE: As there are only 16384 routes, it is possible to solve this problem by
trying every route. However, Problem 67, is the same challenge with a triangle
containing one-hundred rows; it cannot be solved by brute force, and requires
a clever method! ;o)

Starting on the next to last row and working toward the first row, loop
through each element in the row and add to it the higher of the left or right
value in the row below. The final value at the top of the triangle will be
highest sum. The example triangle,

   3
  7 4
 2 4 6
8 5 9 3

would become

      23
    20  19
  10  13  15
08  05  09  03
*/

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn load_triangle_from_file(fname: &str) -> Vec<Vec<usize>> {
    let mut tri = Vec::new();
    let f = File::open(fname).unwrap();
    let file = BufReader::new(&f);

    for (_, line) in file.lines().enumerate() {
        let mut nums = Vec::new();

        for num in line.unwrap().split(" ") {
            nums.push(num.parse::<usize>().unwrap());
        }

        tri.push(nums);
    }

    tri
}

fn main() {
    let mut tri = load_triangle_from_file("data/euler67.txt");

    for i in (0..(tri.len() - 1)).rev() {
        for j in 0..tri[i].len() {
            let left = tri[i+1][j];
            let right = tri[i+1][j+1];

            if left > right {
                tri[i][j] += left
            } else {
                tri[i][j] += right
            }
        }
    }

    println!("Sum: {}", tri[0][0]);
}
