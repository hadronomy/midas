use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Effects, Style};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum VersionFormat {
    /// Display the version as a plain text.
    Text,
    /// Display the version as a JSON.
    Json,
    /// Display the version as a TOML.
    Toml,
}

// Configures Clap v3-style help menu colors
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser)]
#[command(name = "midas", version = "0.1.0")]
#[command(about = "Awesome templates.")]
#[command(propagate_version = true)]
#[command(
    after_help = "Use `midas help` for more details.",
    after_long_help = "",
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true
)]
#[command(styles=STYLES)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Box<Command>,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    #[command(help_template = "\
{about-with-newline}
{usage-heading} {usage}{after-help}
",
        after_help = format!("\
{heading}Options:{heading:#}
  {option}--no-pager{option:#} Disable pager when printing help
",
            heading = Style::new().bold().underline(),
            option = Style::new().bold(),
        ),
    )]
    Help(HelpArgs),
}

#[derive(Args, Clone)]
pub struct HelpArgs {
    #[arg(long)]
    no_pager: bool,
}
