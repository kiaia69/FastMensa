use std::{ io::Write, path::Path, fs::File};
use serde::Deserialize;
use std::fs::OpenOptions;

#[derive(Debug, Deserialize)]
pub(crate) struct Registrazione {
    username: String,
    email: String, 
    password: String,
}



impl Registrazione{
    
    pub(crate) fn registra(dati:  Registrazione) -> bool {

        let file_path = Path::new("db/utenti.txt");
       
       /*  let mut file = OpenOptions::new().append(true).open(file_path).expect("Impossibile aprire il file");
        let  linea =  format!("{} {} {}\n", dati.username, dati.password, dati.email);
         file.write_all(linea.as_bytes()).expect("Impossibile scrivere nel file");
   */
        let mut fd = File::open(file_path).expect("Non sono riuscito ad aprire il file");
        let  linea =  format!("{} {} {}\n", dati.username, dati.password, dati.email);
        fd.write(linea.as_bytes()).expect("Errore nella scrittura del file");
        return true;
       
    }
}