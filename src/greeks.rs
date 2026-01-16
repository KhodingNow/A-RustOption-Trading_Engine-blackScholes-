use statrs::distribution::{Normal, ContinuousCDF};
use crate::types::*;

pub fn delta_call(
    spot: Spot,
    strike: Strike,
    rate: Rate,
    vol: Volatility,
    time: TimeToMaturity,
    
) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();

    let d1 = (
        (spot.0 / strike.0).ln() + (rate.0 + 0.5 * vol.0 * vol.0) * time.0) / (vol.0 * time.0.sqrt());

     normal.cdf(d1)
}
