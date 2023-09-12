mod app_db;
mod db;
use app_db::{AppDbMessages, AppDbResposne, AppDbTaskInterface, app_db_run};

use std::thread;
use std::sync::mpsc::{self};
use std::time::Duration;


fn main() {
    
    let (tx_from_app_to_db, rx_from_app_to_db )= mpsc::channel::<AppDbMessages>();
    let (tx_from_db_to_app, rx_from_db_to_app ) = mpsc::channel::<AppDbResposne>();

    let app_main = thread::spawn(move || {
        loop {
            // tx_from_app_to_db.send(DbMessages::DeleteLog( 1 )).unwrap();
            // tx_from_app_to_db.send(DbMessages::UpdateLogMessage(1, "This is a message".to_string())).unwrap();
            // tx_from_app_to_db.send(DbMessages::UpdateLogTitle(1, "New Title".to_string())).unwrap();
            tx_from_app_to_db.send(AppDbMessages::GetLog( 2 )).unwrap();
            //tx_from_app_to_db.send(DbMessages::AddNewLog("Title".to_string(), "Message".to_string())).unwrap();
            let received_data = rx_from_db_to_app.recv().unwrap();
            match received_data {
                AppDbResposne::LogNames(logs) => {
                    println!("LogNames");
                    for log in logs {
                        println!("Log: {:?}", log);
                    }
                },
                AppDbResposne::Log(log) => {
                    println!("Log: {:?}", log);
                },
                AppDbResposne::Ok => {
                    println!("Ok");
                },
                AppDbResposne::Error => {
                    println!("Error");
                },
                _ => {
                    println!("Default");
                },
            }

            // Sleep for a while to simulate work
            thread::sleep(Duration::from_secs(2));
        }
    });

    app_db_run( AppDbTaskInterface { rx_from_app_to_db, tx_from_db_to_app } );
    app_main.join().unwrap();


}
