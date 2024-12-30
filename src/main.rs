use clap::Parser;
/// Search for a pattern in a file and display that line
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern:String,
    /// The path to the file to read
    path:std::path::PathBuf,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
   
   let args = Cli::parse();
   // rewrite using BufReader yes
   let content = std::fs::read_to_string(&args.path)?;
   for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line);
    }
   }
   Ok(())
}
