use clap::Parser;
/// Search for a pattern in a file and display that line
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern:String,
    /// The path to the file to read
    path:std::path::PathBuf,
}
fn main() {
   
   let args = Cli::parse();
   // rewrite using BufReader yes
   let content = std::fs::read_to_string(&args.path)?;
//    let content = match result{
//     Ok(content) => { content },
//     Err(error) => {panic!("Cant deal with {}, just exit here",error);}
//    };

   for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line);
    }
   }
}
