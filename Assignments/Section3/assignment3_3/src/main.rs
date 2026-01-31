// Assignment 3.3: Network Client with Error Recovery
use colored::*;

struct Connection;

fn connect(ip: &str) -> Result<Connection, String> {
    if ip == "0.0.0.0" {
        return Err("Connection refused".to_string());
    }

    Ok(Connection)
}

fn login(conn: &Connection, user: &str) -> Result<(), String> {
    if user == "admin" {
        return Ok(());
    }

    Err("Invalid credentials".to_string())
}

fn send(conn: &Connection, data: &str) -> Result<(), String> {
    if data == "" {
        return Err("Empty payload".to_string());
    }

    println!("Sent: {}", data);
    Ok(())
}

fn transaction(ip: &str, user: &str, data: &str) -> Result<(), String> {
    let conn = connect(ip)?;
    login(&conn, user)?;
    send(&conn, data)?;

    println!("{}", "Success!".green());
    Ok(())
}

fn main() {
    // Attempt 1: Valid arguments
    let error = transaction("192.168.1.1", "admin", "Helloooo!");

    match error {
        Ok(c) => {}
        Err(msg) => println!("{}", msg.red()),
    }

    // Attempt 2: Invalid IP address
    let error = transaction("0.0.0.0", "admin", "Helloooo!");

    match error {
        Ok(c) => {}
        Err(msg) => println!("{}", msg.red()),
    }

    // Attempt 3: Invalid user
    let error = transaction("192.168.1.1", "Ezra", "Helloooo!");

    match error {
        Ok(c) => {}
        Err(msg) => println!("{}", msg.red()),
    }
}
