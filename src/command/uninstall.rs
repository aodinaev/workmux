use anyhow::Result;
use console::style;

use crate::agent_setup::{self, Agent};
use crate::skills;
use crate::xdg;

/// Run the uninstall process.
///
/// Removes status tracking hooks, skills, and XDG cache/state dirs for
/// all known agents, regardless of detection. This is a hidden
/// implementation detail for the uninstall shell script -- not meant for
/// end users directly.
pub fn run(dry_run: bool) -> Result<()> {
    if dry_run {
        println!("  {} Dry run mode", style("●").dim());
    }
    println!();

    // Phase 1: Status tracking hooks (try ALL agents, not just detected ones)
    println!("  {} Status tracking hooks", style("●").dim());
    for agent in Agent::all() {
        if dry_run {
            println!(
                "    {} {:12} Would remove hooks for {}",
                style("~").dim(),
                agent.name(),
                agent.name()
            );
        } else {
            let result = agent_setup::uninstall_one(agent);
            match result {
                Ok(msg) => println!("    {} {:12} {}", style("~").dim(), agent.name(), msg),
                Err(e) => println!("    {} {:12} {}", style("~").yellow(), agent.name(), e),
            }
        }
    }
    println!();

    // Phase 2: Skills
    println!("  {} Skills", style("●").dim());
    for agent in Agent::all() {
        if skills::skills_dir(agent).is_some() {
            if dry_run {
                println!(
                    "    {} {:12} Would remove skills for {}",
                    style("~").dim(),
                    agent.name(),
                    agent.name()
                );
            } else {
                let result = skills::remove_skills(agent);
                match result {
                    Ok(msg) => println!("    {} {:12} {}", style("~").dim(), agent.name(), msg),
                    Err(e) => println!("    {} {:12} {}", style("~").yellow(), agent.name(), e),
                }
            }
        }
    }
    println!();

    // Phase 3: XDG dirs (only cache + state, NOT config)
    println!("  {} Data directories", style("●").dim());
    let dirs = [("Cache", xdg::cache_dir()), ("State", xdg::state_dir())];

    for (label, path_result) in &dirs {
        match path_result {
            Ok(path) if path.exists() => {
                if dry_run {
                    println!(
                        "    {} Would remove {} ({})",
                        style("~").dim(),
                        label.to_lowercase(),
                        path.display()
                    );
                } else {
                    match std::fs::remove_dir_all(path) {
                        Ok(()) => println!(
                            "    {} Removed {} ({})",
                            style("~").dim(),
                            label.to_lowercase(),
                            path.display()
                        ),
                        Err(e) => println!(
                            "    {} Failed to remove {}: {}",
                            style("~").yellow(),
                            label.to_lowercase(),
                            e
                        ),
                    }
                }
            }
            Ok(_path) => println!(
                "    {} No {} directory",
                style("~").dim(),
                label.to_lowercase()
            ),
            Err(e) => println!("    {} Could not resolve: {}", style("~").yellow(), e),
        }
    }

    Ok(())
}
