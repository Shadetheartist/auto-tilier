use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, DrawingArea, gdk, Paned};
use gtk::glib::Propagation::Proceed;
use gtk::glib::Value;
use gtk::Orientation::{Horizontal};
use autotiler::matrix::Matrix;
use autotiler::point::Point;

fn main() {
    let application = Application::new(Some("com.example.MouseEvents"), Default::default());

    application.connect_activate(|app| {
        // Create a new GTK window.
        let window = ApplicationWindow::new(app);
        window.set_title("Test Window");
        window.set_default_size(100, 100);

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

        let bot = Paned::new(Horizontal);

        let random_matrix = autotiler::matrix::generate_random_matrix(&tile_set, 8, 8);
        let test_matrix = random_matrix.strip_invalid();
        let grid_widget = tile_matrix_widget(test_matrix, 27 * 3);
        let gtk_box = gtk::Box::builder().margin(16).build();
        gtk_box.add(&grid_widget);
        bot.pack1(&gtk_box, true, false);

        let button = Button::with_label("Click Me");

        // Connect to mouse button press event for the button
        button.connect_button_press_event(|_, event| {
            println!("Button pressed! Event type: {:?}", event.event_type());
            Proceed
        });

        bot.pack2(&button, true, false);

        window.add(&bot);

        // CSS for hover effect
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_data(
            b".hover>image {  border: 1px solid white; margin: 0px;} image {margin: 1px;}",
        ).expect("Failed to load CSS");

        // Apply CSS
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error initializing gtk css provider."),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );

        window.show_all();
    });


    application.run();
}

struct TileWidgetState {
    hovered_pos: Point,
    active_pos: Option<Point>,
    pathing_pos: Option<Point>,
    active_button: u32,
    matrix: Matrix,
}

fn tile_matrix_widget(matrix: Matrix, size: u32) -> DrawingArea {
    let drawing_area = DrawingArea::new();

    let size = size as f64;
    let px_coefficient = size / 3.0;
    drawing_area.set_size_request(matrix.px_bounds.w * px_coefficient as i32, matrix.px_bounds.h * px_coefficient as i32);

    let state = Rc::new(RefCell::new(TileWidgetState {
        hovered_pos: Point { x: 0, y: 0 },
        active_pos: None,
        pathing_pos: None,
        active_button: 0,
        matrix: matrix,
    }));


    pub fn pt(idx: usize) -> Point {
        let x = idx as i32 % 3;
        let y = idx as i32 / 3;
        Point { x, y }
    }

    {
        let state = Rc::clone(&state);
        drawing_area.connect_draw(move |_, ctx| {
            let state = state.borrow_mut();

            for (pos, tile) in state.matrix.iter_tiles_enumerate() {
                // Generate an image.
                //let img = grid_image_buffer(1024, 1024, 64, 3);

                let px_offset_x = pos.x as f64 * size;
                let px_offset_y = pos.y as f64 * size;

                for (i, px) in tile.iter().enumerate() {
                    let hovered = state.hovered_pos.eq(&pos);
                    let is_active = state.active_pos.is_some() && state.active_pos.unwrap().eq(&pos);
                    let is_pathing = state.pathing_pos.is_some() && state.pathing_pos.unwrap().eq(&pos);

                    if is_active  {
                        ctx.set_source_rgba(
                            if hovered { 0.3 } else { 0.0 },
                            if hovered { 0.3 } else { 0.0 },
                            if *px { 1.0 } else { if hovered { 0.3 } else { 0.0 } },
                            1.0,
                        );
                    } else {
                        if is_pathing {
                            ctx.set_source_rgba(
                                if hovered { 0.3 } else { 0.0 },
                                if hovered { 0.3 } else { 0.0 },
                                if *px { 1.0 } else { if hovered { 0.3 } else { 0.0 } },
                                1.0,
                            );
                        } else {
                            ctx.set_source_rgba(
                                if *px { 1.0 } else { if hovered { 0.3 } else { 0.0 } },
                                if hovered { 0.3 } else { 0.0 },
                                if hovered { 0.3 } else { 0.0 },
                                1.0,
                            );
                        }
                    }

                    let p = pt(i);

                    ctx.rectangle(
                        px_offset_x + p.x as f64 * px_coefficient,
                        px_offset_y + p.y as f64 * px_coefficient,
                        px_coefficient,
                        px_coefficient,
                    );

                    ctx.fill().unwrap();
                }
            }

            Proceed
        });
    }

    {
        drawing_area.add_events(gdk::EventMask::POINTER_MOTION_MASK);

        let state = Rc::clone(&state);
        drawing_area.connect_motion_notify_event(move |_widget, event| {
            let mut state = state.borrow_mut();

            let x = (event.position().0 / size) as i32;
            let y = (event.position().1 / size) as i32;

            if x != state.hovered_pos.x || y != state.hovered_pos.y {
                println!("Mouse move position: ({}, {})", x, y);

                _widget.queue_draw();
                state.hovered_pos.x = x;
                state.hovered_pos.y = y;

                if let Some(active_pos) = state.active_pos {
                    if x != active_pos.x || y != active_pos.y {
                        if (active_pos.x - x).abs() <= 1 && y == active_pos.y || (active_pos.y - y).abs() <= 1 && x == active_pos.x {
                            state.pathing_pos = Some(state.hovered_pos);
                        }
                    }
                }
            }

            Proceed
        });
    }

    {
        let state = Rc::clone(&state);
        drawing_area.add_events(gdk::EventMask::BUTTON_PRESS_MASK);

        drawing_area.connect_button_press_event(move |_widget, event| {
            let mut state = state.borrow_mut();

            let x = (event.position().0 / size) as i32;
            let y = (event.position().1 / size) as i32;

            if state.active_button != event.button() {
                _widget.queue_draw();
                state.active_pos = Some(state.hovered_pos.clone());
                state.active_button = event.button();
            }

            println!("Mouse press position: ({}, {})", x, y);

            Proceed
        });
    }

    {
        let state = Rc::clone(&state);
        drawing_area.add_events(gdk::EventMask::BUTTON_RELEASE_MASK);

        drawing_area.connect_button_release_event(move |_widget, event| {

            let x = (event.position().0 / size) as i32;
            let y = (event.position().1 / size) as i32;

            println!("Mouse release position: ({}, {})", x, y);

            let mut state = state.borrow_mut();

            if state.pathing_pos.is_some() && state.active_pos.is_some() {
                let from = state.active_pos.unwrap();
                let to = state.pathing_pos.unwrap();
                if state.active_button == 1 {
                    state.matrix.path(&from, &to);
                } else if state.active_button == 3 {
                    state.matrix.erase_path(&from, &to);
                    state.matrix = state.matrix.strip_invalid();
                }
            } else if state.active_pos.is_some() {
                let pt = state.active_pos.unwrap();
                if state.active_button == 1 {
                    state.matrix.fill(&pt);
                    state.matrix = state.matrix.strip_invalid();
                } else if state.active_button == 3 {
                    state.matrix.erase(&pt);
                    state.matrix = state.matrix.strip_invalid();
                }
            }

            state.pathing_pos = None;
            state.active_pos = None;
            state.active_button = 0;

            _widget.queue_draw();

            Proceed
        });
    }


    drawing_area
}
