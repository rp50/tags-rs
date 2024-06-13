use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add tags to a file
    Tag {
        path: std::path::PathBuf,
        tags: Vec<String>,
    },
    /// Remove tags from a file
    Untag {
        path: std::path::PathBuf,
        tags: Vec<String>,
    },
    /// List all files with a tag
    Ls { tag: String },
}

fn add_tag(tag_map: &mut HashMap<String, Vec<PathBuf>>, path: &PathBuf, tags: &Vec<String>) {
    tags.into_iter().for_each(|t| match tag_map.get_mut(t) {
        Some(v) => {
            if !v.contains(path) {
                v.push(path.to_path_buf())
            }
        }
        None => {
            tag_map.insert(t.to_string(), vec![path.to_path_buf()]);
        }
    });
}

fn rm_tag(tag_map: &mut HashMap<String, Vec<PathBuf>>, path: &PathBuf, tags: &Vec<String>) {
    tags.into_iter().for_each(|t| match tag_map.get_mut(t) {
        Some(v) => v.retain(|e| e != path),
        None => (),
    })
}

fn ls(tag_map: &HashMap<String, Vec<PathBuf>>, tag: &String) {
    match tag_map.get(tag) {
        Some(v) => {
            if v.is_empty() {
                println!("No items tagged with \"{:?}\"", tag)
            } else {
                v.into_iter()
                    .for_each(|e| println!("{}", e.to_str().expect("Invalid Path")))
            }
        }
        None => println!("No items tagged with \"{:?}\"", tag),
    }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let mut tags_file = match dirs::home_dir() {
        Some(p) => p,
        None => PathBuf::from(""),
    };
    tags_file.push(".tags-rs.json");
    let contents = match std::fs::read_to_string(&tags_file) {
        Ok(s) => s,
        _ => String::from(""),
    };

    let mut tag_map: HashMap<String, Vec<PathBuf>> = match contents.as_str() {
        "" => HashMap::new(),
        _ => match serde_json::from_str(&contents) {
            Ok(map) => map,
            _ => HashMap::new(),
        },
    };

    match &cli.command {
        Commands::Tag { path, tags } => add_tag(&mut tag_map, path, tags),
        Commands::Untag { path, tags } => rm_tag(&mut tag_map, path, tags),
        Commands::Ls { tag } => ls(&tag_map, tag),
    }

    let serialized = serde_json::to_string(&tag_map)?;
    std::fs::write(tags_file, serialized)?;

    Ok(())
}
