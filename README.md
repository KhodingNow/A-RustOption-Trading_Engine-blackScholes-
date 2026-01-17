rust-option-engine

A test-driven, invariant-based implementation of Black-Scholes option pricing in Rust.

Overview.

rust-option-engine is an educational and experimental Rust project that explores European option pricing through correctness-first engineering practices.

Rather than focusiing on raw performance or trading strategies, the project emphasizes:

- explicit financial assumptions,
- mathematically grounded pricing formulas,
- and tests that encode economic invariants.

The goal is to demonstrate how modern Rust tooling and disciplined testing can be used to reason about financial models safely and incrementally.

Motivation.

Financial models are numerically sensitive and easy to misuse.
Small implementation errors can silently violate fundamental principles such as:

- intrinsic value bounds,
- put-call parity,
- monotonocity with respect to spot,
- or reasonable bounds on Greeks.

This projects treats such principles as executable specifications, enforced directly through tests.

Engineering Philosophy

Test-Driven Development Influence

The development style of this project is strongly influenced by Ken Youens-Clark's teaching and writing on test-driven development, particularly:

    * Command-Line Rust (2024)

 Clark's emphasis on:

 - writing tests before implementation,
 - letting failures guide design,
 - and treating tests as documentation
  
 directly shaped how this codebase evolved.

While these ideas trace back to the foundational work of Kent Beck, the practical approach used here reflects Youens-Clark's modern, Rust-centric intepretation.

Tests as Financial Specifications.

In this project, tests do more that check numerical outputs.

They encode financial invariants, including:

- call price >= intrinsic value,
- put price >= intrinsic value,
- put-call parity,
- stability under small pertubations of spot price,
- bounded finite-difference Greeks.

This mirrors real-world model validation practices, where correctness is established structurally rather than through a handful of known values.

Implemented Models

Black-Scholes (European Options)
- Closed-form pricing for calls and puts
- Standard assumptions;
     * log-normal underlying
     * constant volatility
     * constant risk-free rate
     * no dividends
     
Implementation lives in:

   src/model/black_scholes.rs
   

Testing Strategy

Unit Tests

- Positivity of option prices
- Behaviour near zero time to maturity
- Basic sanity checks 
 

Property-based Tests (via proptest)

- Prices respect intrinsic value over wide input ranges
- Put-call parity holds within numerical tolerance
- Prices behave consistently across spot/strike combinations

Numerical Sensitivity Tests

- Central-difference approximation of Delta
- Bounds on Delta for call options (0 <= DELTA >= 1)

Floating-point tolerance (epsilon) are used deliberatley to distinguish numerical noise from genuine violations.

Project Structure

src/
├── lib.rs
├── main.rs
├── types.rs          # Strongly-typed financial primitives
├── greeks.rs         # Greeks (incremental)
└── model/
    ├── mod.rs
    └── black_scholes.rs

tests/
└── known_values.rs   # Property & invariant tests

benches/
└── pricing.rs        # Criterion benchmarks (optional)

Benchmarks (Optional)

Criterion benchmarks are included to explore performance characteristics of pricing functions once correctness is established.

The emphasis:

| correct first, fast later.

Background.

The author (myself), other than a Post-Graduate Certificate in Econometrix (Wits, 2003-2005, which covered quantitative training in Calculus, Statistics, Micro and Macro Economics, International Trade as modules )  has no other formal or further Reseach training in Quants. 

I am approaching derivatives pricing from a Software correctness and model-validation perspective.

Although options trading is not my primary professional domain - I am repurposing my Econometrix training I obtained at Wits University in my course using my recently acquired Rust Developer skills in the last two and a half years.

This project demonstrates how rigorous testing methodologies can be used to learn, validate, and reason about financial models step by step.

References & Further reading.

Options & Quantitative Finance

- Hull, J. - Options, Futures, and Other Derivatives
- Natenberg, S. - Option Volatility & Pricing
- Glasserman, P. - Monte Carlo Methods in Financial Engineering

Software Engineering & Testing

- Ken Youens-Clark. - Command-Line Rust (2024)
- Beck, K - Test-Driven Deelopment: (foundational influence).
- I used DeepSeek / Gpt to debug some elements of the code - prompt these to explain some Mathematical concepts of Black-Scholes I did not understand at first for code veracity and rigour. 
Disclaimer


This project is for educational, my education as a Rust /API Security developer only.
It is not intended for production trading or financial advice.

I intend to submit it to RustaceansAfrica Hackathon (4-18January 2026).

Why this project exists?

To explore how disciplined testing, strong typing and explicit assumptions can make complex financial models easier to reason about - even when entering a new domain.


