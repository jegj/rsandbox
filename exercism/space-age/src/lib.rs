// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
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

impl Mercury {
    const ORBITAL_PERIOD_FROM_EARTH: f64 = 0.2408467_f64;
}

impl Venus {
    const ORBITAL_PERIOD_FROM_EARTH: f64 = 0.61519726_f64;
}

impl Earth {
    const ORBITAL_PERIOD_FROM_EARTH: f64 = 1_f64;
}

impl Mars {
    const ORBITAL_PERIOD_FROM_EARTH: f64 = 1.8808158_f64;
}

impl Jupiter {
    const ORBITAL_PERIOD_FROM_EARTH: f64 = 11.862615_f64;
}

impl Saturn {
    const ORBITAL_PERIOD_FROM_EARTH: f64 = 29.44749_f64;
}

impl Uranus {
    const ORBITAL_PERIOD_FROM_EARTH: f64 = 84.016846_f64;
}

impl Neptune {
    const ORBITAL_PERIOD_FROM_EARTH: f64 = 164.79132_f64;
}

macro_rules! impl_planet {
    (for $($t:ty),+) => {
        $(impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                d.seconds as f64 / 31557600_f64 / <$t>::ORBITAL_PERIOD_FROM_EARTH
            }
        })*
    }
}

impl_planet!(for Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
/*
impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600_f64 / Mercury::ORBITAL_PERIOD_FROM_EARTH
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600_f64 / 0.61519726_f64
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600_f64 / 1_f64
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600_f64 / 1.8808158_f64
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600_f64 / 11.862615_f64
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600_f64 / 29.44749_f64
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600_f64 / 84.016846_f64
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600_f64 / 164.79132_f64
    }
}
*/
