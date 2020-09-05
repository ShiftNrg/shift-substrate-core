use structopt::StructOpt;

#[allow(missing_docs)]
#[derive(Debug, StructOpt)]
pub enum Subcommand {
	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	#[allow(missing_docs)]
	#[structopt(name = "validation-worker", setting = structopt::clap::AppSettings::Hidden)]
	ValidationWorker(ValidationWorkerCommand),
}

#[allow(missing_docs)]
#[derive(Debug, StructOpt)]
pub struct ValidationWorkerCommand {
	#[allow(missing_docs)]
	pub mem_id: String,
}

#[allow(missing_docs)]
#[derive(Debug, StructOpt)]
pub struct RunCmd {
	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub base: sc_cli::RunCmd,

	/// Force using ShiftNrg native runtime.
	#[structopt(long = "force-shiftNrg")]
	pub force_shiftNrg: bool,

	/// Enable the authority discovery module on validator or sentry nodes.
	///
	/// When enabled:
	///
	/// (1) As a validator node: Make oneself discoverable by publishing either
	///     ones own network addresses, or the ones of ones sentry nodes
	///     (configured via the `sentry-nodes` flag).
	///
	/// (2) As a validator or sentry node: Discover addresses of validators or
	///     addresses of their sentry nodes and maintain a permanent connection
	///     to a subset.
	#[structopt(long = "enable-authority-discovery")]
	pub authority_discovery_enabled: bool,

	/// Setup a GRANDPA scheduled voting pause.
	///
	/// This parameter takes two values, namely a block number and a delay (in
	/// blocks). After the given block number is finalized the GRANDPA voter
	/// will temporarily stop voting for new blocks until the given delay has
	/// elapsed (i.e. until a block at height `pause_block + delay` is imported).
	#[structopt(long = "grandpa-pause", number_of_values(2))]
	pub grandpa_pause: Vec<u32>,
}

#[allow(missing_docs)]
#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: RunCmd,
}