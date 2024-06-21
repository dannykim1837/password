mod pentry;

use crate::pentry::{delete_entry, prompt, read_passwords_from_file, ServiceInfo};

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    loop {
        println!("Password Manager Menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search");
        println!("4. Delete Entry");
        println!("5. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service :"),
                    prompt("Username :"),
                    prompt("Password :"),
                );
                println!("Entry added successfully.");
                entry.write_to_file();
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                if services.is_empty() {
                    println!("The list is empty. No entries found.");
                } else {
                    for item in &services {
                        println!(
                            "Service = {}
                            - Username : {}
                            - Password : {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                if services.is_empty() {
                    println!("The list is empty. No entries to search.");
                } else {
                    let search = prompt("Search :");
                    let mut found = false;
                    for item in &services {
                        if item.service.as_str() == search.as_str() {
                            println!(
                                "Service = {}
                                - Username : {}
                                - Password : {}",
                                item.service, item.username, item.password
                            );
                            found = true;
                        }
                    }
                    if !found {
                        println!("No entries found matching '{}'.", search);
                    }
                }
            }
            "4" => {
                clr();
                let deletion_service = prompt("Enter the service name to delete: ");
                if let Err(e) = delete_entry(&deletion_service) {
                    eprintln!("Error deleting entry: {}", e);
                } else {
                    println!("Entry deleted successfully.");
                }
            }
            "5" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
        println!("\n\n");
    }
}
