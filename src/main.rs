use show_image::create_window;

#[show_image::main]
fn main() {
    let name = "lena";

    let path = format!("samples/pgm/{name}.pgm");
    let img = image::open(path).unwrap();

    let wdw = create_window("Test", Default::default()).unwrap();

    wdw.set_image(name, img).unwrap();
    wdw.wait_until_destroyed().unwrap();
}
