use postgres::{Client, NoTls, error};

// These params are Strings as they will be taken as env vars
pub fn connect(db_username: String, db_password: String, db_host: String, db_port: String, db_name: String) -> Result<Client, error::Error> {
    let details: String = format!("user={user} password={password} host={host} port={port} dbname={dbname}", user = db_username, password = db_password, host = db_host, port = db_port, dbname = db_name);
    return Client::connect(&details.as_str(), NoTls);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect(){
        println!("I am running the test.");
        let result = connect("nicholasf".to_string(), "password".to_string(), "127.0.0.1".to_string(), "5432".to_string(), "word_counter".to_string());

        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false) 
        };
    }
}