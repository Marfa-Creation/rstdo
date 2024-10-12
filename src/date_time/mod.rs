pub struct DateTime {
    second: i8,
    minute: i8,
    hour: i8,
    day: i8,
    month: i8,
    year: i8,
}

#[allow(dead_code)]
impl DateTime {
    pub fn new() -> DateTime {
        return DateTime {
            second: 0,
            minute: 0,
            hour: 0,
            day: 0,
            month: 0,
            year: 0,
        };
    }

    pub fn get_second(&self) -> i8 {
        return self.second;
    }
    pub fn get_minute(&self) -> i8 {
        return self.minute;
    }
    pub fn get_hour(&self) -> i8 {
        return self.hour;
    }
    pub fn get_day(&self) -> i8 {
        return self.day;
    }
    pub fn get_month(&self) -> i8 {
        return self.month;
    }
    pub fn get_year(&self) -> i8 {
        return self.year;
    }

    pub fn set_second(mut self, value: i8) -> DateTime {
        self.second = value;
        return self;
    }
    pub fn set_minute(mut self, value: i8) -> DateTime {
        self.minute = value;
        return self;
    }
    pub fn set_hour(mut self, value: i8) -> DateTime {
        self.hour = value;
        return self;
    }
    pub fn set_day(mut self, value: i8) -> DateTime {
        self.day = value;
        return self;
    }
    pub fn set_month(mut self, value: i8) -> DateTime {
        self.month = value;
        return self;
    }
    pub fn set_year(mut self, value: i8) -> DateTime {
        self.year = value;
        return self;
    }
}
