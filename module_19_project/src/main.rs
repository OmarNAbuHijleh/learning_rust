// Needs lifetime annotations - multiple input references and returns a reference
fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &'a str) -> &'a str {
    if first.contains(target) {
        return first;
    } else if second.contains(target) {
        return second;
    }
    ""
}

// This function requires a lifetime specifier because we return a reference and take in multiple references
fn first_five<'a>(text: &'a str, announcement: &'a str) -> &'a str {
    println!("{}", announcement);
    &text[..5]
}

// This function does not need lifetime annotations because we only offer one reference as an input and we do not return any references
fn double_the_length<T>(input: &Vec<T>) -> usize {
    input.len() * 2
}

// This function does not need lifetime annotations because there is only 1 lifetime input and output, so they are assumed to be the same lifetime
fn last_two<T>(input_slice: &[T]) -> &[T] {
    // return the last two elements of the slice as a slice
    let slice_length = input_slice.len();
    &input_slice[slice_length-2_usize..]
}

fn main() {
    let input_vector = vec![1,2,3];
    println!("{}", double_the_length(&input_vector));

    println!("{:?}", last_two(&input_vector));

    let text = "refrigerator";
    let announcement = "Hello";

    println!("{}", first_five(text, announcement));

    let first = "programming";
    let second = "dining";
    let target = "gram";
    println!("{}", find_string_that_has_content(first, second, target))
}
