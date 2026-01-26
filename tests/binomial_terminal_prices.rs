use rust_option_engine::model::binomial::terminal_prices;
use rust_option_engine::types::*;

#[test]
fn crr_terminal_prices_are_correct_for_two_step() {
    let spot = Spot(100.0);
    let steps = 2;

    let u = 1.1;
    let d = 0.9;

    let prices = terminal_prices(spot, u, d, steps);

    // N steps => N + 1 terminal nodes
    assert_eq!(prices.len(), 3);

    // Expected:
    // k=2: 100 * u^2
    // k=1: 100 * u * d
    // k=0: 100 * d^2

    let expected = vec![
        100.0 * u * u,
        100.0 * u * d,
        100.0 * d * d,
        
    ];

    for (p, e) in prices.iter().zip(expected.iter()) {
        assert!(

            (p-e).abs() < 1e-10,
            
            "price {p} != expected {e}"
        );
    }
}
