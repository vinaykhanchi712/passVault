use std::io;
use std::fs::OpenOptions;
use std::fs::File;
use serde::Serialize;
use serde::Deserialize;
use std::io::{BufRead, Write};
use std::io::stdin;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub service : String,
    pub username : String,
    pub password : String,
}

impl ServiceInfo{
    
    pub fn new(service_name: String, username :String, password: String) -> ServiceInfo{
        ServiceInfo{
            service: service_name,
            username,
            password,
        }
    }
    
    pub fn to_json(&self)-> String {
        let json = serde_json::to_string(self).expect("error parsing to json.");
        return json
    }

    pub fn from_json(json : &str) -> ServiceInfo{
        let service = serde_json::from_str(json).expect("Error while parsing from json.");
        return service
    }

    
    pub fn from_user_input() -> Self {
        println!("Enter Service Entry:");
        let mut service = String::new();
        io::stdin()
            .read_line(&mut service)
            .expect("Failed to read line");

        println!("Enter Username:");
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");

        println!("Enter Password:");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line");

        ServiceInfo::new(
            service.trim().to_string(),
            username.trim().to_string(),
            password.trim().to_string(),
        )
    }
    
    
    pub fn write_to_file(&self) -> Result<(), std::io::Error>{
        let json_output = format!("{}\n", self.to_json());
        
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("passwords.json"){
        
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_output.as_bytes()) {
                    eprintln!("Error writing to file: {}", e);
                } else {
                    println!("Successfully wrote to passwords.json");
                }
            }
            Err(e) => eprintln!("Error opening file: {}", e),
        }
        
        Ok(())
    }
}

pub fn read_password_from_file()->  Result<Vec<ServiceInfo>, io::Error>{
    let file = File::open("passwords.json").expect("Error opening the file.");
    let reader = io::BufReader::new(file);

    let mut services = Vec::new();

    for line in reader.lines() {
        if let Ok(json_string) = line {
            if let service_info = ServiceInfo::from_json(&json_string) {
                services.push(service_info);
            }
        }
    }

    Ok(services)

}

pub fn prompt(prompt : &str) -> String{
    println!("{}",prompt);
    let mut value = String::new();
    stdin().read_line(&mut value).expect("Error while taking input from user.");
    return value.trim().to_string()
}