import React from 'react'
import  { useState } from 'react';
import { Grid, Avatar, TextField, Button, Typography, Paper } from '@mui/material'
import {   NavLink } from "react-router-dom";

const paperStyle={padding :45,height:'40vh',width:280, margin:"100px auto"}

const Registrazione=({onRegistration})=>{
    const avatarStyle={backgroundColor:'red'}
    const btnstyle={margin:'8px 0', backgroundColor: 'red'}
    
    const [registrazione, setRegistrazione] = useState({
        username: '',
        email: '',
        password: '',
        confermaPassword: ''
      })

      function verificaEmail(email) {
        var regex = /^[\w-.]+@([\w-]+\.)+[\w-]{2,4}$/;
        return regex.test(email);
      }

      const changeUsername = (e) => {

        setRegistrazione({
            ...registrazione,
            username: e.target.value
        });
    }

    const changeConfermaPassword = (e) => {

        setRegistrazione({
            ...registrazione,
            confermaPassword: e.target.value
        });
    }
    
    const changePassword = (e) => {
    
        setRegistrazione({
            ...registrazione,
            password: e.target.value
        });
    }

    const changeEmail = (e) => {

        setRegistrazione({
            ...registrazione,
            email: e.target.value
        });
    }

      const registra = (e) => {
        if(!verificaEmail(registrazione.email)){
                     alert("Email inserita non valida");
                     setRegistrazione({
                             ...registrazione,
                           email: ''
            });

        }
        else if (registrazione.confermaPassword === registrazione.password){
        onRegistration(registrazione.username, registrazione.email, registrazione.password, e)
        }
        else {

            alert("Le password non corrispondono");
            setRegistrazione({
                ...registrazione,
                password: '',
                confermaPassword: '',
            });
        }
           
      };
    
    return(
       <div>
          
       <Grid>
            <Paper elevation={10} style={paperStyle}>
          
          
                <Grid align='center'>
                     <Avatar style={avatarStyle}></Avatar>
                    <h3>Benvenuto, registrati</h3>
                </Grid>
                <TextField label='Username' value={registrazione.username} placeholder='Enter username' fullWidth required onChange={changeUsername}/>
                <TextField label='Email' value={registrazione.email} placeholder='Enter email' fullWidth required onChange={changeEmail}/>
                <TextField label='Password' value={registrazione.password} placeholder='Enter password' type='password' fullWidth required onChange={changePassword}/>
                <TextField label='Conferma Password' value={registrazione.confermaPassword} placeholder='Enter password' type='password' fullWidth required onChange={changeConfermaPassword}/>
              
                <Button type='submit' variant="contained" style={btnstyle} fullWidth onClick={registra}>Registrati</Button>
              
                <Typography > hai già un account? 
                     <NavLink className="navbar-item" activeclassname="is-active" to="/" exact="true">Accedi</NavLink>
                </Typography> 
                
                  </Paper>
      
                 </Grid>
                </div>
         
    )
}

export default Registrazione;