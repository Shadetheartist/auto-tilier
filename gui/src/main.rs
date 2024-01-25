use gtk::prelude::*;
use gtk::{gdk, glib, Grid, Image, Paned, Window, WindowType};
use gdk_pixbuf::{Pixbuf, Colorspace};
use gtk::glib::Value;
use gtk::Orientation::{Horizontal, Vertical};
use image::{ImageBuffer, Rgba};
use autotiler::grid::RectVec;
use autotiler::point::Point;
use autotiler::tile::Tile3x3;

fn main() {
    // Initialize GTK.
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new GTK window.
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Test Window");
    window.set_default_size(1024, 1024);

    // Connect a key press event handler.
    window.connect("key-press-event", true, |args| {
        let raw_event = &args[1].get::<gdk::Event>().unwrap();
        match raw_event.downcast_ref::<gdk::EventKey>() {
            None => {}
            Some(event) => {
                println!("key value: {:?}", event.keyval());
                println!("modifiers: {:?}", event.state());
            }
        }
        Some(Value::from(true))
    });

    let tile_set = autotiler::tile::minimal_3x3_tile_set();

    let tileset_widget = minimal_3x3_tileset_widget(&tile_set);

    let top = Paned::new(Vertical);
    top.pack1(&tileset_widget, true, false);

    let bot = Paned::new(Horizontal);

    let test_grid = autotiler::grid::generate_test_grid(&tile_set, 8, 8);
    let grid_widget = tile_grid_widget(&test_grid, 27 * 2);
    let gtk_box = gtk::Box::builder().margin(16).build();
    gtk_box.add(&grid_widget);
    bot.pack1(&gtk_box, true, false);

    let stripped = autotiler::grid::grid_strip_invalid(&test_grid);
    let grid_widget = tile_grid_widget(&stripped, 27 * 2);
    let gtk_box = gtk::Box::builder().margin(16).build();
    gtk_box.add(&grid_widget);
    bot.pack2(&gtk_box, true, false);

    top.pack2(&bot, true, false);

    window.add(&top);

    window.show_all();

    // Start the GTK main loop.
    gtk::main();
}

fn tile_grid_widget(tile_grid: &RectVec, size: u32) -> Grid {
    let grid = Grid::builder().build();


    for (pos, tile) in tile_grid.iter_enumerate() {
        // Generate an image.
        //let img = grid_image_buffer(1024, 1024, 64, 3);
        let img = render_tile(tile, size, size);

        // Convert the image to a GdkPixbuf.
        let pixel_buffer = image_to_pixbuf(&img);

        // Create a GTK image widget from the file.
        let image = Image::from_pixbuf(Some(&pixel_buffer));
        image.set_margin(1);

        // Add the image to the window and show everything.
        grid.attach(&image, pos.x, pos.y, 1, 1);
    }

    grid
}

fn minimal_3x3_tileset_widget(tile_set: &Vec<Tile3x3>) -> Grid {
    let grid = Grid::builder().build();

    for (idx, tile) in tile_set.iter().enumerate() {
        // Generate an image.
        //let img = grid_image_buffer(1024, 1024, 64, 3);
        let img = render_tile(tile, 64, 64);

        // Convert the image to a GdkPixbuf.
        let pixel_buffer = image_to_pixbuf(&img);

        // Create a GTK image widget from the file.
        let image = Image::from_pixbuf(Some(&pixel_buffer));

        let row = idx / 12;
        let col = idx % 12;

        // Add the image to the window and show everything.
        grid.attach(&image, col as i32, row as i32, 1, 1);
    }

    grid
}

// Function to convert an image to a GdkPixbuf.
fn image_to_pixbuf(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Pixbuf {
    let (width, height) = img.dimensions();
    let row_stride = 4 * width as i32;
    Pixbuf::from_bytes(
        &glib::Bytes::from(&img.clone().into_raw()),
        Colorspace::Rgb,
        true,
        8,
        width as i32,
        height as i32,
        row_stride,
    )
}

// Function to generate an image.
pub fn grid_image_buffer(width: u32, height: u32, grid_size: u32, grid_stroke_width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    ImageBuffer::from_fn(width, height, |x, y| {
        if x % grid_size < grid_stroke_width || y % grid_size < grid_stroke_width {
            Rgba([0, 0, 0, 128])
        } else {
            Rgba([0, 0, 0, 0])
        }
    })
}


pub fn render_tile(tile: &Tile3x3, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width_coef = 3.0 / width as f32;
    let height_coef = 3.0 / height as f32;

    ImageBuffer::from_fn(width, height, |x, y| {
        let sample_x = (x as f32 * width_coef) as u8;
        let sample_y = (y as f32 * height_coef) as u8;
        let tile_sample = tile.get(&Point{x: sample_x as i32, y: sample_y as i32});
        let pix = tile_sample as u8 * 255;

        Rgba([pix, 0, 0, 255])
    })
}