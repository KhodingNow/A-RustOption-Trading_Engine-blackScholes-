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
    fn put_price_respects_intrinsic_value_property(
        spot in 50.0f64..150.0,
        strike in 50.0f64..150.0,
        
    ) {
        let price = put_price(

            Spot(spot),
            Strike(strike),
            Rate(0.01),
            Volatility(0.2),
            TimeToMaturity(1.0),
            
        );

        let intrinsic_value = (strike - spot).max(0.0);
        let epsilon = 1e-8;

        prop_assert!(
            price + epsilon >= intrinsic_value,
            "put price {price} < intrinsic value {intrinsic_value}"
            
         );
        
    }
    
}
