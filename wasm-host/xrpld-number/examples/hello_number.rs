use xrpld_number::{Number, NumberError, RoundingMode};

fn main() -> Result<(), NumberError> {
    println!("🔢 Welcome to XRPLD Number - High Precision Decimal Arithmetic!");
    println!("{}", "=".repeat(60));
    
    // Basic number creation
    println!("\n📝 Creating Numbers:");
    let zero = Number::new();
    let a = Number::from(12345);
    let b = Number::from_i64(-6789)?;
    let c = Number::from_mantissa_exponent(314159, -5)?; // π ≈ 3.14159
    
    println!("  Zero:     {}", zero);
    println!("  a:        {}", a);
    println!("  b:        {}", b);
    println!("  c (π≈):   {}", c);
    
    // Basic arithmetic
    println!("\n➕ Basic Arithmetic:");
    let sum = (&a + &b)?;
    let diff = (&a - &b)?;
    let prod = (&a * &b)?;
    let quot = (&a / &Number::from(100))?;
    
    println!("  {} + {} = {}", a, b, sum);
    println!("  {} - {} = {}", a, b, diff);
    println!("  {} × {} = {}", a, b, prod);
    println!("  {} ÷ 100 = {}", a, quot);
    
    // Comparisons
    println!("\n⚖️  Comparisons:");
    println!("  {} > {} ? {}", a, b, a > b);
    println!("  {} == {} ? {}", a, a.clone(), a == a);
    println!("  {} is zero? {}", zero, zero.is_zero());
    println!("  Sign of {}: {}", b, b.signum());
    
    // Mathematical functions
    println!("\n🧮 Mathematical Functions:");
    let sixteen = Number::from(16);
    let sqrt_16 = sixteen.sqrt()?;
    let two = Number::from(2);
    let eight = two.pow(3)?;
    let abs_b = b.abs()?;
    
    println!("  √{} = {}", sixteen, sqrt_16);
    println!("  {}³ = {}", two, eight);
    println!("  |{}| = {}", b, abs_b);
    
    // Try some larger calculations
    println!("\n💰 Working with Large Numbers:");
    let million = Number::from(1_000_000);
    let billion = Number::from(1_000_000_000);
    let large_product = (&million * &billion)?;
    let very_large = large_product.pow(2)?;
    
    println!("  1 million × 1 billion = {}", large_product);
    println!("  ({})² = {}", large_product, very_large);
    
    // Precision demonstration
    println!("\n🎯 Precision Demonstration:");
    let precise1 = Number::from_mantissa_exponent(123456789012345, -10)?;
    let precise2 = Number::from_mantissa_exponent(987654321098765, -10)?;
    let precise_sum = (&precise1 + &precise2)?;
    
    println!("  High precision 1: {}", precise1);
    println!("  High precision 2: {}", precise2);
    println!("  Sum:              {}", precise_sum);
    
    // Rounding mode demonstration
    println!("\n🔄 Rounding Modes:");
    let original_mode = Number::get_rounding_mode();
    println!("  Current rounding mode: {:?}", original_mode);
    
    // Try different rounding modes
    for &mode in &[RoundingMode::ToNearest, RoundingMode::Downward, RoundingMode::Upward] {
        Number::set_rounding_mode(mode);
        let result = Number::from_mantissa_exponent(12345, -3)?;
        println!("  Mode {:?}: {}", mode, result);
    }
    
    // Restore original mode
    Number::set_rounding_mode(original_mode);
    
    // Constants
    println!("\n📊 Number Constants:");
    let min_num = Number::min();
    let max_num = Number::max();
    let lowest_num = Number::lowest();
    
    println!("  Minimum positive: {}", min_num);
    println!("  Maximum:          {}", max_num);
    println!("  Lowest (most negative): {}", lowest_num);
    
    // Internal representation
    println!("\n🔍 Internal Representation:");
    println!("  {} -> mantissa: {}, exponent: {}", a, a.mantissa(), a.exponent());
    println!("  {} -> mantissa: {}, exponent: {}", c, c.mantissa(), c.exponent());
    
    // Assignment operations
    println!("\n📝 Assignment Operations:");
    let mut mutable_num = Number::from(100);
    println!("  Starting with: {}", mutable_num);
    
    mutable_num += &Number::from(50);
    println!("  After += 50:   {}", mutable_num);
    
    mutable_num *= &Number::from(2);
    println!("  After *= 2:    {}", mutable_num);
    
    mutable_num /= &Number::from(3);
    println!("  After /= 3:    {}", mutable_num);
    
    println!("\n✨ That's a wrap! XRPLD Number provides high-precision");
    println!("   decimal arithmetic with full Rust safety guarantees.");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_runs() {
        // Just ensure the example can run without panicking
        assert!(main().is_ok());
    }
}