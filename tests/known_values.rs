use rust_option_engine::model::black_scholes::{call_price, put_price};
use rust_option_engine::types::*;
use proptest::prelude::*;


/*

// test Before testint epsilo

#[test]
fn black_scholes_call_known_value() {
    let price = call_price(
        Spot(100.0),
        Strike(100.0),
        Rate(0.05),
        Volatility(0.2),
        TimeToMaturity(1.0),
    );

    assert!(
    (price - 10.45).abs() < 0.05,
   
    "price was {price}"

    );
}

*/


// I use epsilon tolerances in pricing tests to distinguish floating-point noise from genuine violations of financial invarients !

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

    let intrinsic_value = (spot.0 - strike.0).max(0.0);
    let epsilon = 1e-8;

    assert!(
        price + epsilon >=intrinsic_value,

        "price {price} < intrinsic value {intrinsic_value}"
    );
}

// property test - dont randomise everything - only S and K.

// I use property-based tests to enforce economic invariaents in option pricing, validating behaviour across a wide range of inputs rather than fixed examples. 

proptest! {
    #[test]
    fn call_price_respects_intrinsic_value_property(
        spot in 50.0f64..150.0,
        strike in 50.0f64..150.0,
        
    ) {

        let price = call_price(

            Spot(spot),
            Strike(strike),
            Rate(0.01),
            Volatility(0.2),
            TimeToMaturity(1.0),
            
        );
        
        let intrinsic_value = (spot - strike).max(0.0);
        let epsilon = 1e-8;
         
        prop_assert!(

            price + epsilon >= intrinsic_value,
            " price {price} < intrinsic value {intrinsic_value}"
            
        );
    }
}

proptest! {

    #[test]
    fn european_put_respects_discounted_lower_bound(
    
        spot in 50.0f64..150.0,
        strike in 50.0f64..150.0,
        
    ) {

        let r = Rate(0.05);
        let vol = Volatility(0.02);
        let t = TimeToMaturity(1.0); 
    
        let price = put_price(

            Spot(spot),
            Strike(strike),
            r,
            vol,
            t,
            
        );

        let discounted_strike = strike * (-r.0 * t.0).exp();
        let lower_bound = (discounted_strike - spot).max(0.0);
        
        let epsilon = 1e-8;

        prop_assert!(
            price + epsilon >= lower_bound,
            "put price {price} < European lower bound {lower_bound}"
            
         );
        
    }
    
}

proptest! {
    #[test]
    fn put_call_parity_holds(
        spot in 50.0f64..150.0,
        strike in 50.0f64..150.0,
        
    ) {

        let r = Rate(0.05);
        let vol = Volatility(0.2);
        let t = TimeToMaturity(1.0);

        let call = call_price(
            Spot(spot),
            Strike(strike),
            r,
            vol,
            t,
        );

        let put = put_price(
            Spot(spot),
            Strike(strike),
            r,
            vol,
            t,
        );

        let lhs = call - put;
        let rhs = spot - strike * (-r.0 * t.0).exp();

        let epsilon = 1e-8;

        prop_assert!(
            (lhs - rhs).abs() < epsilon,

            "put-call parity violated: C-P={lhs}, S-Ke^(-rt)={rhs}"
            
        );
    }
     
 }

//Delta central - difference test

proptest! {
    #[test]
    fn call_delta_is_between_zero_and_one (
        spot in 60.0f64..140.0, 
        strike in 50.0f64..150.0,
        
    ) {

        let r = Rate(0.50);
        let vol = Volatility(0.2);
        let t = TimeToMaturity(1.0);


        let epsilon = 1e-4; // finite-differnce step

        let price_up = call_price(
            Spot(spot + epsilon),
            Strike(strike),
            r,
            vol,
            t,              
            
        );

        let price_down = call_price(
            Spot(spot - epsilon),
            Strike(strike),
            r, 
            vol,
            t,
        );   

        let delta = (price_up - price_down) / (2.0 * epsilon);

        prop_assert!(
            delta >= -1e-6 && delta <= 1.0 + 1e-6,

            "call delta out of bounds: delta={delta}"
        );
    }
}

 
