use crate::parser::ShellCompletions;
use crate::{cli::build_cli, parser::OutDir};
use clap::Command;
use clap_complete::{generate_to, shells};
use std::path::PathBuf;

pub fn generate_shell_completions(
    shell: ShellCompletions,
) -> anyhow::Result<()> {
    let shell_name = match_shell(shell);
    let mut cmd = build_cli();
    let path = generate_completions_to(shell_name, &mut cmd)?;
    println!("Generated shell completions to: {}", path.display());
    Ok(())
}

fn default_dir(outdir: Option<PathBuf>) -> PathBuf {
    outdir.unwrap_or_else(|| PathBuf::from("."))
}

fn generate_completions_to(
    shell_name: ShellName,
    cmd: &mut Command,
) -> anyhow::Result<PathBuf> {
    match shell_name {
        ShellName::Bash { name, outdir } => {
            generate_to(name, cmd, "geolocate", default_dir(outdir))
                .map_err(|error| anyhow::anyhow!("{}", error))
        }
        ShellName::Zsh { name, outdir } => {
            generate_to(name, cmd, "geolocate", default_dir(outdir))
                .map_err(|error| anyhow::anyhow!("{}", error))
        }
        ShellName::Fish { name, outdir } => {
            generate_to(name, cmd, "geolocate", default_dir(outdir))
                .map_err(|error| anyhow::anyhow!("{}", error))
        }
        ShellName::PowerShell { name, outdir } => {
            generate_to(name, cmd, "geolocate", default_dir(outdir))
                .map_err(|error| anyhow::anyhow!("{}", error))
        }
        ShellName::Elvish { name, outdir } => {
            generate_to(name, cmd, "geolocate", default_dir(outdir))
                .map_err(|error| anyhow::anyhow!("{}", error))
        }
    }
}

fn match_shell(shell: ShellCompletions) -> ShellName {
    match shell {
        ShellCompletions::Bash(OutDir { path: outdir }) => ShellName::Bash {
            name: shells::Bash,
            outdir,
        },
        ShellCompletions::Zsh(OutDir { path: outdir }) => ShellName::Zsh {
            name: shells::Zsh,
            outdir,
        },
        ShellCompletions::Fish(OutDir { path: outdir }) => ShellName::Fish {
            name: shells::Fish,
            outdir,
        },
        ShellCompletions::PowerShell(OutDir { path: outdir }) => {
            ShellName::PowerShell {
                name: shells::PowerShell,
                outdir,
            }
        }
        ShellCompletions::Elvish(OutDir { path: outdir }) => {
            ShellName::Elvish {
                name: shells::Elvish,
                outdir,
            }
        }
    }
}

pub enum ShellName {
    Bash {
        name: shells::Bash,
        outdir: Option<PathBuf>,
    },
    Zsh {
        name: shells::Zsh,
        outdir: Option<PathBuf>,
    },
    Fish {
        name: shells::Fish,
        outdir: Option<PathBuf>,
    },
    PowerShell {
        name: shells::PowerShell,
        outdir: Option<PathBuf>,
    },
    Elvish {
        name: shells::Elvish,
        outdir: Option<PathBuf>,
    },
}
