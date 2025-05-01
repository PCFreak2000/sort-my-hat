use leptos::tachys::renderer::dom::Element;
use rand::prelude::*;
use web_sys::console::assert;

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
    let canvas: web_sys::Element = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let context: web_sys::CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut rng = rand::rng();
    let mut nums: Vec<i16> = (1..1000).collect();
    nums.shuffle(&mut rng);

    let sorted_array = insertion_sort(&nums);

    visualize_array(&sorted_array, canvas, context);
}

fn _rgb_string(r: u8, g: u8, b: u8) -> String {
    let output = format!("rgb({},{},{})", r, g, b);
    output
}

fn visualize_array(
    array: &Vec<i16>,
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
) {
    let draw_width: f64 = canvas.width() as f64 / array.len() as f64;
    let height_amplifier: f64 = canvas.height() as f64 / array.len() as f64;
    let y: f64 = canvas.height() as f64;

    for (index, integer) in array.iter().enumerate() {
        let x: f64 = index as f64 * draw_width;
        let height = *integer as f64 * height_amplifier;
        context.fill_rect(x, y, draw_width, -height);
    }
}

#[test]
fn test_insertion_sort(){
    let mut rng = rand::rng();
    let mut nums: Vec<i16> = (1..10).collect();
    let sorted_array: &mut Vec<i16> = &mut(nums.clone());
    nums.shuffle(&mut rng);
    let array: &mut Vec<i16> = &mut nums;
    insertion_sort(array);
    
    assert!(array==sorted_array);

}

fn insertion_sort(array: &Vec<i16>) -> Vec<i16> {
    let mut sorted_array = array.clone();

    for i in 0..sorted_array.len() {
        let key: i16 = sorted_array[i];
        let mut j: i16 = i as i16- 1;

        while j >= 0 && sorted_array[j as usize] > key {  

            sorted_array[(j+1) as usize] = sorted_array[j as usize];
            j = j - 1;
        }
        
        sorted_array[(j+1) as usize] = key;
    }

    sorted_array
}
