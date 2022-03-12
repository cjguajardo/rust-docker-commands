use crate::mods::printer::plnc;
use colored::*;

pub fn start_dev() {
    plnc(&"BUILD&RUN DEVELOPMENT DOCKER IMAGE.".yellow().bold());
    println!("START DEV");
}
