use colored::Colorize;

use crate::mods::printer::*;

// Menu
// Start Dev Container
// Start Prod Container
// Push to AWS ECR
// PUSH to DigitalOcean CR
// Exit
//
pub fn main_menu() {
    plf(&"-".yellow());
    plnc(&"Welcome to the Docker Deployment Tool".bold());
    plf(&"-".yellow());

    plm(&"(1) Build & Start Dev Container".green().bold());
    plm(&"(2) Build & Start Prod Container".green().bold());
    plm(&"(3) Push Docker Image to AWS ECR".green().bold());
    plm(&"(4) Push Docker Image to DigitalOcean CR".green().bold());
    plm(&"(5) Exit".green().bold());

    plf(&"-".yellow());
    pln(&"Please choose an option: ".bold());
}

pub fn exit_confirm() {
    plf(&"-".yellow());
    plnc(&"Are you sure you want to exit? (y/n)".bold());
    plf(&"-".yellow());
}
