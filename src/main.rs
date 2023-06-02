fn main() {
    let file_string =
        std::fs::read_to_string("./src/input.txt").expect("Couldn't read string from file");

    let mut highest_calories = 0;
    let mut calories = 0;

    for line in file_string.lines() {
        if line.is_empty() {
            if calories > highest_calories {
                highest_calories = calories;
            }

            println!("Calories: {}", calories);
            calories = 0;
        } else {
            let line_calories = line.parse::<i32>().expect(
                format!("Error trying to parse line with content '{}' to int", line).as_str(),
            );

            calories += line_calories;
        }
    }

    println!("Highest calories: {}", highest_calories);
}
