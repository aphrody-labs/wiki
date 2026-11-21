// SPDX-License-Identifier: Apache-2.0
//! `cargo xtask`: repository tasks that read only the commit (`git ls-tree`, `git show`), so their
//! output depends on the commit and not on build outputs, caches or uncommitted edits.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xtask", about = "Shenron repository tasks")]
struct Cli {
    #[command(subcommand)]
    task: Task,
}

#[derive(Subcommand)]
enum Task {
    /// Write MAP.tsv (areas, tracked files, bytes, tier, gate); with --check, fail when it is stale.
    Map {
        #[arg(long)]
        check: bool,
    },
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("xtask: {error:#}");
            ExitCode::FAILURE
        },
    }
}

fn run(cli: Cli) -> Result<ExitCode> {
    let root = repository_root()?;
    match cli.task {
        Task::Map { check } => map(&root, check),
    }
}

fn repository_root() -> Result<PathBuf> {
    let output = git(Path::new("."), &["rev-parse", "--show-toplevel"])?;
    Ok(PathBuf::from(String::from_utf8(output)?.trim()))
}

fn git(root: &Path, arguments: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()
        .with_context(|| format!("running git {}", arguments.join(" ")))?;
    if !output.status.success() {
        bail!("git {} failed: {}", arguments.join(" "), String::from_utf8_lossy(&output.stderr));
    }
    Ok(output.stdout)
}

/// One tracked blob of HEAD: its path and size in bytes (0 for submodules).
struct Blob {
    path: String,
    bytes: u64,
}

fn head_blobs(root: &Path) -> Result<Vec<Blob>> {
    let listing = git(root, &["ls-tree", "-r", "-l", "-z", "HEAD"])?;
    let mut blobs = Vec::new();
    for record in listing.split(|byte| *byte == 0).filter(|record| !record.is_empty()) {
        let record = String::from_utf8_lossy(record);
        let Some((meta, path)) = record.split_once('\t') else { continue };
        // "<mode> <type> <object> <size>"; submodules report "-" as size.
        let bytes = meta.split_whitespace().nth(3).and_then(|size| size.parse().ok()).unwrap_or(0);
        blobs.push(Blob { path: path.to_string(), bytes });
    }
    Ok(blobs)
}

fn area_of(path: &str) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    match parts.as_slice() {
        ["crates", name, _, ..] => format!("crates/{name}"),
        [top, _, ..] => (*top).to_string(),
        _ => "(root)".to_string(),
    }
}

fn tier(area: &str) -> &'static str {
    let top = area.split('/').next().unwrap_or(area);
    match top {
        "(root)" => "config",
        "replica" => "docs",
        _ if top.starts_with('.') => "config",
        _ => "first-party",
    }
}

fn gate(area: &str) -> String {
    let mut parts = area.splitn(2, '/');
    match (parts.next(), parts.next()) {
        (Some("crates"), Some(name)) => {
            format!("cargo clippy -p {name} --all-targets -- -D warnings (fr1)")
        },
        _ => "just check".to_string(),
    }
}

fn map(root: &Path, check: bool) -> Result<ExitCode> {
    let mut areas: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    for blob in head_blobs(root)?.into_iter().filter(|blob| blob.path != "MAP.tsv") {
        let entry = areas.entry(area_of(&blob.path)).or_default();
        entry.0 += 1;
        entry.1 += blob.bytes;
    }
    let mut output = String::from("area\tfiles\tbytes\ttier\tgate\n");
    for (area, (files, bytes)) in &areas {
        output.push_str(&format!("{area}\t{files}\t{bytes}\t{}\t{}\n", tier(area), gate(area)));
    }
    let target = root.join("MAP.tsv");
    if check {
        let current = std::fs::read_to_string(&target).unwrap_or_default();
        if current != output {
            eprintln!("MAP.tsv is stale: run `cargo xtask map`");
            return Ok(ExitCode::FAILURE);
        }
        println!("MAP.tsv up to date ({} areas)", areas.len());
    } else {
        std::fs::write(&target, &output).context("writing MAP.tsv")?;
        println!("MAP.tsv: {} areas, {} bytes", areas.len(), output.len());
    }
    Ok(ExitCode::SUCCESS)
}
