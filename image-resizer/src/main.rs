use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use image::{open, DynamicImage};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Resize an image
    Resize {
        /// Path to the input image
        input: PathBuf,

        /// New width for the image
        width: u32,

        /// New height for the image
        height: u32,

        /// Path to save the resized image
        #[arg(long)]
        output: PathBuf,
    },

    /// Display information about an image
    Info {
        /// Path to the input image
        #[clap(value_parser)]
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Resize {
            input,
            width,
            height,
            output,
        } => {
            let result = resize_image(input, width, height)?;
            result.save(output)?;
            println!("Resized image size: {}x{}", result.width(), result.height());
        }

        Commands::Info { input } => {
            let img = open(input)?;
            println!("Image dimensions: {}x{}", img.width(), img.height());
            println!("Image color type: {:?}", img.color());
        }
    }

    Ok(())
}

fn resize_image(
    image_path: impl AsRef<std::path::Path>,
    width: u32,
    height: u32,
) -> Result<DynamicImage> {
    let img = open(image_path)?;
    Ok(img.resize(width, height, image::imageops::FilterType::Gaussian))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_resize_image() {
        let image_path = Path::new("test_image.jpeg");

        let initial_width = 1080;
        let initial_height = 1005;
        let test_image = image::open(&image_path).expect("Failed to open test image");

        assert_eq!(test_image.width(), initial_width);
        assert_eq!(test_image.height(), initial_height);

        let new_width = 800;
        let new_height = 600;

        let resized_image = resize_image(&image_path, new_width, new_height).unwrap();
        assert!(resized_image.width() <= new_height || resized_image.height() <= new_height);
    }
}
