mod login; 
mod registrazione;


use bytes::Bytes;
use http::HeaderValue;
use login::Login;
use registrazione::Registrazione;
use anyhow::{Result};
use serde::Deserialize;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use std::str;



#[derive(Debug, Deserialize)]
struct Type {
   operazione: String,
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

  println!("Ricevuto una richiesta di tipo {} per il {}", method, tipo.operazione);
 
   println!("{}", body_string);

  match tipo.operazione.as_str(){
        "login" =>  handle_login(serde_json::from_str(&body_string).expect("Errore durante la deserializzazione del JSON")),  
        "registrazione" => handle_registration(serde_json::from_str(&body_string).expect("Errore durante la deserializzazione del JSON")),          
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

fn handle_login(credenziali: Login) -> Result<Response> {


    match Login::verifica_credenziali(credenziali) {
        Ok(value) => logged(value.to_string()),
        Err(e) => server_error(e.to_string())
    }

}

fn handle_registration(credenziali: Registrazione) -> Result<Response> {

    match Registrazione::registra(credenziali) {
        Ok(value) => registrato(value.to_string()),
        Err(e) => server_error(e.to_string())
    }

}
fn server_error(errore: String) -> Result<Response> {
    Ok(http::Response::builder()
    .status(http::StatusCode::INTERNAL_SERVER_ERROR)
    .body(Some(errore.into()))?)
}

fn registrato(esito: String) -> Result<Response> {
    
    Ok(http::Response::builder()
      .status(http::StatusCode::OK)
       .header(http::header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization")
      .header(http::header::CONTENT_TYPE, "application/json")
      .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
      .body(Some(esito.into()))?)
  }