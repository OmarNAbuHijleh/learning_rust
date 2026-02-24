mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";

    pub fn ask_about_program() {
        println!("The nutritionist for this program is {}", NUTRITIONIST);
    }
}

pub mod cardio;
pub mod weightlifting;

use crate::weightlifting::Exercise as WeightliftingExercise;
use crate::cardio::{CardioTool, Exercise as CardioExercise};

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new() -> GymWorkout {
        weightlifting::ask_about_program();
        cardio::ask_about_program();
        diet::ask_about_program();
        GymWorkout {
            cardio: CardioExercise::new(
                String::from("Monday"),
                CardioTool::Treadmill,
                30
            ),
            weightlifting: WeightliftingExercise::new(String::from("Bench"), 10),
        }
    }
}
