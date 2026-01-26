use rust_option_engine::model::binomial::risk_neutral_params;


#[test]
fn crr_probabilities_are_valid() {
    let (u, d, p) = risk_neutral_params(0.05, 0.2, 0.01);
    assert!(u > d);
    assert!(p > 0.0 && p < 1.0);
}

/// Cox–Ross–Rubinstein parameters must satisfy
/// u > 1, d < 1, and 0 < p < 1
#[test]
fn crr_parameters_are_arbitrage_free() {
    let rate = 0.05;
    let vol = 0.2;
    let dt = 0.01;

    let (u, d, p) = risk_neutral_params(rate, vol, dt);

    assert!(u > 1.0, "u must be > 1");
    assert!(d < 1.0, "d must be < 1");
    assert!(p > 0.0 && p < 1.0, "p must be in (0,1)");
}
