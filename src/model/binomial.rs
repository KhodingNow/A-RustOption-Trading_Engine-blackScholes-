
// PUBLIC API

use crate::types::Spot;

/// Cox–Ross–Rubinstein risk-neutral parameters

/// Returns (u, d, p)

pub fn risk_neutral_params(

    rate: f64,
    volatility: f64,
    dt: f64,
    
) -> (f64, f64, f64) {

    let u = (volatility * dt.sqrt()).exp();
    let d = 1.0 / u;
    let p = ((rate * dt).exp() - d) / (u - d);

    (u, d, p)
}

/// Terminal asset prices at maturity (CRR tree)
///
/// Returns N+1 prices for N steps

pub fn terminal_prices(

    spot: Spot,
    u: f64,
    d: f64,
    steps: usize,
    
) -> Vec<f64> {

    let mut prices = Vec::with_capacity(steps + 1);

    // k = number of up moves
    for k in (0..=steps).rev() {
        let up_moves = k;
        let down_moves = steps - k;

        let price =
            spot.0 * u.powi(up_moves as i32) * d.powi(down_moves as i32);

        prices.push(price);
    }

    prices
}

// -------------------------
// Binomial pricing types
// -------------------------

pub struct BinomialParams {
    pub spot: f64,
    pub strike: f64,
    pub rate: f64,
    pub volatility: f64,
    pub maturity: f64,
    pub steps: usize,
}


#[derive(Copy, Clone, PartialEq)]
pub enum OptionStyle {
    European,
    American,
}

#[derive(Copy, Clone, PartialEq)]
pub enum OptionType {
    Call,
    Put,

}

// -------------------------
// Payoff logic
// -------------------------

pub fn payoff(price: f64, strike: f64, kind: OptionType) -> f64 {
    match kind {
        OptionType::Call => (price - strike).max(0.0),
        OptionType::Put  => (strike - price).max(0.0),
    }
}

pub fn terminal_payoffs(
    prices: &[f64],
    strike: f64,
    kind: OptionType,

) -> Vec<f64> {
    prices

        .iter()
        .map(|&price| payoff(price, strike, kind))
        .collect()
}

// -------------------------
// Pricing entry point
// -------------------------

pub fn price(
    params: &BinomialParams,
    style: OptionStyle,
    kind: OptionType,
    
) -> f64  {

    let dt = params.maturity / params.steps as f64;

    let (u, d, p) = risk_neutral_params(params.rate, params.volatility, dt);

    let discount = (-params.rate *dt).exp();

    // Terminal prices

    let prices = terminal_prices(
        crate::types::Spot(params.spot),

        u,
        d,
        params.steps,
    );
    
    // Terminal payoffs
    
    let mut values: Vec<f64> = prices
        .iter()
        .map(|&s| payoff(s, params.strike, kind))
        .collect(); 

    // Backward induction

   for step in (0..params.steps).rev() {

    for i in 0..=step {

        let hold_value = 
        
              (p * values[i] + (1.0 - p) * values[i + 1]) * discount;
        
        if style == OptionStyle::American {

            let stock_price = 
                 params.spot 

                 * u.powi(i as i32) 
                 * d.powi((step - i) as i32);

            let intrinsic = match kind {

                OptionType::Call => 
                (stock_price - params.strike).max(0.0),

                OptionType::Put => 
                (params.strike - stock_price).max(0.0),
                
            };

            values[i] = hold_value.max(intrinsic);
            
        } else {

            values[i] = hold_value;
        }
    } 
    
   }       
    
    // Root node value
    values[0]                 

}
