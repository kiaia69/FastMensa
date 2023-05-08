use spin_sdk::{
    key_value::{ Store},
};
use serde::Deserialize;
use anyhow::Result;
#[derive(Debug, Deserialize)]
pub(crate) struct Login {
    username: String,
    password: String,
}



impl Login{
    
    pub(crate) fn verifica_credenziali_utente(credenziali: Login) -> Result<bool> {

        let key = credenziali.username.clone();

        let store = Store::open_default()?;
        if !store.exists(key).unwrap_or_default() { //username non valido se non presente 
            return Ok(false)
        }
       let value = store.get(credenziali.username)?;
       let string = String::from_utf8(value).unwrap();
       let mut values = string.split_whitespace();
       let pass = values.next().unwrap_or_default();

        if credenziali.password == pass{
                return Ok(true);
        }
       
        return Ok(false);
    }

    pub(crate) fn verifica_credenziali_mensa(credenziali: Login) -> Result<bool> {

        let key = credenziali.username.clone();

        let store = Store::open("mense")?;
        if !store.exists(key).unwrap_or_default() { //nome della mensa non valido se non presente 
            return Ok(false)
        }
       let value = store.get(credenziali.username)?;
       let string = String::from_utf8(value).unwrap();
       let mut values = string.split_whitespace();
       let pass = values.next().unwrap_or_default();

        if credenziali.password == pass{
                return Ok(true);
        }
       
        return Ok(false);
    }
}