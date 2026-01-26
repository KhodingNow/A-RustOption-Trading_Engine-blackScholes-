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

pub fn put_price(
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

    let d1 = (( s / k).ln() + (r + 0.5 * sigma * sigma) *t) / (sigma * t.sqrt());

    let d2 = d1 - sigma * t.sqrt();

    k * (-r * t).exp() * normal.cdf(-d2) - s * normal.cdf(-d1)


    // canonical Black-Scholes formula: P = Ke*(-rt) N(-d2) - SN(-d1) 
}


#[cfg(test)]

mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn call_price_ispositive() {

        let price = call_price(
            Spot(100.0),
            Strike(100.0),
            Rate(0.01),
            Volatility(0.02),
            TimeToMaturity(1.0),           
            
        );
        
        assert!(price > 0.0);
         
    }
    
}

#[test]

fn zero_time_to_maturity() {
    let price = call_price(
        Spot(100.0),
        Strike(100.0),
        Rate(0.05),
        Volatility(0.2),
        TimeToMaturity(0.0001),
    );

    assert!(price >= 0.0);
    
}


#[test]
fn call_price_is_never_below_intrinsic_value() {
    let spot = Spot(120.0);
    let strike = Strike(100.0);
    
    let price = call_price(
        spot,
        strike,
        Rate(0.01),
        Volatility(0.2),
        TimeToMaturity(1.0),
        
    );
    
    let intrinsic_value = (strike.0 - spot.0).max(0.0);
    
    assert!(
        price >= intrinsic_value,

        "price {price} < intrinsic value {intrinsic_value }"


); 
       
   

     
}
