import { Grid, Avatar, TextField, Button, Typography, Paper } from '@mui/material'
import React from 'react';
import image from "../utils/mensa.jpg";
import '../style/Welcome.css'




function Welcome({nome}) {


  const titleStyle={ textAlign: "center",fontFamily: "Arial",fontSize: "48px",fontWeight: "bold",padding: "10px"}
  
  const customStyle = {
    border: "1px solid black",
    margin: '8px 0px',
    backgroundColor: 'black',
    fontSize: '20px',
    color: 'withe',
    fontWeight: 'bold',
  };



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
      <Button className="bottone" type='submit' variant="contained" style={customStyle} >PRENOTA</Button>
     <span className='destra'>Ciao {nome}! </span>
      
     
      
      
      <br/>
      <Button className="bottone" type='submit' variant="contained" style={customStyle}>FUNZIONALITA1</Button><br/>
      <Button className="bottone" type='submit' variant="contained" style={customStyle}>FUNZIONALITA2</Button><br/>
   

      

      
     
       </Paper>
      
      </div>
    );
  }
  
  export default Welcome;
  