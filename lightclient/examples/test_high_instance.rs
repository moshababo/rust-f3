// Test F3 with high instance
use anyhow::Result;
use filecoin_f3_lightclient::LightClient;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Testing F3 light client with high instance...");
    
    let endpoint = "http://api.calibration.node.glif.io/rpc/v1";
    let network = "calibrationnet";
    let start_instance = 742420u64;

    let mut client = LightClient::new(endpoint, network)?;
    println!("Initializing with instance {}", start_instance);
    let mut state = client.initialize(start_instance).await?;
    println!("✅ Initialized! Power table size: {}", state.power_table.len());
    println!("State instance: {}", state.instance);

    // Try to validate the first certificate
    println!("\nFetching certificate for instance {}...", start_instance);
    let cert = client.get_certificate(start_instance).await?;
    println!("✅ Fetched certificate, ec_chain length: {}", cert.ec_chain.suffix().len());

    println!("\nValidating certificate for instance {}...", start_instance);
    match client.validate_certificates(&state, &[cert]) {
        Ok(new_state) => {
            println!("✅ Certificate validated successfully!");
            println!("New state instance: {}", new_state.instance);
            println!("New power table size: {}", new_state.power_table.len());
            state = new_state;
        }
        Err(e) => {
            println!("❌ Validation failed: {}", e);
            return Err(e.into());
        }
    }

    // Try the next one
    println!("\nFetching certificate for instance {}...", start_instance + 1);
    let cert2 = client.get_certificate(start_instance + 1).await?;
    println!("✅ Fetched certificate");

    println!("\nValidating certificate for instance {}...", start_instance + 1);
    match client.validate_certificates(&state, &[cert2]) {
        Ok(new_state) => {
            println!("✅ Certificate validated successfully!");
            println!("New state instance: {}", new_state.instance);
        }
        Err(e) => {
            println!("❌ Validation failed: {}", e);
            return Err(e.into());
        }
    }

    println!("\n🎉 SUCCESS! F3 light client works with high instances!");
    Ok(())
}

