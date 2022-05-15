use std::{
    env,
    fs::{self, File},
    io::Write,
};

mod helpers {
    pub fn capitalize(str: &String) -> String {
        str.chars().nth(0).unwrap().to_uppercase().to_string() + &str[1..]
    }
}

fn update_mod(dir: &str, model: &str) -> std::io::Result<()> {
    let mut file = File::options()
        .append(true)
        .open(dir.to_string() + "/mod.rs")?;

    let msg = format!("pub mod {};", model);

    file.write_all(msg.as_bytes())
}

fn replace(
    file_path: &str,
    dest_path: &str,
    replace_with: &str,
    replace_with_2: &str,
) -> std::io::Result<()> {
    let contents = fs::read_to_string(file_path).expect("Could not read file");

    let new_contents = contents
        .replace("$1", replace_with)
        .replace("$c1", &helpers::capitalize(&replace_with.to_string()))
        .replace("$2", replace_with_2);

    fs::write(dest_path, new_contents)
}

/// Example: model: "status", table: "statuses"
fn all(dir: &str, model: &str, table: &str) -> std::io::Result<()> {
    for i in ["api", "ops", "model"] {
        replace(
            (dir.to_string() + "/" + i + ".txt").as_str(),
            &format!("src/{}/{}.rs", i, model),
            model,
            table,
        )?;
        update_mod(("src/".to_string() + i).as_str(), model)?;
    }

    println!("Generated. Include this service in your Actix service handler: `.service(api::{}::service())`", model);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let templates_path = match env::var("TEMPLATES_PATH") {
        Ok(val) => val,
        Err(_) => "/home/jimsu/code/rust/actix-gen/templates".to_string(),
    };

    if args.len() == 1 {
        panic!("Must specify subcommand.");
    }

    let command = &args[1];
    let arg = &args[2];
    let arg2 = &args[3];
    let arg3 = match args.len() > 4 {
        true => &args[4],
        false => "",
    };

    match command.as_str() {
        "all" => all(&templates_path, arg, arg2),
        val @ "api" | val @ "ops" | val @ "model" => {
            replace(&format!("{}/{}.rs", templates_path, val), arg, arg2, arg3)
        }
        _ => {
            panic!("Command not recognized.")
        }
    }
}
