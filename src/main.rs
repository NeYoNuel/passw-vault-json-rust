mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;
use crate::pentry::ServiceInfo;

fn clean_al() {
    println!("{}[2J", 27 as char);

}

fn main() {
    clean_al();
    let ascii = r"
    \  |              _  \                                   \ \     /              |  |   
    |\/ |  |   |      |   |  _` |   __|   __| \ \  \   /      \ \   /  _` |  |   |  |  __| 
    |   |  |   |      ___/  (   | \__ \ \__ \  \ \  \ /        \ \ /  (   |  |   |  |  |   
   _|  _| \__, |     _|    \__,_| ____/ ____/   \_/\_/          \_/  \__,_| \__,_| _| \__| 
          ____/                                                                            
    ";

    println!("{ascii}");

    loop {
        println!("Password manager menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search Entry");
        println!("4. Quit");
        
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clean_al();
                let entry = ServiceInfo::new(
                    prompt("Service :"),
                    prompt("Username :"),
                    prompt("Password :"),
                );
                println!("Entry added successfully.");
                entry.write_to_file();
            },
            "2" => {
                clean_al();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords:{}", err);
                    Vec::new()
                });
                for item in &services{
                    println!(
                        "Service = {}
                        - Username : {}
                        - Password : {}",
                        item.service, item.username, item.password
                    );
                }
                
            },
            "3" => {
                clean_al();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords:{}", err);
                    Vec::new()
                });
                let search = prompt("Search :");
                for item in &services{
                    if item.service.as_str() == search.as_str(){
                        println!(
                            "Service = {}
                            - Username : {}
                            - Password : {}", 
                            item.service, item.username, item.password
                        )
                    }
                }
            },
            "4" => {
                clean_al();
                println!("Goodbye!!!");
                break; 
            },
            _ => println!("Invalid choice.")
        }
        println!("\n\n");
    }
}