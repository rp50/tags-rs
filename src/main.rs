use clap::{Parser, Subcommand};
use std::collections::HashMap;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        path: std::path::PathBuf,
        tags: Vec<String>,
    },
    Ls {
        tag: String,
    },
    Rm {
        path: std::path::PathBuf,
        tags: Vec<String>,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let contents = match std::fs::read_to_string(".tags-rs.json") {
        Ok(s) => s,
        _ => String::from(""),
    };

    let mut tag_map: HashMap<String, Vec<std::path::PathBuf>> = match contents.as_str() {
        "" => match serde_json::from_str(&contents) {
            Ok(hm) => hm,
            _ => HashMap::new(),
        },
        _ => HashMap::new(),
    };

    match &cli.command {
        Commands::Add { path, tags } => tags.into_iter().for_each(|t| match tag_map.get_mut(t) {
            Some(v) => v.push(path.to_path_buf()),
            None => {
                tag_map.insert(t.to_string(), vec![path.to_path_buf()]);
                ()
            }
        }),
        Commands::Ls { tag } => {}
        Commands::Rm { path, tags } => {}
    }

    let serialized = serde_json::to_string(&tag_map).unwrap();
    std::fs::write(".tags-rs.json", serialized)?;
    Ok(())
}
