use bs58;

pub fn get_address_from_private_key_array(private_key_array: [u8; 64]) -> String {
    let address: String = bs58::encode(&private_key_array[32..]).into_string();
    // println!("{:?}", bs58::decode(new_address).into_vec().unwrap());
    address
}

fn main() {
    // Your private key array
    let mut private_key_array: [u8; 64] = [70,78,110,94,85,0,135,80,166,45,90,217,5,156,156,100,137,7,65,211,7,77,29,139,89,44,153,24,133,174,93,16,118,41,171,77,210,111,83,115,104,39,216,149,249,245,115,111,22,23,129,1,60,255,189,99,30,135,251,254,177,32,169,53];

    let target_address: &str = "8xFxixdETqwW4kQiUzS4moRDJqRhFgwPbKMWCPKHtP9N";

    // Missing values indices
    let huh_index = 5;
    let haha_index = 48;

    // Iterate through possible values for 'huh' and 'haha'
    for huh_value in 0u8..=255 {
        for haha_value in 0u8..=255 {
            // Set the missing values
            private_key_array[huh_index] = huh_value;
            private_key_array[haha_index] = haha_value;

            // Get the address for the current private key
            let address = get_address_from_private_key_array(private_key_array);

            // Check if the address matches the target
            if address == target_address {
                println!("Found matching address! Private key: {:?}", private_key_array);
                println!("Ethereum Address: {:?}", address);
                return;
            }
        }
    }

    println!("No matching private key found.");

}
