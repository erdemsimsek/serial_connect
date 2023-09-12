use crate::db::{Db, DbError, Log};


use std::thread;
use std::sync::mpsc::{self};

pub enum AppDbMessages {
    AddNewLog(String, String),
    UpdateLogTitle( u32, String),
    UpdateLogMessage(u32, String),
    GetAllLogNames,
    GetLog(u32),
    DeleteLog(u32),
    KillThread,
}

pub enum AppDbResposne {
    Ok,
    Error,
    LogNames(Vec<(u32, String)>),
    Log(Log),
}

pub struct AppDbTaskInterface {
    pub rx_from_app_to_db: mpsc::Receiver<AppDbMessages>,
    pub tx_from_db_to_app: mpsc::Sender<AppDbResposne>,
}


pub fn app_db_run( interface : AppDbTaskInterface )
{
    let rx_from_app_to_db = interface.rx_from_app_to_db;
    let tx_from_db_to_app = interface.tx_from_db_to_app;

    let app_db = thread::spawn( move || {
    let db = Db::new().unwrap();

        loop {

            match rx_from_app_to_db.recv() {
                Ok(AppDbMessages::AddNewLog(title, message)) => {
                    println!("AddNewLog to db");
                    let _ = match db.add_log(&title, &message)
                    {
                        Ok(_) => tx_from_db_to_app.send(AppDbResposne::Ok),
                        Err(_) => tx_from_db_to_app.send(AppDbResposne::Error),
                    };
                },
                Ok(AppDbMessages::UpdateLogTitle(id, title)) => {
                    println!("UpdateLogTitle to db");
                    let _ = match db.update_log_title(id, &title)
                    {
                        Ok(_) => tx_from_db_to_app.send(AppDbResposne::Ok),
                        Err(_) => tx_from_db_to_app.send(AppDbResposne::Error),
                    };
                },
                Ok(AppDbMessages::UpdateLogMessage(id, message)) => {
                    println!("UpdateLogMessage to db");
                    let _ = match db.update_log(id, &message)
                    {
                        Ok(_) => tx_from_db_to_app.send(AppDbResposne::Ok),
                        Err(_) => tx_from_db_to_app.send(AppDbResposne::Error),
                    };
                },
                Ok(AppDbMessages::GetAllLogNames) => {
                    let _ = match db.get_all_log_names()
                    {
                        Ok(logs) => tx_from_db_to_app.send(AppDbResposne::LogNames(logs)),
                        Err(_) => tx_from_db_to_app.send(AppDbResposne::Error),
                    };
                },
                Ok(AppDbMessages::GetLog(log_id)) => {
                    println!("GetLog for {}", log_id);
                    let _ = match db.get_log(log_id)
                    {
                        Ok(log) => tx_from_db_to_app.send(AppDbResposne::Log(log)),
                        Err(_) => tx_from_db_to_app.send(AppDbResposne::Error),
                    };
                },
                Ok(AppDbMessages::DeleteLog(id)) => {
                    println!("DeleteLog for {}", id);
                    let _ = match db.delete_log(id)
                    {
                        Ok(_) => tx_from_db_to_app.send(AppDbResposne::Ok),
                        Err(_) => tx_from_db_to_app.send(AppDbResposne::Error),
                    };
                },
                Ok(AppDbMessages::KillThread) => {
                    println!("KillThread");
                    break;
                },
                Err(_) => {
                    println!("Error");
                    break;
                },
                _ => {
                    println!("Default");
                    break;
                },
            }
        }
    });

    app_db.join().unwrap();
}