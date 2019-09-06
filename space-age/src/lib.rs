
const EARTH_SECONDS_PER_YEAR: u64 = 31_557_600;

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
    const EARTH_YEAR_RATIO: f64;
    fn years_during(d: &Duration) -> f64 {
        let seconds = d.seconds;
        (seconds as f64) / (Self::EARTH_YEAR_RATIO  * EARTH_SECONDS_PER_YEAR as f64)
    }
}

macro_rules! planet {
    ($planet:ident, $earth_year_ratio:expr) => {
        impl Planet for $planet {
            const EARTH_YEAR_RATIO: f64 = $earth_year_ratio;
        }

    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);


