use colored::*;
use text_io::read;
//use crossterm::{execute, Result, terminal::{size}}
mod mods;
use crate::mods::menu::{exit_confirm, main_menu};
use crate::mods::printer::{pln, plnc};
use crate::mods::start::{aws::*, development::*, docr::*, production::*};

fn main() {
    pln(&" ".bold());
    plnc(&"Simple Dev&Deploy".yellow().bold());

    main_menu();

    let line: String = read!("{}\n");
    let option = line.trim() as &str;

    match option {
        "1" => {
            start_dev();
        }
        "2" => {
            start_prod();
        }
        "3" => {
            push_to_aws();
        }
        "4" => {
            push_to_docr();
        }
        "5" => {
            exit_confirm();
        }
        _ => {
            pln(&"Invalid option".red());
            main();
        }
    }
}
