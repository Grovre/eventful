mod actions;

use actions::Actions;

fn main() {
    let mut actions = Actions::new();
    let hello_world_fn_key = actions.add_action(|| println!("Hello world!"));
    for _i in 0..10 {
        actions.add_action(|| println!("{}", random_str::get_string(5, true, true, true, false)));
    }

    actions.invoke_actions();
    println!();
    let hello_world_fn = actions.remove_action(&hello_world_fn_key).unwrap();
    actions.invoke_actions();

    println!("{}", actions);
}
