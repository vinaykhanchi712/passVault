mod pentry;



use crate::pentry::{prompt, ServiceInfo};


fn clr(){
    println!("{}[2J",27 as char)
}

fn main() {
    clr();
    let ascii = r#"

__________                        ____   ____            .__   __
\______   \_____    ______ ______ \   \ /   /____   __ __|  |_/  |_
 |     ___/\__  \  /  ___//  ___/  \   Y   /\__  \ |  |  \  |\   __\
 |    |     / __ \_\___ \ \___ \    \     /  / __ \|  |  /  |_|  |
 |____|    (____  /____  >____  >    \___/  (____  /____/|____/__|
                \/     \/     \/                 \/

    "#;


    println!("{}",ascii);

    loop{
        println!("Password Manager Menu:");
        println!("1. Add Entry.");
        println!("2. List Entries.");
        println!("3. Search Entry.");
        println!("4. Quit.");
        
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            
            "1" =>{
                
                let service = ServiceInfo::from_user_input();
                
                if let Err(_e) = service.write_to_file(){
                    eprintln!("Error adding entry");
                }else{
                    println!("Entry added successfully.");
                }
                
            }
            
            "2" =>{
                clr();
                let services = pentry::read_password_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service = {}
                        - Username : {} 
                        - Password : {}",
                        item.service, item.username, item.password
                    );
                }
            }
            
            "3" =>{
                clr();
                let services = pentry::read_password_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                let search = prompt("Search :");
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "Service = {}
                            - Username : {} 
                            - Password : {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            
            "4" =>{
                println!("Good bye");
                break;
            }
            
            _ => {
                println!("Invalid choice")
            }
        }

    }
}


