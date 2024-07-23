use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub struct Offset {
    pub offset: usize,
    pub negative: bool,
}

impl Offset {
    pub fn as_usize(&self) -> usize {
        if self.negative {
            self.offset.wrapping_neg()
        } else {
            self.offset
        }
    }

    pub fn as_isize(&self) -> isize {
        if self.negative {
            self.offset.wrapping_neg() as isize
        } else {
            self.offset as isize
        }
    }

    pub fn is_negative(&self) -> bool {
        self.negative
    }
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.negative {
            write!(f, "-{}", self.offset)
        } else {
            write!(f, "{}", self.offset)
        }
    }
}

impl fmt::LowerHex for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.negative {
            write!(f, "-{:#x}", self.offset)
        } else {
            write!(f, "{:#x}", self.offset)
        }
    }
}

impl fmt::UpperHex for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.negative {
            write!(f, "-{:#X}", self.offset)
        } else {
            write!(f, "{:#X}", self.offset)
        }
    }
}

impl From<usize> for Offset {
    fn from(value: usize) -> Self {
        Self {
            offset: value,
            negative: false,
        }
    }
}

impl From<isize> for Offset {
    fn from(value: isize) -> Self {
        if value < 0 {
            Self {
                offset: value.wrapping_neg() as usize,
                negative: true,
            }
        } else {
            Self {
                offset: value as usize,
                negative: false,
            }
        }
    }
}

impl Add<Offset> for Offset {
    type Output = Offset;
    fn add(self, rhs: Offset) -> Self::Output {
        match (self.negative, rhs.negative) {
            (false, false) | (true, true) => {
                Self {
                    offset: self.offset + rhs.offset,
                    negative: self.negative,
                }
            }
            (false, true) | (true, false) => {
                if self.offset > rhs.offset {
                    Self {
                        offset: self.offset - rhs.offset,
                        negative: self.negative,
                    }
                } else {
                    Self {
                        offset: rhs.offset - self.offset,
                        negative: !self.negative,
                    }
                }
            }
        }
    }
}

impl AddAssign<Offset> for Offset {
    fn add_assign(&mut self, rhs: Offset) {
        *self = *self + rhs;
    }
}

impl Sub<Offset> for Offset {
    type Output = Offset;

    fn sub(self, rhs: Offset) -> Self::Output {
        match (self.negative, rhs.negative) {
            (false, false) | (true, true) => {
                if self.offset >= rhs.offset {
                    Self {
                        offset: self.offset - rhs.offset,
                        negative: self.negative,
                    }
                } else {
                    Self {
                        offset: rhs.offset - self.offset,
                        negative: !self.negative,
                    }
                }
            }
            (false, true) => {
                Self {
                    offset: self.offset + rhs.offset,
                    negative: false,
                }
            }
            (true, false) => {
                Self {
                    offset: self.offset + rhs.offset,
                    negative: true,
                }
            }
        }
    }
}

impl SubAssign<Offset> for Offset {
    fn sub_assign(&mut self, rhs: Offset) {
        *self = *self - rhs;
    }
}

impl PartialOrd for Offset {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if self.negative == other.negative {
            if self.negative {
                other.offset.partial_cmp(&self.offset)
            } else {
                self.offset.partial_cmp(&other.offset)
            }
        } else if self.negative {
            Some(core::cmp::Ordering::Less)
        } else {
            Some(core::cmp::Ordering::Greater)
        }

    }
}

impl Ord for Offset {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Mul<usize> for Offset {
    type Output = Offset;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            offset: self.offset * rhs,
            negative: self.negative,
        }
    }
}

impl Mul<isize> for Offset {
    type Output = Offset;

    fn mul(self, rhs: isize) -> Self::Output {
        if rhs < 0 {
            Self {
                offset: self.offset * rhs.wrapping_neg() as usize,
                negative: !self.negative,
            }
        } else {
            Self {
                offset: self.offset * rhs as usize,
                negative: self.negative,
            }
        }
    }
}

impl Mul<Offset> for Offset {
    type Output = Offset;

    fn mul(self, rhs: Offset) -> Self::Output {
        Self {
            offset: self.offset * rhs.offset,
            negative: self.negative ^ rhs.negative,
        }
    }
}

impl Div<usize> for Offset {
    type Output = Offset;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            offset: self.offset / rhs,
            negative: self.negative,
        }
    }
}

impl Div<isize> for Offset {
    type Output = Offset;

    fn div(self, rhs: isize) -> Self::Output {
        if rhs < 0 {
            Self {
                offset: self.offset / rhs.wrapping_neg() as usize,
                negative: !self.negative,
            }
        } else {
            Self {
                offset: self.offset / rhs as usize,
                negative: self.negative,
            }
        }
    }
}

impl Div<Offset> for Offset {
    type Output = Offset;

    fn div(self, rhs: Offset) -> Self::Output {
        Self {
            offset: self.offset / rhs.offset,
            negative: self.negative ^ rhs.negative,
        }
    }
}