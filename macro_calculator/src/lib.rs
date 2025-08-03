use json;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut total_calories = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;
    let mut total_carbs = 0.0;

    for food in foods {
        let kcal_str = food.calories.1.clone();
        let kcal_value: f64 = kcal_str
            .trim_end_matches("kcal")
            .parse()
            .unwrap_or(0.0);

        total_calories += kcal_value * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
    }

    total_calories = round_to_decimal(total_calories);
    total_proteins = round_to_decimal(total_proteins);
    total_fats = round_to_decimal(total_fats);
    total_carbs = round_to_decimal(total_carbs);

    json::object! {
        "cals": total_calories,
        "carbs": total_carbs,
        "proteins": total_proteins,
        "fats": total_fats,
    }
}

fn round_to_decimal(value: f64) -> f64 {
    let rounded = (value * 100.0).round() / 100.0;
    
    if (rounded * 10.0).round() / 10.0 == rounded {
        (rounded * 10.0).round() / 10.0
    } else {
        rounded
    }
}
