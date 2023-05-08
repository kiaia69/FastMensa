use serde::Deserialize;
use anyhow::Result;
use spin_sdk::{
    key_value::{ Store},
};

#[derive(Debug, Deserialize)]
pub(crate) struct RegistrazioneUtente {
    username: String,
    email: String, 
    password: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RegistrazioneMensa {
    nome: String,
    citta: String,
    posizione: String, 
    email: String,
    password: String,
}




impl RegistrazioneUtente{
    
    pub(crate) fn registra_utente(dati:  RegistrazioneUtente) -> Result<bool> {

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
impl RegistrazioneMensa{

    pub(crate) fn registra_mensa(dati:  RegistrazioneMensa) -> Result<bool> {

        //la mia chiave è l'username
        let key = dati.nome.clone();
       
        let store = Store::open("mense")?;
        
        if store.exists(key).unwrap_or_default() { //username già esistente
            return Ok(false)
        }

        println!("{}", format!("{} {}", dati.password, dati.email));
    
        store.set(dati.nome, format!("{} {} {} {}", dati.password, dati.email, dati.citta, dati.posizione))?; //il valore è dato dalla password e dall'email separate da uno spazio
        //let value = store.get("mykey")?;
       
        return Ok(true);
       
    }
}