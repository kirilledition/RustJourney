// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_IN_SECONDS: f64 = 31_557_600_f64;

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            earth_years: s as f64 / EARTH_YEAR_IN_SECONDS,
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
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

macro_rules! implement_planets {
    ( $($planet:ty, $coefficient:expr),* ) => {
        $(
            impl Planet for $planet {
                fn years_during(d: &Duration) -> f64 {
                    d.earth_years / $coefficient
                }
            }
        )*
    };
}

implement_planets! {
    Mercury, 0.2408467,
    Venus, 0.61519726,
    Earth, 1.0,
    Mars, 1.8808158,
    Jupiter, 11.862615,
    Saturn, 29.447498,
    Uranus, 84.016846,
    Neptune,164.79132
}
