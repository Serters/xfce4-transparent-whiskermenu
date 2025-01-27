mod color_updates;
mod config;
mod regex_patterns;
mod utils;

use clap::{CommandFactory, Parser};
use std::error::Error;

use crate::color_updates::{update_border, update_panel, update_search_bar, update_whiskar_menu};
use crate::config::create_default_config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "xfce4-transparent-whiskermenu")]
struct Cli {
    #[arg(long)]
    updatepanel: bool,

    #[arg(long)]
    updatewhisker: bool,

    #[arg(long)]
    updatesearch: bool,

    #[arg(long)]
    updateall: bool,

    #[arg(long)]
    createconfig: bool,

    #[arg(long)]
    updateborder: bool,
}
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if !cli.updatepanel
        && !cli.updatewhisker
        && !cli.updatesearch
        && !cli.updateall
        && !cli.createconfig
    {
        Cli::command().print_help()?;
        return Ok(());
    }

    if cli.createconfig {
        create_default_config()?;
        return Ok(());
    }

    if cli.updateall || cli.updatewhisker {
        update_whiskar_menu()?;
    }

    if cli.updateall || cli.updatesearch {
        update_search_bar()?;
    }

    if cli.updateall || cli.updatepanel {
        update_panel()?;
    }

    if cli.updateall || cli.updateborder {
        update_border()?;
    }

    Ok(())
}
