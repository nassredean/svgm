use std::env;
use std::fs;
use std::path::Path;
use svg::Document;
use svg::node::element::Rectangle;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} <output_file_name> <directory_path> <rows> <columns>", args[0]);
        std::process::exit(1);
    }

    let output_file_name = &args[1];
    let directory_path = &args[2];
    let rows: u32 = args[3].parse().expect("Invalid number of rows");
    let columns: u32 = args[4].parse().expect("Invalid number of columns");

    let file_count = count_files_in_directory(directory_path).unwrap_or_else(|err| {
        eprintln!("Error counting files: {}", err);
        std::process::exit(1);
    });

    println!("Number of files in {}: {}", directory_path, file_count);

    let width = 279.4;
    let height = 431.8;

    let cm_to_pixel = 96.0 / 2.54; // 1 inch = 2.54 cm, 1 inch = 96 pixels
    let width_px = width * cm_to_pixel;
    let height_px = height * cm_to_pixel;

    let column_width = width_px / columns as f64;
    let row_height = height_px / rows as f64;

    println!("Column width: {} pixels", column_width);
    println!("Row height: {} pixels", row_height);

    let background = Rectangle::new()
        .set("width", width_px)
        .set("height", height_px)
        .set("fill", "white");

    let document = Document::new()
        .set("viewBox", (0, 0, width_px, height_px))
        .add(background);

    svg::save(output_file_name, &document).expect("Failed to save SVG file");
}

fn count_files_in_directory<P: AsRef<Path>>(path: P) -> Result<usize, std::io::Error> {
    let entries = fs::read_dir(path)?;
    let mut file_count = 0;

    for entry in entries {
        let entry = entry?;
        if entry.metadata()?.is_file() {
            file_count += 1;
        }
    }

    Ok(file_count)
}

