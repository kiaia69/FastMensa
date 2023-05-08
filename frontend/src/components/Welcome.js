import { Button, Typography, Paper } from '@mui/material'
import React, { useState } from 'react';
import image from "../utils/mensa.jpg";
import '../style/Welcome.css'
import Prenota from './Prenota';
import axios from "axios";



function Welcome({nome}) {

  const [listaMense, setLista] = useState([]);
  const [bottoni, setBottoni] = useState({
    prenota: false,
    visualizza: false,
    prenotazioni: false,
  })


  const titleStyle={ textAlign: "center",fontFamily: "Arial",fontSize: "48px",fontWeight: "bold",padding: "10px"}
  
  const customStyle = {
    border: "1px solid black",
    margin: '8px 0px',
    backgroundColor: 'black',
    fontSize: '20px',
    color: 'withe',
    fontWeight: 'bold',
  };



  const mostra = (e) => {
    e.preventDefault();
    let bottoneId = e.target.id;
    
    setBottoni(prevState => {
      const newBottoni = {};
      Object.keys(prevState).forEach(key => {
        newBottoni[key] = key === bottoneId ? !prevState[key] : false;
      });
      return newBottoni;
    });
    
      
      const request = {
          operazione: "lista",
          tipo: "utente",
      };
      axios.post("http://127.0.0.1:3000/App", request, {
          headers: {
              'Content-Type': 'application/json'
          }
      }).then((response) => {
          console.log(response);
          if (response.statusText === "OK") {
              const risposta = response.data;
              setLista(risposta);
              
              
          }
          else{
            alert("Errore da parte del server. Riprova!");
          }
  
      }).catch(error => {
          console.log(error.response);
      });
      
  };

  let obj =  <span className='benvenuto' style={{ fontFamily: "Arial",fontSize: "30px",fontWeight: "bold"}}>Ciao {nome}, non aspettare prenota subito il tuo posto alla mensa! </span>;

  


    return (
    <div className="Welcome">

     
      <Paper elevation={20} style={{
        padding :45,height:'75vh',width:'170vh', margin:"50px auto", border: "1px solid black",
        backgroundImage: `url(${image})`,
        backgroundRepeat: 'no-repeat',
        backgroundSize: 'cover',
        
                 }}>
      <Typography style={titleStyle} variant="h1">
        FastMensa
      </Typography>
      <Typography style={{fontFamily: "Arial",fontSize: "30px",fontWeight: "bold"}}variant="h5">
        Funzionalità disponibili
      </Typography>
      {bottoni.prenota ? (
        <Prenota mense={listaMense} />
      ) : bottoni.visualizza ? (
        <Prenota mense={listaMense}/>
      ) : bottoni.prenotazioni ? (
        <Prenota mense={listaMense}/>
      ) 
        : ( obj
      )
}

      <Button id="prenota" className="bottone" type='submit' variant="contained" style={customStyle} onClick={mostra}>PRENOTA</Button>
      
     
      
      
      <br/>
      <Button  id="visualizza" className="bottone" type='submit' variant="contained" style={customStyle} onClick={mostra}>VISUALIZZA DETTAGLI</Button><br/>
      
      <Button id="prenotazioni" className="bottone" type='submit' variant="contained" style={customStyle} onClick={mostra}>LE MIE PRENOTAZIONI</Button><br/>
      <Button  id="logout"className="bottone" type='submit' variant="contained" style={customStyle}>LOGOUT</Button><br/>
   

      

      
     
       </Paper>
      
      </div>
    );
  }
  
  export default Welcome;
  