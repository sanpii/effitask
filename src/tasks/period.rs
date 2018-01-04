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
