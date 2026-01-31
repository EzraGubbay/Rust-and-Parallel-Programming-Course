// Assignment 3.2: Project Template Generator (with Dependencies)
use colored::*;
use std::fs;

fn main() {
    println!("Creating project...");

    fs::create_dir("dummy_project").unwrap();
    fs::File::create("README.md").unwrap();

    println!("{}", "Project created successfully!".green());
}
