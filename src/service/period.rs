pub trait IPeriod {
    fn get_code(&self) -> i32;
    fn year(&self) -> i16;
    fn month(&self) -> i16;
    fn is_period_greater_or_equal_than(&self, year_from: i16, month_from: i16) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct Period {
    code: i32
}

impl IPeriod for Period {
    fn get_code(&self) -> i32 {
        self.code
    }
    fn year(&self) -> i16 {
        (self.code/100) as i16
    }
    fn month(&self) -> i16 {
        (self.code%100) as i16
    }
    fn is_period_greater_or_equal_than(&self, year_from: i16, month_from: i16) -> bool {
        self.year() > year_from || (self.year() == year_from && self.month() >= month_from)
    }
}

#[allow(dead_code)]
impl Period {
    pub fn new() -> Period {
        Period { code: 0 }
    }
    pub fn get(_code: i32) -> Period {
        Period { code: _code }
    }
    pub fn get_with_year_month(_year: i16, _month: i16) -> Period {
        Period { code: (i32::from(_year) * 100 + i32::from(_month)) }
    }
}