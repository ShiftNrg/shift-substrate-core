use log::info;
use wasm_bindgen::prelude::*;
use substrate_browser_utils::{
	Client,
    browser_configuration, 
    set_console_error_panic_hook, 
    init_console_log,
    start_client,
};
use std::str::FromStr;

/// Starts the client.
#[wasm_bindgen]
pub async fn start_client(chain_spec: String, log_level: String) -> Result<Client, JsValue> {
	start_inner(chain_spec, log_level)
		.await
		.map_err(|err| JsValue::from_str(&err.to_string()))
}

async fn start_inner(chain_spec: String, log_level: String) -> Result<Client, Box<dyn std::error::Error>> {
	set_console_error_panic_hook();
	init_console_log(log_level.parse()?)?;

	let chain_spec = service::ShiftNrgChainSpec::from_json_bytes(chain_spec.as_bytes().to_vec())
		.map_err(|e| format!("{:?}", e))?;
	let config = browser_configuration(chain_spec).await?;

	info!("ShiftNrg browser node");
	info!("  version {}", config.impl_version);
	info!("  by Pactum IO, 2016-2020");
	info!("📋 Chain specification: {}", config.chain_spec.name());
	info!("🏷  Node name: {}", config.network.node_name);
	info!("👤 Role: {}", config.display_role());

	// Create the service. This is the most heavy initialization step.
	let (task_manager, rpc_handlers) = service::build_light(config).map_err(|e| format!("{:?}", e))?;

	Ok(start_client(task_manager, rpc_handlers))
}