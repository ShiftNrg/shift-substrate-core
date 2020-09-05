// std
use std::path::PathBuf;
// crates
use log::info;
// substrate
use sc_cli::{SubstrateCli, Result, RuntimeVersion, Role};
use sp_core::crypto::Ss58AddressFormat;
// shiftNrg
use crate::cli::{Cli, Subcommand};

#[cfg(not(feature = "service-rewr"))]
use service::{IdentifyVariant, self};
#[cfg(feature = "service-rewr")]
use service_new::{IdentifyVariant, self as service};


fn get_exec_name() -> Option<String> {
	std::env::current_exe()
		.ok()
		.and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
		.and_then(|s| s.into_string().ok())
}

impl SubstrateCli for Cli {
	fn impl_name() -> String { "ShiftNrg".into() }

	fn impl_version() -> String { env!("SUBSTRATE_CLI_IMPL_VERSION").into() }

	fn description() -> String { env!("CARGO_PKG_DESCRIPTION").into() }

	fn author() -> String { env!("CARGO_PKG_AUTHORS").into() }

	fn support_url() -> String { "https://github.com/shiftnrg/shift-substrate-core/issues/new".into() }

	fn copyright_start_year() -> i32 { 2016 }

	fn executable_name() -> String { "shiftNrg".into() }

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let id = if id == "" {
			let n = get_exec_name().unwrap_or_default();
			["mainnet", "testnet", "devnet"].iter()
				.cloned()
				.find(|&chain| n.starts_with(chain))
				.unwrap_or("mainnet")
		} else { id };
		Ok(match id {
			"shiftNrg-dev" | "dev" => Box::new(service::chain_spec::polkadot_development_config()?),
			"shiftNrg-testnet" => Box::new(service::chain_spec::polkadot_local_testnet_config()?),
			"shiftNrg" => Box::new(service::chain_spec::polkadot_staging_testnet_config()?),
			path => {
				let path = std::path::PathBuf::from(path);

				let starts_with = |prefix: &str| {
					path.file_name().map(|f| f.to_str().map(|s| s.starts_with(&prefix))).flatten().unwrap_or(false)
				};

				// When `force_*` is given or the file name starts with the name of one of the known chains,
				// we use the chain spec for the specific chain.
				if self.run.force_kusama || starts_with("kusama") {
					Box::new(service::KusamaChainSpec::from_json_file(path)?)
				} else if self.run.force_westend || starts_with("westend") {
					Box::new(service::WestendChainSpec::from_json_file(path)?)
				} else {
					Box::new(service::PolkadotChainSpec::from_json_file(path)?)
				}
			},
		})
	}

	fn native_runtime_version(spec: &Box<dyn service::ChainSpec>) -> &'static RuntimeVersion {
		if spec.is_mainnet() {
			&service::mainnet_runtime::VERSION
		} else if spec.is_testnet() {
			&service::testnet_runtime::VERSION
		} else {
			&service::devnet_runtime::VERSION
		}
	}
}

/// Parses polkadot specific CLI arguments and run the service.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	fn set_default_ss58_version(spec: &Box<dyn service::ChainSpec>) {
		let ss58_version = Ss58AddressFormat::Shift

		sp_core::crypto::set_default_ss58_version(ss58_version);
	};

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run.base)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			let authority_discovery_enabled = cli.run.authority_discovery_enabled;
			let grandpa_pause = if cli.run.grandpa_pause.is_empty() {
				None
			} else {
				Some((cli.run.grandpa_pause[0], cli.run.grandpa_pause[1]))
			};

			info!(" ____            _                     ___ ___  ");
			info!("|  _ \ __ _  ___| |_ _   _ _ __ ___   |_ _/ _ \ ");
			info!("| |_) / _` |/ __| __| | | | '_ ` _ \   | | | | |");
			info!("|  __/ (_| | (__| |_| |_| | | | | | |  | | |_| |");
			info!("|_|   \__,_|\___|\__|\__,_|_| |_| |_| |___\___/ ");
			info!("	    ____  _     _  __ _   _   _              ");
			info!("	   / ___|| |__ (_)/ _| |_| \ | |_ __ __ _    ");
			info!("	   \___ \| '_ \| | |_| __|  \| | '__/ _` |   ");
			info!("	    ___) | | | | |  _| |_| |\  | | | (_| |   ");
			info!("	   |____/|_| |_|_|_|  \__|_| \_|_|  \__, |   ");
			info!("   		  						    |___/    ");

			runner.run_node_until_exit(|config| {
				let role = config.role.clone();

				match role {
					Role::Light => service::build_light(config).map(|(task_manager, _)| task_manager),
					_ => service::build_full(
						config,
						None,
						authority_discovery_enabled,
						grandpa_pause,
					).map(|r| r.0),
				}
			})
		},
		Some(Subcommand::Base(subcommand)) => {
			let runner = cli.create_runner(subcommand)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			if chain_spec.is_devnet() {
				runner.run_subcommand(subcommand, |config|
					service::new_chain_ops::<
						service::devnet_runtime::RuntimeApi,
						service::ShiftNrgDevnetExecutor,
					>(config)
				)
			} else if chain_spec.is_testnet() {
				runner.run_subcommand(subcommand, |config|
					service::new_chain_ops::<
						service::testnet_runtime::RuntimeApi,
						service::ShiftNrgTestnetExecutor,
					>(config)
				)
			} else { 
				runner.run_subcommand(subcommand, |config|
					service::new_chain_ops::<
						service::mainnet_runtime::RuntimeApi,
						service::ShiftNrgExecutor,
					>(config)
				)
			}
		},
	}
}