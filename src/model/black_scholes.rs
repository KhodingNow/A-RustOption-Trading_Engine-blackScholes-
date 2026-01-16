use statrs::distribution::{Normal, ContinuousCDF};
use crate::types::*;

pub fn call_price(
    spot: Spot,
    strike: Strike,
    rate: Rate,
    vol: Volatility,
    time: TimeToMaturity,
) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();

    let s = spot.0;
    let k = strike.0;    
    let r = rate.0;
    let sigma = vol.0;
    let t = time.0;

    let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());

    let d2 = d1 - sigma * t.sqrt();

    s * normal.cdf(d1) - k * (-r * t).exp() * normal.cdf(d2)
    
}
