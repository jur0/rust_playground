
extern crate envy;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Environment {
    // List of env variables we are interested in.
    // Envy normalises the names to lowercase.
    lang: String,
}

fn main() {
    match envy::from_env::<Environment>() {
        Ok(env) => println!("{:?}", env),
        _ => println!("No environment var!"),
    }
}
