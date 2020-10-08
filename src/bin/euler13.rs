use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

/*
Problem
---
Work out the first ten digits of the sum of the one-hundred 50-digit numbers
in the file data/euler13.txt.

Research
---
It is only necessary to add up the first 12 or so digits of each of these
numbers because the lower order values in each number have no bearing on the
final answer. For example:

  1230        1239
  2560        2567
  3870        3878
+ 9430      + 9439
------      ------
 17090       17323

 In this case, to get the first 2 digits of the sum of these numbers it is
 only necessary to add the first 3 digits of each number. 
*/

fn main() {
    let f = File::open("data/euler13.txt").unwrap();
    let file = BufReader::new(&f);
    let mut sum: u64 = 0;

    for (_, line) in file.lines().enumerate() {
        let n_str = line.unwrap();
        let n = n_str[..14].parse::<u64>().unwrap();

        sum += n;
    }

    println!("The first 10 digits are {}", &sum.to_string()[..10]);
}
