use std::error::Error;
use std::fmt::Display;


#[derive(Debug)]
struct ContainsPizzaError;

impl Display for ContainsPizzaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Something went wrong: Hey, there's a pizza emoji in the text. So cheesy. Moving on to next transform")
    }
}

impl Error for ContainsPizzaError{}

#[derive(Debug)]
struct EmptyStringError;

impl Display for EmptyStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error Message: Something went wrong: The string has nothing left in it. Moving on to next transform")
    }
}

impl Error for EmptyStringError{}

trait TextTransformer {
    fn transform(&self, input_slice: &str) -> Result<String, Box<dyn Error>>;
}

#[derive(PartialEq)]
enum Case {
    Uppercase,
    Lowercase,
}

struct CaseTransformer {
    case: Case
}

impl TextTransformer for CaseTransformer {
    fn transform(&self, input_slice: &str) -> Result<String, Box<dyn Error>> {
        if self.case == Case::Lowercase {
            return Ok(input_slice.to_lowercase());
        }
        return Ok(input_slice.to_uppercase());
    }
}

struct WhitespaceTransformer {
    start: bool,
    end: bool
}

impl TextTransformer for WhitespaceTransformer {
    fn transform(&self, input_slice: &str) -> Result<String, Box<dyn Error>> {
        let mut ret_result = input_slice.to_string();
        if self.start {
            ret_result = ret_result.trim_start().to_string();
        }
        if self.end {
            ret_result = ret_result.trim_end().to_string();
        }

        if ret_result.contains("🍕") {
           return Err(Box::new(ContainsPizzaError));
        }

        if ret_result == "" {
            return Err(Box::new(EmptyStringError));
        }

        Ok(ret_result)
    }
}


fn apply_transformations(input_string: String, input_vec: Vec<Box<dyn TextTransformer>>) -> String {
    let mut ret_string = input_string;
    for transformation_struct in input_vec {
        match transformation_struct.transform(&ret_string) {
            Ok(value) => ret_string = value,
            Err(error) => println!("{}", error)
        };
    }
    return ret_string;
}

fn main() {
    // Input
    // let text = String::from("  homer simpson  ");
    // Output
    // Content: "HOMER SIMPSON"

    // Input
    // let text = String::from("  data  🍕  ");
    // Output
    // Error Message: Something went wrong: Hey, there's a pizza emoji in the text. So cheesy. Moving on to next transform
    // Content: "  DATA  🍕  "

    // Input
    let text = String::from("    ");
    // Output:
    // Error Message: Something went wrong: The string has nothing left in it. Moving on to next transform
    // Content: "    "

    let pipeline: Vec<Box<dyn TextTransformer>> = vec![
        Box::new(WhitespaceTransformer {
            start: true,
            end: true,
        }),
        Box::new(CaseTransformer {
            case: Case::Uppercase,
        }),
    ];

    let transformed_text = apply_transformations(text, pipeline);
    println!("Output: {transformed_text}");
}
