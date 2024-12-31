use clap::Parser;
use anyhow::{Context, Result};
/// Search for a pattern in a file and display that line
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern:String,
    /// The path to the file to read
    path:std::path::PathBuf,
}
fn main() -> Result<()> {
   
   let args = Cli::parse();
   // rewrite using BufReader yes
   let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {}",args.path.display()))?;
   for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line);
    }
   }
   Ok(())
}
// use std::{thread, time::Duration};
// fn main() {
//     let bar = indicatif::ProgressBar::new(1000);
//     for _i in 0..1001{
//         thread::sleep(Duration::from_millis(10));
//         bar.inc(1);
//     }
//     bar.finish()
// }