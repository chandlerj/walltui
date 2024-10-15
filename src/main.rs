use viuer::{print_from_file, Config};

fn main() {
    let conf = Config {
        // Set offset.
        x: 20,
        y: 4,
        // Set dimensions.
        width: Some(80),
        height: Some(25),
        ..Default::default()
    };

    // Starting from row 4 and column 20,
    // display `img.jpg` with dimensions 80×25 (in terminal cells).
    // Note that the actual resolution in the terminal will be 80×50.
    print_from_file("/home/chandler/Pictures/papes/waffles.jpg", &conf).expect("Image printing failed.");
}
