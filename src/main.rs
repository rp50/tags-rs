use clap::{Parser, Subcommand};
use std::collections::HashMap;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // add tags to a file
    Add {
        path: std::path::PathBuf,
        tags: Vec<String>,
    },
    // remove tags from a file
    Rm {
        path: std::path::PathBuf,
        tags: Vec<String>,
    },
    // get all tags for a file
    Ls {
        tag: String,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let mut tags_file = match dirs::home_dir() {
        Some(p) => p,
        None => std::path::PathBuf::from(""),
    };
    tags_file.push(".tags-rs.json");
    let contents = match std::fs::read_to_string(&tags_file) {
        Ok(s) => s,
        _ => String::from(""),
    };

    let mut tag_map: HashMap<String, Vec<std::path::PathBuf>> = match contents.as_str() {
        "" => HashMap::new(),
        _ => match serde_json::from_str(&contents) {
            Ok(map) => map,
            _ => HashMap::new(),
        },
    };

    match &cli.command {
        Commands::Add { path, tags } => tags.into_iter().for_each(|t| match tag_map.get_mut(t) {
            Some(v) => {
                if !v.contains(path) {
                    v.push(path.to_path_buf())
                }
            }
            None => {
                tag_map.insert(t.to_string(), vec![path.to_path_buf()]);
            }
        }),
        Commands::Rm { path, tags } => tags.into_iter().for_each(|t| match tag_map.get_mut(t) {
            Some(v) => v.retain(|e| e != path),
            None => (),
        }),
        Commands::Ls { tag } => match tag_map.get(tag) {
            Some(v) => {
                if v.is_empty() {
                    println!("No items tagged with \"{:?}\"", tag)
                } else {
                    v.into_iter()
                        .for_each(|e| println!("{}", e.to_str().expect("Invalid Path")))
                }
            }
            None => println!("No items tagged with \"{:?}\"", tag),
        },
    }

    let serialized = serde_json::to_string(&tag_map)?;
    std::fs::write(tags_file, serialized)?;

    Ok(())
}
