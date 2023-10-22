use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::path::PathBuf;

enum Template {
    Client = 1,
    UserEvent,
    Suitelet,
    Restlet,
    Scheduled,
    Portlet,
    MapReduce,
}

struct Templates {
    client: String,
    user_event: String,
    suitelet: String,
    restlet: String,
    scheduled: String,
    portlet: String,
    map_reduce: String,
}

fn main() {
    println!("Which template would you like to use?");
    println!("Enter number: Client(1), User Event(2), Suitelet(3), Restlet(4), Scheduled(5), Portlet(6), Map/Reduce(7) or Quit(0): ");

    let templates = Templates::new();
    let _template = match_templ(&templates);
    if _template == "Goodbye!" {
        return;
    }
    create_file(&_template);
}

impl Templates {
    fn new() -> Templates {
        Templates {
            client: String::from("client.js"),
            user_event: String::from("userEvent.js"),
            suitelet: String::from("suitelet.js"),
            restlet: String::from("restlet.js"),
            scheduled: String::from("scheduled.js"),
            portlet: String::from("portlet.js"),
            map_reduce: String::from("mapReduce.js"),
        }
    }
    fn get(&self, template: Template) -> &str {
        match template {
            Template::Client => return &self.client,
            Template::UserEvent => return &self.user_event,
            Template::Suitelet => return &self.suitelet,
            Template::Restlet => return &self.restlet,
            Template::Scheduled => return &self.scheduled,
            Template::Portlet => return &self.portlet,
            Template::MapReduce => return &self.map_reduce,
        };
    }
}

fn match_templ(templates: &Templates) -> String {
    let mut _template = "".to_string();
    let stdin = io::stdin();

    for tmpl in stdin.lines() {
        if let Ok(choice) = tmpl.as_ref().unwrap().parse::<i32>() {
            match choice {
                1 => _template = templates.get(Template::Client).to_string(),
                2 => _template = templates.get(Template::UserEvent).to_string(),
                3 => _template = templates.get(Template::Suitelet).to_string(),
                4 => _template = templates.get(Template::Restlet).to_string(),
                5 => _template = templates.get(Template::Scheduled).to_string(),
                6 => _template = templates.get(Template::Portlet).to_string(),
                7 => _template = templates.get(Template::MapReduce).to_string(),
                0 => _template = "Goodbye!".to_string(),
                _ => {
                    _template =
                        "To select a template, enter the associated number (1-7 or 0 to quit)"
                            .to_string()
                }
            };

            if choice <= 7 && choice > 0 {
                break;
            } else if choice == 0 {
                println!("{}", _template);
                break;
            } else {
                println!("{}", _template);
            }
        } else {
            println!("Not a valid option. Please enter a number between 0 and 7");
        }
    }
    return _template;
}

fn create_file(_template: &str) {
    println!("Enter a name for the new file (q to exit): ");

    let f = std::io::stdin().lines().next().unwrap().unwrap();

    if f == "q" {
        println!("See you next time!");
        return;
    }

    println!("Creating file: {}", f.trim());

    let sys_path = get_home_dir() + "/.config/suite_script_templates/";
    let mut _file = File::create(f.trim()).expect("Unable to create the file");
    let file_contents = fs::read_to_string(format!("{sys_path}{_template}"))
        .expect(&format!("{sys_path}{_template}"));
    _file
        .write_all(file_contents.as_bytes())
        .expect("Unable to write contents to file");

    println!(
        "File created in {}",
        get_current_working_dir().unwrap().display()
    );
}

fn get_home_dir() -> String {
    let home_dir = env::var("HOME").unwrap();
    home_dir
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

