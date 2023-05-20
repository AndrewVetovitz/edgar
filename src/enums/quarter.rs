pub enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl Quarter {
    pub fn to_string(&self) -> &'static str {
        match self {
            Quarter::Q1 => "Q1",
            Quarter::Q2 => "Q2",
            Quarter::Q3 => "Q3",
            Quarter::Q4 => "Q4",
        }
    }
}
