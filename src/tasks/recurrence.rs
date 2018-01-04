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

impl ::std::convert::Into<::chrono::Duration> for Recurrence
{
    fn into(self) -> ::chrono::Duration
    {
        use super::Period::*;

        match self.period {
            Day => ::chrono::Duration::days(self.num),
            Week => ::chrono::Duration::weeks(self.num),
            Month => ::chrono::Duration::weeks(self.num * 4),
            Year => ::chrono::Duration::weeks(self.num * 52),
        }
    }
}
