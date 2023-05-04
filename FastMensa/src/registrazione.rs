use serde::Deserialize;
use anyhow::Result;
use spin_sdk::{
    key_value::{ Store},
};

#[derive(Debug, Deserialize)]
pub(crate) struct Registrazione {
    username: String,
    email: String, 
    password: String,
}



impl Registrazione{
    
    pub(crate) fn registra(dati:  Registrazione) -> Result<bool> {

        //la mia chiave è l'username
        let key = dati.username.clone();
        
        let store = Store::open_default()?;
        if store.exists(key).unwrap_or_default() { //username già esistente
            return Ok(false)
        }

        println!("{}", format!("{} {}", dati.password, dati.email));
    
        store.set(dati.username, format!("{} {}", dati.password, dati.email))?; //il valore è dato dalla password e dall'email separate da uno spazio
        //let value = store.get("mykey")?;
       
        return Ok(true);
       
    }
}