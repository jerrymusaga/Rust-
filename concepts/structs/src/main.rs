
struct GrayscalePic {
    pixels: Vec<u8>,
    size: (u32,u32)
}

fn main() {

    let w = 250;
    let h = 450;
    let image = GrayscalePic {
        pixels: vec![3,5,6],
        size: (4, w+h)
    };
    println!("{:?}, {:?}",image.pixels, image.size);
}
