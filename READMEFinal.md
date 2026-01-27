Implemented Models

Black-Scholes (European OPtions)

= Closed-form pricing for European calls and puts
= Standard assumptions:

    - Log-normal underlying
    - Constant volatility
    - Constant risk-free rate
    - No dividends

Implementation:

    src/model/black_scholes.rs

Binomial Tree (European Options)

    - Discrete-time lattice model
    - Backward induction pricing
    - Serves as a bridge toward American options and early exercise logic

Implementation:

    src/model/binomial.rs


Getting started.

Clone the repository and run the full test suite:

    git clone https://github.com/<.your-username>/rust-option-engine.git
    cd rust-option-engine
    cargo test
    
Run the example binary:

    cargo run
    
All pricing logic is exercised through tests - a passing test suite is considered the primary success criterion.

Testing Strategy

Unit Tests
  
    - Positivity of adoption prices
    - Behaviour near zero time to maturity
    - Sanity checks on closd-form solutions

Property-Based Tests (via proptest)

    tests/known_values.rs

Examples include:

    - call_price_is_never_below_intrinsic_value
    - call_price_respects_intrinsic_value_property
    - european_put_respects_discounted_lower_bound
    - put_call_parity_holds

These tests validate correctness across wide ranges of inputs, not just fixed examples.

Numerical Sensitivity & Greeks

    - Central-difference Delta approximation
    - Bounds enforced on Delta fall call options

Floating-point tolerances (epsilon) are used deliberately to distinguish numeracal nose from genuine violations of financial principles.

Project Structure:

src/
├── lib.rs
├── main.rs
├── types.rs # Strongly-typed financial primitives
├── greeks.rs # Greeks (incremental)
└── model/
├── mod.rs
├── black_scholes.rs
└── binomial.rs


tests/
└── known_values.rs # Invariant & property-based tests


benches/
└── pricing.rs # Criterion benchmarks (optional)

Benchmarks (Optional)

Criterion benchmarks are included to explore performance charecteristics after correctness is established.

    | Correct first. Fast later.


What did I learn while Building This Engine?

    - One is that - Financial models benefit from ernomously from executabale invariants, not just equastions in comments
    - Property-based testing is a natural fir for quantitative finance
    - Strong typing helps surface conceptual mistakes early (e.g. mixing rates, times, and prices).
    - Backward induction models (binomial trees) are easier to reason about when tested against structural guarantees rather than prices alone.
    - Rust's tooling encourages incremental correctness instead of premature optimization

Most importantly, this project reinforced that learning anew quantitative domain is safer and faster when correctness is enforced continuously.

Background.

I hold a Postgraduate Certificate in Econometrics (Wits 2003-2005), covering calculus, statistics, macro/micro economics, Int Trade.

Also a BA Psychology Honours (Fort Hare 1989-1992).

Outside of that training, this project represents a software-engineering-driven re-entry into derivative pricing, using Rust skills developed over the last 2.5 years.

The focus is not trading startegy, but model correctness, validation and reasoning.

Rust Africa-Hackathon.

This project is intended for submission to the RustaceansAfrica Hackathon (4-31 January 2026).

It demonstrates how:

    - disciplined testing,
    - strong typing,
    - and explicit assumptions

 can make complex financial models easier to reason about - even when entering a new domain.

References & Further Reading
Options & Quantitative Finance

Hull, J. — Options, Futures, and Other Derivatives

Natenberg, S. — Option Volatility & Pricing

Glasserman, P. — Monte Carlo Methods in Financial Engineering

Software Engineering & Testing

Youens-Clark, K. — Command-Line Rust (2024)

Beck, K. — Test-Driven Development

Large language models (including GPT-based tools) were used selectively to clarify mathematical concepts and validate reasoning, not to generate trading logic.

 

DISCLAIMER!

This project is for educational purposes only.

It is not intended for production trading, investment decisions, or financial advice.


WHY THIS PROJECT EXISTS?

It is to explore how disciplined testing strong typing, and explicit assumptions can make complex financial modelseasier to reason about - even when learning a new quantitative domain from the outside.

 






