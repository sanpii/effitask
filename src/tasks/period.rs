#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Period {
    Day,
    Week,
    Month,
    Year,
}

impl ::std::str::FromStr for Period
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()>
    {
        use self::Period::*;

        match s {
            "d" => Ok(Day),
            "w" => Ok(Week),
            "m" => Ok(Month),
            "y" => Ok(Year),
            _ => Err(()),
        }
    }
}

impl ::std::fmt::Display for Period
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result
    {
        use self::Period::*;

        let s = match *self {
            Day => "d",
            Week => "w",
            Month => "m",
            Year => "y",
        };

        f.write_str(s)?;

        Ok(())
    }
}

impl ::std::convert::Into<::chrono::Duration> for Period
{
    fn into(self) -> ::chrono::Duration
    {
        use super::Period::*;

        match self {
            Day => ::chrono::Duration::days(1),
            Week => ::chrono::Duration::weeks(1),
            Month => ::chrono::Duration::weeks(4),
            Year => ::chrono::Duration::weeks(52),
        }
    }
}
