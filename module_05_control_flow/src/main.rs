fn main() {
    /*
        The if statement


    */

    let var: i32 = 3;
    if var==3{
        println!("Variable is {var}=3");
    }


    let season_var: &str = "summer"; // Change this as desired
    fun_func(season_var);

    /*
        Assigning the result of an "if" statement to a variable
     */

    let is_summer: bool = if season_var=="summer" {true} else {false};
    println!("{is_summer}"); 


    /*
        The match statement
    */
}

fn fun_func(input_var: &str) {
    if input_var == "summer" {
        println!("It is summer. Beach Fun");
    } else if input_var == "winter" {
        println!("Brrr");
    } else if input_var == "spring" {
        println!("Green Grass!");
    } else {
        println!("Hope you had a great fall!");
    }
}
