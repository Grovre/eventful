mod actions;

use actions::Actions;

fn main() {
    let mut actions = Actions::new();
    actions.add_action(|| println!("{}", 1));
    actions.add_action(|| println!("{}", "Hello World"));

    actions.invoke_actions();
}
