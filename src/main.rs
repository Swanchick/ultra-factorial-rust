mod dnumber;

use std::io::Result;
use dnumber::DNumber;

fn main() -> Result<()> {
    let mut dnumber1 = DNumber::new();
    
    dnumber1.factorial(500);
    dnumber1.show();

    Ok(())
}
