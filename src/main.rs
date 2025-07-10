use std::{env, path::PathBuf};
use clap::Parser;
use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
struct Movement {
    target: PathBuf,
    destination: PathBuf
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// target dirrectory
    #[arg(short, long)]
    target: Option<PathBuf>,

    /// output dirrectory
    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long, action)]
    destructive: bool
}


fn main() -> Result<()> {
    let args = Args::parse();

    let working_dir = env::current_dir()?;

    let target_dir = match args.target {
        Some(c) => {
            if c.is_absolute() {
                c
            } else {
                working_dir.join(c)
            }
        },
        None => working_dir.clone(),
    };

    let config: Vec<Movement> = serde_json::from_str(fs::read_to_string(&target_dir.join("t-copy.json")).expect("t-copy.json not found").as_str())?;


    let output_dir = match args.output.is_absolute() {
        true => args.output,
        false => working_dir.join(args.output),
    };

    // print!("{:?}", working_dir.join(config.get(0).unwrap().destination.clone()));

    for i in config {

        let tpath = target_dir.join(i.target.clone());
        let dpath = output_dir.join(i.destination.clone());

        println!("Copying {:?} to {:?}", tpath, dpath);

        if !tpath.exists() {
            eprintln!("{} not found", tpath.to_str().expect("non unicode paths not supported in errors"));
            continue;
        }

        if tpath.is_dir() {
            if !dpath.exists() || args.destructive {
                let _ = fs_extra::dir::create_all(dpath.clone(), args.destructive)?;
            }

            let mut options = fs_extra::dir::CopyOptions::new();
            options.content_only = true;
            options.overwrite = true;
            let _ = fs_extra::dir::copy(tpath, dpath, &options)?;
            continue;
        }

        if tpath.is_file() {
            let mut testpath = dpath.clone();
            testpath.pop();
            if !testpath.exists() || args.destructive {
                let _ = fs_extra::dir::create_all(testpath, args.destructive)?;
            }

            let options = fs_extra::file::CopyOptions::new().overwrite(true);
            let _ = fs_extra::file::copy(tpath, dpath, &options)?;
            continue;
        }

    }
    
    Ok(())
}
