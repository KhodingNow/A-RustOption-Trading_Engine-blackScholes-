use rust_option_engine::model::black_scholes::call_price;
use rust_option_engine::types::*;

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
