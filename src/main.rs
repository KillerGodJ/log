extern crate lettre;
extern crate lettre_email;
extern crate mime;
extern crate time as out_time;

use lettre_email::Email;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};

use log::info;
use log4rs;
use std::{thread, time};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let a ="abc";
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    loop{
        
        let time = out_time::Time::now() ;
        println!("hour{} minute{} sec{}",time.hour(),time.minute(),time.second() );
        if (time.hour()+8 == 12 && time.minute() == 0 && time.second() ==0) || (time.hour()+8 == 0 && time.minute() == 0 && time.second() ==0) {
            email(a);
        }
        info!("123123");
        let one_min = time::Duration::new(1,1);
        thread::sleep(one_min);
        
    }
    Ok(())
}
//wdgqvencifnadiaf
fn email(a:&str){
    let email_receiver = "448193918@qq.com";
    let mine_email = "2443775863@qq.com";
    let smtp_server = "smtp.qq.com";
    let password = "wdgqvencifnadiaf"; 


    let email = Email::builder()
    .to(email_receiver)
    .from(mine_email)
    .subject("subject")
    .text(a)
    .build()
    .unwrap();
 
    let creds = Credentials::new(
        mine_email.to_string(),
        password.to_string(),
    );
 
    // Open connection to Gmail
    let mut mailer = SmtpClient::new_simple(smtp_server)
    .unwrap()
    .credentials(creds)
    .transport();
 
    // Send the email
    let result = mailer.send(email.into());
 
    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }
 
    print!("{:?}", result);
    mailer.close();
}