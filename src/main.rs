use rand::prelude::*;

fn main() {
    // //generate a vec with about a thousand numbers
    // let mut rng = rand::rng();
    // // Generate and shuffle a sequence:
    // let mut nums: Vec<i32> = (1..1000).collect();
    // nums.shuffle(&mut rng);

    // //convert nums to string
    // let mut output_string=String::new();
    // for num in nums{
    //     //trying to limit the amount of characters on a single line to a max of 80
    //     if output_string.len() % 80 < 5 {
    //         output_string.push_str("\n");
    //     }
    //     output_string.push_str(&format!("{num} "));

    // };

    // println!("{}", output_string);

    start();
}

use std::f64;
use wasm_bindgen::prelude::*;

//https://www.w3schools.com/jsref/api_canvas.asp
#[wasm_bindgen(start)]
fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.move_to(50.0, 50.0);
    context.set_fill_style_str("rgb( 250,50,100)");

    context.fill_rect(50., 50., 50., 20.);

    context.move_to(0., 0.);
    context.line_to(500., 500.);

    context.stroke();
}

fn rgb_string(r: u8, g: u8, b: u8) -> String {
    let output = format!("rgb({},{},{})", r, g, b);
    output
}
