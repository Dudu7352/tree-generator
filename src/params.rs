use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author = "Dudu7352",
    version,
    about = "Simple tree generator that outputs svg files"
)]
pub struct Params {
    #[arg(short, long, default_value_t = 1.0)]
    /// describes the maximum angle in which new branches may diverge
    pub max_branch_angle: f32,
    #[arg(short, long, default_value_t = 1)]
    /// number of SVG files to produce during the growth
    pub images: u16,
    #[arg(long, default_value_t = String::from("tree"))]
    /// prefix of the file name ("little_tree" will produce files such as little_tree_0.svg etc.)
    pub image_prefix: String,
    /// time to pass in a simulation (200-300 range works the best)
    pub time: f32,
}

