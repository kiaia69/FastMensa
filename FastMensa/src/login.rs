use std::{fs::File, io::Read, path::Path};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Login {
    username: String,
    password: String,
}



impl Login{
    
    pub(crate) fn verifica_credenziali(credenziali: Login) -> bool {

/* NON MI FA LEGGERE IL FILE PERCHÈ??? */
        let mut iter;
        let mut contents = String::new();
        let file_path = Path::new("db/utenti.txt");

        let mut fd = File::open(file_path).expect("Non sono riuscito ad aprire il file");
        fd.read_to_string(&mut contents).expect("Non sono riuscito a leggere il file");
        let mut result = false;
        

        for line in contents.lines(){
            iter = line.split_whitespace();
            if iter.next().unwrap_or_default() == credenziali.username && iter.next().unwrap_or_default() == credenziali.password {

                    result=true;
                    break;

            }
        }

        return result;
    }
}