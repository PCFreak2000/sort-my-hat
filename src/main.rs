use rand::prelude::*;

fn main() {
    
    //generate a vec with about a thousand numbers
    let mut rng = rand::rng();
    // Generate and shuffle a sequence:
    let mut nums: Vec<i32> = (1..1000).collect();
    nums.shuffle(&mut rng);

    //convert nums to string
    let mut output_string=String::new();
    for num in nums{
        //trying to limit the amount of characters on a single line to a max of 80
        if output_string.len() % 80 < 5 {
            output_string.push_str("\n");
        }
        output_string.push_str(&format!("{num} "));
        
    };

    println!("{}", output_string);
    
}



