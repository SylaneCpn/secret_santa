
use rand::prelude::SliceRandom;
use std::fs;

use lettre::{Message, SmtpTransport, Transport}; 
use lettre::transport::smtp::{authentication::{Credentials}}; 

#[derive(serde::Deserialize)]
struct User {
    name : String,
    address : String
}




fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    
    let email_addr = "your_email_addr@gmail.com";
    let cre = "your 16 digit credentials";


    let users = load_users()?;

    let dest = randomise_dest(users.len());

    let mut r = String::from("|sender ~~> receiver|\n");

    for (send , dest ) in (0..users.len()).zip(dest.into_iter()) {

        r = format!("{}|{} ~~> {}|\n",r,users[send].name , users[dest].name);
        send_email(&users[send] , &users[dest] , email_addr ,cre)?;

    }

    fs::write("report.txt" , r.as_bytes())?;
    
    Ok(())
}


fn load_users() -> Result<Vec<User> , Box<dyn std::error::Error>> 
{
    let content = fs::read_to_string("users.json")?;
    let mut users = serde_json::from_str::<Vec<User>>(&content)?;
    users.sort_by_key(|x| x.name.clone());
    Ok(users)
}

fn randomise_dest(size : usize) -> Vec<usize>
{
    let mut rng = rand::thread_rng();
    let i_array = (0..size).collect::<Vec<_>>();
    let mut uns_i_array = i_array.clone();
    uns_i_array.shuffle(&mut rng);

    while i_array.iter().zip(uns_i_array.iter()).any(|(sorted , unsorted)| *sorted == *unsorted) {
        uns_i_array.shuffle(&mut rng);
    }

    uns_i_array

    
}

fn send_email(send : &User , dest : &User , sending_addr : &str , credentials : &str ) -> Result<(), Box<dyn std::error::Error>>
{   
    // let send_mail = send.address.as_str();
    let dest_mail = send.address.as_str();

    


    let email = Message::builder()
        // Set the sender's name and email address
        .from(sending_addr.parse().unwrap()) 
        // Set the recipient's name and email address
        .to(dest_mail.parse().unwrap()) 
        // Set the subject of the email
        .subject("Secret Santa") 
        // Set the body content of the email
        .body(format!("Congratulations {} !\n You are the secret Santa of {} !\nGive {} a present that will be remembered !\n Peace Out !\n This Email was sent automaticly",&send.name , &dest.name , &dest.name)).unwrap();

    let creds = Credentials::new(sending_addr.to_string() , credentials.to_string());

        
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email).unwrap();
    Ok(())
}
