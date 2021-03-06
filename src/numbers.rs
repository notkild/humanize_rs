static DIGITS: [&'static str; 10] = ["", "one", "two", "three", "four", "five", "six", "seven",
                                     "eight", "nine"];

static TO_20: [&'static str; 10] = ["ten",
                                    "eleven",
                                    "twelve",
                                    "thirteen",
                                    "fourteen",
                                    "fifteen",
                                    "sixteen",
                                    "seventeen",
                                    "eighteen",
                                    "nineteen"];

static TENS: [&'static str; 10] = ["", "ten", "twenty", "thirty", "fourty", "fifty", "sixty",
                                   "seventy", "eighty", "ninety"];

static SCALE: [&'static str; 9] = ["",
                                   " thousand ",
                                   " million ",
                                   " billion ",
                                   " trillion ",
                                   " quadrillion ",
                                   " quintillion ",
                                   " hexillion ",
                                   " heptillion "];

pub trait HumanizeNumbers {
    fn ord(&self) -> String;
    fn to_text(&self) -> String;
    fn intcomma(&self) -> String;
    fn times(&self) -> String;
}

fn stringify(res: &mut String, chunk: Vec<usize>) {
    match chunk.len() {
        3 => {
            match chunk[1] {
                1 => {
                    *res += DIGITS[chunk[0] as usize];
                    *res += " hundred and ";
                    *res += TO_20[chunk[2] as usize];
                }
                0 => {
                    *res += DIGITS[chunk[0] as usize];
                    *res += " hundred";
                    if chunk[2] != 0 {
                        *res += " and ";
                        *res += DIGITS[chunk[2] as usize]
                    }
                }
                _ => {
                    if chunk[0] != 0 {
                        *res += DIGITS[chunk[0] as usize];
                        *res += " hundred and ";
                    }
                    *res += TENS[chunk[1] as usize];
                    if chunk[2] != 0 {
                        *res += "-";
                        *res += DIGITS[chunk[2] as usize]
                    }
                }
            }
        }
        2 => {
            match chunk[0] {
                1 => *res += TO_20[chunk[1] as usize],
                _ => {
                    *res += TENS[chunk[0] as usize];
                    if chunk[1] != 0 {
                        *res += "-";
                        *res += DIGITS[chunk[1] as usize]
                    }
                }
            }
        }
        1 => {
            *res += DIGITS[chunk[0] as usize];
        }
        _ => unreachable!{},
    }
}

macro_rules! impl_humanize_numbers_u {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {
            fn ord(&self) -> String {
                let suffix = match self % 10 {
                    1 => "st",
                    2 => "nd",
                    3 => "rd",
                    _ => "th",
                };
                format!("{}{}", self, suffix)
            }

            fn to_text(&self) -> String {
                if *self == 0 {
                    return "zero".to_string();
                }
                let mut num = *self;
                let mut split_digits = Vec::new();

                while num > 0 {
                    split_digits.insert(0, (num % 10) as usize);
                    num /= 10
                }

                let (first, remainder) = split_digits.split_at(split_digits.len() % 3);
                let chunks = first.chunks(3).chain(remainder.chunks(3)).map(|x| x.to_vec()).collect::<Vec<_>>();

                let mut res = String::new();
                let mut scale_idx = chunks.len();

                for c in chunks {
                    stringify(&mut res, c);
                    scale_idx -= 1;
                    res += SCALE[scale_idx];
                }

                res
            }

            fn intcomma(&self) -> String {
                let mut s  = format!("{}", self);
                if s.len() <= 3 {
                    return s;
                }

                let mut insert_idx = if s.len() % 3 == 0 {
                    3
                } else {
                    s.len() % 3
                };

                while insert_idx < s.len() {
                    s.insert(insert_idx, ',');
                    insert_idx += 4;
                }

                s
            }

            fn times(&self) -> String {
                match *self {
                    0 => "never".to_string(),
                    1 => "once".to_string(),
                    2 => "twice".to_string(),
                    n => format!("{} times", n.to_text()),
                }
            }
        }
    )*)
}

macro_rules! impl_humanize_numbers_i {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {
            fn ord(&self) -> String {
                if *self < 0 {
                    "-".to_string() + &(self.abs() as u64).ord()
                } else {
                    (*self as u64).ord()
                }
            }

            fn to_text(&self) -> String {
                if *self < 0 as $t {
                    format!("minus {}", (self.abs() as u64).to_text())
                }
                else {
                    (self.abs() as u64).to_text()
                }
            }

            fn intcomma(&self) -> String {
                if *self < 0 {
                    "-".to_string() + &(self.abs() as u64).intcomma()
                } else {
                    (*self as u64).intcomma()
                }
            }

            fn times(&self) -> String {
                match *self {
                    -1 => "minus one time".to_string(),
                    0 => "never".to_string(),
                    1 => "once".to_string(),
                    2 => "twice".to_string(),
                    n => format!("{} times", n.to_text()),
                }
            }
        }
    )*)
}

impl_humanize_numbers_i!(HumanizeNumbers for isize i8 i16 i32 i64);
impl_humanize_numbers_u!(HumanizeNumbers for usize u8 u16 u32 u64);

#[test]
fn test_ordinals() {
    assert_eq!(101.ord(), "101st");
    assert_eq!(2.ord(), "2nd");
    assert_eq!(10093.ord(), "10093rd");
    assert_eq!((-159652).ord(), "-159652nd");
    assert_eq!(0.ord(), "0th");
}

#[test]
fn test_int_separators() {
    assert_eq!(0.intcomma(), "0");
    assert_eq!(12.intcomma(), "12");
    assert_eq!(123.intcomma(), "123");
    assert_eq!(1234.intcomma(), "1,234");
    assert_eq!(12345.intcomma(), "12,345");
    assert_eq!(123456.intcomma(), "123,456");
    assert_eq!(1234567.intcomma(), "1,234,567");
}

#[test]
fn test_int_separators_negative() {
    assert_eq!((-1).intcomma(), "-1");
    assert_eq!((-12).intcomma(), "-12");
    assert_eq!((-123).intcomma(), "-123");
    assert_eq!((-1234).intcomma(), "-1,234");
    assert_eq!((-12345).intcomma(), "-12,345");
    assert_eq!((-123456).intcomma(), "-123,456");
    assert_eq!((-1234567).intcomma(), "-1,234,567");
}

#[test]
fn test_times() {
    assert_eq!(0.times(), "never");
    assert_eq!(1.times(), "once");
    assert_eq!(2.times(), "twice");
    assert_eq!(3.times(), "three times");
    assert_eq!(10.times(), "ten times");
    assert_eq!((-1).times(), "minus one time");
    assert_eq!((-2).times(), "minus two times");
}
