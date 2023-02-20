mod actions;

use actions::Actions;

fn main() {
    let mut fizzbuzz_actions = Actions::new();
    let mut processed_actions = Actions::new();

    // Need to add states
    fizzbuzz_actions.add_action(Box::new(|| println!("We found a fizzbuzz!")));
    processed_actions.add_action(Box::new(|| println!("Finished processing another number")));

    for i in 0..100 {
        if i % 15 == 0 {
            fizzbuzz_actions.invoke_actions();
        } else {
            processed_actions.invoke_actions();
        }
    }
}
