mod login; 
mod registrazione;


use bytes::Bytes;
use http::HeaderValue;
use login::Login;
use registrazione::{RegistrazioneMensa, RegistrazioneUtente};
use anyhow::{Result};
use serde::Deserialize;
use spin_sdk::{
    http::{Request, Response},
    http_component, key_value::Store,
};
use std::str;



#[derive(Debug, Deserialize)]
struct Type {
   operazione: String,
   tipo: String, //per capire se è un utente o una mensa durante il login
}


/// A simple Spin HTTP component.
#[http_component]
fn handle_fast_mensa(req: Request) -> Result<Response> {

 

  let method = req.method(); //metodo della richiesta   
  let ok =  controlla_metodo(method.clone());

  if !ok {
        return bad_request();
  }
  // Estrarre il corpo della richiesta come un vettore di byte
  let temp = Bytes::new();
  let body: &bytes::Bytes = req.body().as_ref().unwrap_or(&temp);

  if body.is_empty(){ //non sono riuscito a leggere il body oppure c'è stato un erroee nella richiesta
    return bad_request();
  }

  // Convertire il corpo da byte a stringa
  let body_string: String = String::from_utf8_lossy(body.as_ref()).to_string();

  // Deserializzare il corpo della richiesta in una struct
  let tipo: Type = serde_json::from_str(&body_string).expect("Errore durante la deserializzazione del JSON");
  //let login: Login = serde_json::from_str(&body_string)?;

  println!("Ricevuto una richiesta di tipo {} per il {} di un/una {}", method, tipo.operazione, tipo.tipo);
 
   println!("{}", body_string);

  match tipo.operazione.as_str(){
        "login" =>  handle_login(serde_json::from_str(&body_string).expect("Errore durante la deserializzazione del JSON"), tipo.tipo),  
        "registrazione" => {
            
            match tipo.tipo.as_str(){
           "Utente" => handle_registration_user(serde_json::from_str(&body_string).expect("Errore durante la deserializzazione del JSON")),          
           "Mensa" => handle_registration_mensa(serde_json::from_str(&body_string).expect("Errore durante la deserializzazione del JSON")),
           _ => bad_request(),
            }
        }
        "lista" => handle_lista(),
        _ =>bad_request(),
   }

}

fn controlla_metodo(metodo: http::Method) -> bool {

    let result = match metodo {
        http::Method::POST => true,
        _ => false,
    };
    return result;
}

fn bad_request() -> Result<Response> {

    Ok(http::Response::builder()
    .status(http::StatusCode::OK) //se non metto OK non funziona, perchè????
    .header(http::header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization")
    .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
    .body(None)?)
}

fn logged(esito: String) -> Result<Response> {

  Ok(http::Response::builder()
    .status(http::StatusCode::OK)
     .header(http::header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization")
    .header(http::header::CONTENT_TYPE, "application/json")
    .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
    .body(Some(esito.into()))?)
}

fn handle_login(credenziali: Login, tipo: String) -> Result<Response> {

if tipo == "Utente" {
    match Login::verifica_credenziali_utente(credenziali) {
        Ok(value) => logged(value.to_string()),
        Err(e) => server_error(e.to_string())
    }
}
else {
    match Login::verifica_credenziali_mensa(credenziali) {
        Ok(value) => logged(value.to_string()),
        Err(e) => server_error(e.to_string())
        }
    }
}

fn handle_registration_user(credenziali: RegistrazioneUtente) -> Result<Response> {
    match RegistrazioneUtente::registra_utente(credenziali) {
        Ok(value) => registrato(value.to_string()),
        Err(e) => server_error(e.to_string())
    }
}

fn handle_registration_mensa(credenziali: RegistrazioneMensa) -> Result<Response> {

   
        match RegistrazioneMensa::registra_mensa(credenziali) {
            Ok(value) => registrato(value.to_string()),
            Err(e) => server_error(e.to_string())
        }

}
fn server_error(errore: String) -> Result<Response> {
    Ok(http::Response::builder()
    .status(http::StatusCode::INTERNAL_SERVER_ERROR)
    .body(Some(errore.into()))?)
}

fn handle_lista() -> Result<Response> {
    let store = Store::open("mense")?;
    let keys = store.get_keys()?;

    let json_data = serde_json::to_string(&keys)?;

    Ok(http::Response::builder()
      .status(http::StatusCode::OK)
       .header(http::header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization")
      .header(http::header::CONTENT_TYPE, "application/json")
      .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
      .body(Some( json_data.into()))?)
}

fn registrato(esito: String) -> Result<Response> {
    
    Ok(http::Response::builder()
      .status(http::StatusCode::OK)
       .header(http::header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization")
      .header(http::header::CONTENT_TYPE, "application/json")
      .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
      .body(Some(esito.into()))?)
  }