fn ones_to_word(n: usize) -> String {
    match n {
        0 => "".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => "".to_string(),
    }
}

fn teens_to_word(n: usize) -> String {
    match n {
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => "".to_string(),
    }
}

fn tens_to_word(n: usize) -> String {
    match n {
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => "".to_string(),
    }
}

pub fn number_to_words(mut n: usize) -> String {
    let mut words = Vec::<String>::new();

    if n < 1 || n >= 10000 {
        return words.join("");
    }

    let thousands = n / 1000;

    if thousands > 0 {
        words.push(format!("{} thousand ", ones_to_word(thousands)));
    };

    n = n % 1000;
    let hundreds = n / 100;

    if hundreds > 0 {
        match n % 100 {
            0 => words.push(format!("{} hundred", ones_to_word(hundreds))),
            _ => words.push(format!("{} hundred and ", ones_to_word(hundreds))),
        }
    };

    n = n % 100;
    let tens = n / 10;
    let ones = n % 10;

    match tens {
        1 => words.push(format!("{}", teens_to_word(tens * 10 + ones))),
        2..=9 => {
            match ones {
                1..=9 => words.push(format!("{}-{}", tens_to_word(tens), ones_to_word(ones))),
                _ => words.push(format!("{}", tens_to_word(tens))),
            }
        },
        _ => words.push(format!("{}", ones_to_word(ones))),
    };

    words.join("")
}

pub fn letter_count(s: String) -> usize {
    let mut sum = 0;

    for (_, c) in s.to_lowercase().chars().enumerate() {
        match c {
            'a'..='z' => sum += 1,
            _ => sum += 0,
        }        
    }

    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_words() {
        assert_eq!(number_to_words(1), "one".to_string());
        assert_eq!(number_to_words(2), "two".to_string());
        assert_eq!(number_to_words(3), "three".to_string());
        assert_eq!(number_to_words(4), "four".to_string());
        assert_eq!(number_to_words(5), "five".to_string());
        assert_eq!(number_to_words(6), "six".to_string());
        assert_eq!(number_to_words(7), "seven".to_string());
        assert_eq!(number_to_words(8), "eight".to_string());
        assert_eq!(number_to_words(9), "nine".to_string());
        assert_eq!(number_to_words(10), "ten".to_string());
        assert_eq!(number_to_words(11), "eleven".to_string());
        assert_eq!(number_to_words(12), "twelve".to_string());
        assert_eq!(number_to_words(13), "thirteen".to_string());
        assert_eq!(number_to_words(14), "fourteen".to_string());
        assert_eq!(number_to_words(15), "fifteen".to_string());
        assert_eq!(number_to_words(16), "sixteen".to_string());
        assert_eq!(number_to_words(17), "seventeen".to_string());
        assert_eq!(number_to_words(18), "eighteen".to_string());
        assert_eq!(number_to_words(19), "nineteen".to_string());
        assert_eq!(number_to_words(20), "twenty".to_string());
        assert_eq!(number_to_words(21), "twenty-one".to_string());
        assert_eq!(number_to_words(22), "twenty-two".to_string());
        assert_eq!(number_to_words(23), "twenty-three".to_string());
        assert_eq!(number_to_words(24), "twenty-four".to_string());
        assert_eq!(number_to_words(25), "twenty-five".to_string());
        assert_eq!(number_to_words(26), "twenty-six".to_string());
        assert_eq!(number_to_words(27), "twenty-seven".to_string());
        assert_eq!(number_to_words(28), "twenty-eight".to_string());
        assert_eq!(number_to_words(29), "twenty-nine".to_string());
        assert_eq!(number_to_words(40), "forty".to_string());
        assert_eq!(number_to_words(100), "one hundred".to_string());
        assert_eq!(number_to_words(101), "one hundred and one".to_string());
        assert_eq!(number_to_words(129), "one hundred and twenty-nine".to_string());
        assert_eq!(number_to_words(1234), "one thousand two hundred and thirty-four".to_string());
    }

    #[test]
    fn test_letter_count() {
        assert_eq!(letter_count("letters".to_string()), 7);
        assert_eq!(letter_count("LetterS".to_string()), 7);
        assert_eq!(letter_count("let-terS".to_string()), 7);
        assert_eq!(letter_count("!@#$%^&".to_string()), 0);
        assert_eq!(letter_count("".to_string()), 0);
        assert_eq!(letter_count("three hundred and forty-two".to_string()), 23);
        assert_eq!(letter_count("one hundred and fifteen".to_string()), 20);
    }
}
