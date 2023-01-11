use std::fmt;

/// Units of digital information expressed in base-10
#[derive(Clone, Copy, Debug)]
pub enum Units {
    B = 1,
    KB = 1_000,
    MB = 1_000_000,
    GB = 1_000_000_000,
    TB = 1_000_000_000_000,
    PB = 1_000_000_000_000_000,
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// takes a [u64] (representative of bytes in a file) and converts to a human readable [String]
/// ```
/// use treewalk::walk::format;
/// assert_eq!(format::human_readable(1_000, false), "1000B");
/// assert_eq!(format::human_readable(10_500, true), "10.50KB");
/// assert_eq!(format::human_readable(10_000_000, false), "10MB");
/// assert_eq!(format::human_readable(100_000_000, false), "100MB");
/// assert_eq!(format::human_readable(1_000_000_000, false), "1000MB");
/// assert_eq!(format::human_readable(1_000_000_001, false), "1GB");
/// assert_eq!(format::human_readable(1_000_000_000_000, false), "1000GB");
/// assert_eq!(format::human_readable(10_000_000_000_000, false), "10TB");
/// ```
pub fn human_readable(num: u64, as_float: bool) -> String {
    let result = String::from("");
    match num {
        ..=1_000 => result + &num.to_string() + "B",
        1_001..=1_000_000 => construct_hr_output(&num, as_float, Units::KB),
        1_000_001..=1_000_000_000 => construct_hr_output(&num, as_float, Units::MB),
        1_000_000_001..=1_000_000_000_000 => construct_hr_output(&num, as_float, Units::GB),
        1_000_000_000_001..=1_000_000_000_000_000 => construct_hr_output(&num, as_float, Units::TB),
        1_000_000_000_000_001.. => construct_hr_output(&num, as_float, Units::PB),
    }
}

fn construct_hr_output(num: &u64, as_float: bool, unit: Units) -> String {
    if as_float {
        format!("{:.2}{}", *num as f64 / (unit as u64) as f64, unit)
    }
    else {
        format!("{:.2}{}", num / unit as u64, unit)
    }
}
