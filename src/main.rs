use std::env;
use std::fs::File;
use std::io::BufWriter;

fn print_image_info(image: &rawloader::RawImage) {
    println!("make        : {}", image.make);
    println!("model       : {}", image.model);
    println!("clean_make  : {}", image.clean_make);
    println!("clean_model : {}", image.clean_model);
    println!("width       : {}", image.width);
    println!("height      : {}", image.height);
    println!("cpp         : {}", image.cpp);
    println!("whitelevels : {:?}", image.whitelevels);
    println!("blacklevels : {:?}", image.blacklevels);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(2);
    }
    let file = &args[1];
    let image = rawloader::decode_file(file).unwrap();

    print_image_info(&image);

    let output_path = format!("{file}.png");
    let path = std::path::Path::new(&output_path);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(
        w,
        u32::try_from(image.width).unwrap(),
        u32::try_from(image.height).unwrap(),
    );
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Sixteen);
    let mut writer = encoder.write_header().unwrap();

    let rawloader::RawImageData::Integer(img_data) = image.data else {
        panic!("Not integer.");
    };

    let byte_data = img_data.into_iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>();
    writer.write_image_data(&byte_data).unwrap(); // Save

}
