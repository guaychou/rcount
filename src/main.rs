use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt, Debug)]
#[structopt(name = "RCount tool")]
struct Options {
    /// Directory path
    #[structopt(short = "d", long = "dir", default_value = ".")]
    dir: String,
    /// Recursive count or not
    #[structopt(short = "r", long = "recursive")]
    recursive: bool,
}

fn main() {
    let options = Options::from_args();
    let mut walker = WalkDir::new(options.dir);
    let mut count = 0;
    if !options.recursive {
        walker = walker.max_depth(1);
    }
    for _ in walker.into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        count += 1;
    }
    println!("Total: {}", count)
}
