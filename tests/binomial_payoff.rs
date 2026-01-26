// testing a Node, not a tree.


use rust_option_engine::model::binomial::payoff;
use rust_option_engine::model::binomial::OptionType;

// TEST - call payoff at maturity

#[test]
fn call_payoff_is_correct() {
    let spot = 120.0;
    let strike = 100.0;

    let payoff = payoff(spot, strike, OptionType::Call);

    
    assert!(

        (payoff - 20.0).abs() < 1e-10,

        "call payoff incorrect: got {payoff}"

    );    
}


// TEST - put payoff at maturity

#[test]
fn put_payoff_is_correct() {

    let spot = 80.0;
    let strike = 100.0;

    let payoff = payoff(spot, strike, OptionType::Put);


    assert!(

        (payoff - 20.0).abs() < 1e-10,

        "put payoff incorrect: got {payoff}"
    );
    
}


//TEST - payoff is never negative

#[test]
fn payoff_is_never_negative() {

    let payoff_call = payoff(90.0, 100.0, OptionType::Call);
    let payoff_put = payoff(110.0, 100.0, OptionType::Put);

    assert!(payoff_call >= 0.0);
    assert!(payoff_put >= 0.0);
     
}


