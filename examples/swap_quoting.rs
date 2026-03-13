//! Example: Liquidity Swaps and Routing
//!
//! This example demonstrates swap quoting, multi-hop routing, and ROI calculations.

use octo_o_weaver::prelude::*;
use solana_sdk::pubkey::Pubkey;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Octo-Weaver Swap Quoting Example ===\n");

    let rpc_endpoint = "http://localhost:8899".to_string();

    // Initialize liquidity engine
    let liquidity = LiquidityEngine::new(rpc_endpoint)?;

    // Example tokens
    let token_a = Pubkey::new_unique();
    let token_b = Pubkey::new_unique();

    // Step 1: Get swap quote
    println!("Step 1: Get swap quote...");
    let input_amount = 1_000_000; // 1M units of token A
    let quote = liquidity.quote_swap(token_a, token_b, input_amount)?;

    println!("  Input: {} units of token A", quote.input_amount);
    println!("  Output: {} units of token B", quote.output_amount);
    println!(
        "  Price impact: {:.4}%",
        (quote.price_impact as f64 / 1_000_000.0) * 100.0
    );
    println!("  Swap fee: {} units", quote.swap_fee);
    println!("  Execution price: {:.6}", quote.execution_price);

    // Step 2: Get pool info
    println!("\nStep 2: Get liquidity pool information...");
    let pool_id = Pubkey::new_unique();
    let pool_info = liquidity.get_pool_info(pool_id)?;

    println!("  Pool ID: {}", pool_info.pool_id);
    println!("  Token A reserve: {}", pool_info.reserve_a);
    println!("  Token B reserve: {}", pool_info.reserve_b);
    println!("  Total supply: {} LP tokens", pool_info.total_supply);
    println!("  Fee tier: {:.2}%", pool_info.fee_tier as f64 / 100.0);

    // Step 3: Compare multiple routes
    println!("\nStep 3: Find best swap route...");
    let route = liquidity.find_best_route(token_a, token_b, input_amount)?;
    println!("  Best route:");
    for (i, hop) in route.iter().enumerate() {
        println!("    Hop {}: {}", i + 1, hop);
    }

    // Step 4: Calculate slippage and impact
    println!("\nStep 4: Analyze slippage and impact...");
    let expected_output = (input_amount as f64 * 0.99) as u64; // Without fees
    let actual_output = quote.output_amount;
    let slippage_amount = expected_output - actual_output;
    let slippage_percent = (slippage_amount as f64 / expected_output as f64) * 100.0;

    println!("  Expected output: {} units", expected_output);
    println!("  Actual output: {} units", actual_output);
    println!(
        "  Slippage: {} units ({:.2}%)",
        slippage_amount, slippage_percent
    );

    // Step 5: Compare multiple swap scenarios
    println!("\nStep 5: Compare different swap sizes...");
    let swap_sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in swap_sizes {
        let q = liquidity.quote_swap(token_a, token_b, size)?;
        let impact = (q.price_impact as f64 / 1_000_000.0) * 100.0;
        println!(
            "  Input: {:>10} → Output: {:>10} (Impact: {:.4}%)",
            size, q.output_amount, impact
        );
    }

    // Step 6: Liquidity provider example
    println!("\nStep 6: Liquidity provider ROI...");
    let lp_tokens_owned = 100_000;
    let pool_fee_daily = (pool_info.reserve_a as f64 * 0.003) as u64; // Estimate

    println!("  LP tokens owned: {}", lp_tokens_owned);
    println!(
        "  Pool share: {:.2}%",
        (lp_tokens_owned as f64 / pool_info.total_supply as f64) * 100.0
    );
    println!(
        "  Daily fee earnings (est.): {}",
        (pool_fee_daily as f64 * 0.001) as u64
    );
    println!(
        "  Annual fee earnings (est.): {}",
        ((pool_fee_daily as f64 * 0.001) * 365.0) as u64
    );

    println!("\n=== Swap Summary ===");
    println!("Swap execution ready for:");
    println!("  - Input: {} units of token A", input_amount);
    println!(
        "  - Expected output: {} units of token B",
        quote.output_amount
    );
    println!("  - Total fee: {} units", quote.swap_fee);
    println!("Status: ✓ Complete\n");

    Ok(())
}
