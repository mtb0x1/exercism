// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

#[derive(Clone,Copy)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Comma
}


impl std::fmt::Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let digit_char = match self {
            Digit::Zero => '0',
            Digit::One => '1',
            Digit::Two => '2',
            Digit::Three => '3',
            Digit::Four => '4',
            Digit::Five => '5',
            Digit::Six => '6',
            Digit::Seven => '7',
            Digit::Eight => '8',
            Digit::Nine => '9',
            Digit::Comma => ','
        };
        write!(f, "{}", digit_char)
    }
}

struct DigitPattern {
    digit: Digit,
    pattern: Vec<&'static str>,
}

impl DigitPattern {
    fn new(digit: Digit, pattern: Vec<&'static str>) -> Self {
        DigitPattern { digit, pattern }
    }
}
#[inline(always)]
fn digit_patterns() -> Vec<DigitPattern> {
    vec![
        DigitPattern::new(Digit::Zero, vec![" _ ", "| |", "|_|","   "]),
        DigitPattern::new(Digit::One, vec!["   ", "  |", "  |","   "]),
        DigitPattern::new(Digit::Two, vec![" _ ", " _|", "|_ ","   "]),
        DigitPattern::new(Digit::Three, vec![" _ ", " _|", " _|","   "]),
        DigitPattern::new(Digit::Four, vec!["   ", "|_|", "  |","   "]),
        DigitPattern::new(Digit::Five, vec![" _ ", "|_ ", " _|","   "]),
        DigitPattern::new(Digit::Six, vec![" _ ", "|_ ", "|_|","   "]),
        DigitPattern::new(Digit::Seven, vec![" _ ", "  |", "  |","   "]),
        DigitPattern::new(Digit::Eight, vec![" _ ", "|_|", "|_|","   "]),
        DigitPattern::new(Digit::Nine, vec![" _ ", "|_|", " _|","   "])
    ]
}

fn recognize_digit(lines: &[&str]) -> Option<Digit> {
    for pattern in digit_patterns().iter() {
        if lines == pattern.pattern {
            return Some(pattern.digit);
        }
    }
    None
}

fn recognize_digits(lines: Vec<&str>) -> Vec<Option<Digit>> {
    let mut digits: Vec<Option<Digit>> = Vec::new();
    let chunk_size = 4;
    let num_chunks = (lines.len() + chunk_size - 1) / chunk_size;

    for (i,col) in lines.chunks(chunk_size).enumerate() {
        for start_col in (0..col[0].len()).step_by(3) {
            let end_col = start_col + 3;
            let digit_lines: Vec<&str> = col.iter().map(|line| &line[start_col..end_col]).collect();
            digits.push(recognize_digit(&digit_lines));
            
        }

        //if the last line that is empty
        //and the last chunk is not the last chunk
        //add a comma 
        if col[col.len()-1].trim().is_empty() && i!=num_chunks-1 {
            digits.push(Some(Digit::Comma));
        }
    }

    digits
}
 
pub fn convert(input: &str) -> Result<String, Error> {
    println!("Convert the input '{input}' to a string");
    let lines = input.lines().count();
    if lines % 4 != 0 {
        return Err(Error::InvalidRowCount(lines));
    }
    let _x = input
        .lines()
        .map(|l| {
            let char_count = l.chars().filter(|c| *c != '\n').count();
            if char_count % 3 != 0 {
                return char_count;
            }
            0
        })
        .collect::<Vec<_>>();
    if _x[0]>0 {
        return Err(Error::InvalidColumnCount(_x[0])); 
    }

    let lines: Vec<&str> = input.lines().collect();
    let recognized_digits = recognize_digits(lines);

    let mut result = String::new(); 

    for some_digit in recognized_digits {
        if let Some(digit) = some_digit  {
            result.push_str(&digit.to_string());
        }else {
            result.push('?');
        }
    }
    Ok(result)
}
