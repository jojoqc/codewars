/*
You're writing code to control your town's traffic lights.
You need a function to handle each change from green,
to yellow, to red, and then to green again.
Complete the function that takes a string as an argument
representing the current state of the light and returns a string
representing the state the light should change to.
For example, when the input is green, output should be yellow.
*/

fn update_light(current: &str) -> String {
    match current.to_lowercase().as_str() {
        "green" => "yellow".to_string(),
        "yellow" => "red".to_string(),
        "red" => "green".to_string(),
        _ => panic!("No more light"),
    }
}

#[test]
fn basic_test() {
    assert_eq!(update_light("green"), "yellow");
    assert_eq!(update_light("yellow"), "red");
    assert_eq!(update_light("red"), "green");
}
