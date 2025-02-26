use structopt::StructOpt;
use sc_cli::RunCmd;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub subcommand: Option<Subcommand>,

    /// Set local port to listen WhiteNoise protocols.
    #[structopt(long)]
    pub noise_port: Option<String>,

    /// Address of the WhiteNoise Network bootstrap node. If it's not set, start as WhiteNoise bootstrap node by default.
    #[structopt(long)]
    pub noise_bootstrap: Option<String>,

    #[structopt(flatten)]
    pub run: RunCmd,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Key management cli utilities
    Key(sc_cli::KeySubcommand),
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

    /// The custom benchmark subcommmand benchmarking runtime pallets.
    #[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}
