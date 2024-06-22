use anyhow::Result;
use image::open;
use std::env;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <IMAGE_PATH> <WIDTH> <HEIGHT>", args[0]);
        std::process::exit(1);
    }

    let image_path = PathBuf::from(&args[1]);
    let width: u32 = args[2]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid width"))?;
    let height: u32 = args[3]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid height"))?;

    let result = resize_image(&image_path, width, height)?;

    println!("Resized image size: {}x{}", result.width(), result.height());

    Ok(())
}

fn resize_image(
    image_path: impl AsRef<Path>,
    width: u32,
    height: u32,
) -> Result<image::DynamicImage> {
    let img = open(image_path).expect("Failed to open image");
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
