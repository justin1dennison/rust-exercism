
#[derive(Debug)]
pub struct Duration{
  seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64; 
}

macro_rules! planet {
    ($planet:ident, $earth_year_ratio:expr) => {
        pub struct $planet;
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                let seconds = d.seconds;
                (seconds as f64) / ($earth_year_ratio * EARTH_SECONDS_PER_YEAR as f64)
            }
        }

    }
}


const EARTH_SECONDS_PER_YEAR: u64 = 31_557_600;
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);


