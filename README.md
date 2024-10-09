# Recover Solana Wallet Private Key
If you're as dumb as me, you probably need this program.

## Overview

Recover the private key from a partial key and address. 

## N.B.

You need a partial key and address to recover the complete private key.

## Usage
Make the following Changes
```rust
// main.rs

fn main() {

    // replace the address with your own address
    let target_address: &str = "8xFxixdETqwW4kQiUzS4moRDJqRhFgwPbKMWCPKHtP9N";

    // Put indexes of the missing values. Remember nerds start counting from 0 and not 1 ;)
    let huh_index = 5;
    let haha_index = 48;
    
    [...]
}
```

Run 

```shell
cargo run
```
