#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recurrence {
    pub num: i64,
    pub period: super::Period,
    pub strict: bool,
}

impl ::std::str::FromStr for Recurrence
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()>
    {
        let mut s = s;

        if s.len() < 2 && s.len() > 3 {
            return Err(());
        }

        let strict = if s.get(0..1) == Some("+") {
            s = s.trim_left_matches('+');
            true
        }
        else {
            false
        };

        let num = match s.get(0..1).unwrap().parse() {
            Ok(num) => num,
            Err(_) => return Err(()),
        };

        let period = super::Period::from_str(s.get(1..2).unwrap())?;

        Ok(Self {
            num,
            period,
            strict,
        })
    }
}

impl ::std::fmt::Display for Recurrence
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result
    {
        if self.strict {
            f.write_str("+")?;
        }

        f.write_str(format!("{}{}", self.num, self.period).as_str())?;

        Ok(())
    }
}

impl ::std::ops::Add<::chrono::NaiveDate> for Recurrence
{
    type Output = ::chrono::NaiveDate;

    fn add(self, rhs: Self::Output) -> Self::Output
    {
        let mut result = rhs;

        for _ in 0..self.num {
            result = self.period.clone() + result;
        }

        result
    }
}
