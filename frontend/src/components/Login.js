import React, { useState } from 'react'
import { Grid, Avatar, TextField, Button, Typography, Paper, Switch } from '@mui/material'
import { NavLink } from "react-router-dom";

const paperStyle = { padding: 45, height: '40vh', width: 280, margin: "100px auto" }

const Login = ({ onChangeUsername, onChangePassword, onLogin, Oncambia}) => {
    const avatarStyle = { backgroundColor: 'black' }
    const btnstyle = { margin: '8px 0', backgroundColor: 'black' }

    const [tipo, setTipo] = useState("Utente");

    const changeUsername = (e) => onChangeUsername(e)

    const login = (e) => onLogin(tipo, e)

    const changePassword = (e) => onChangePassword(e)

    const swictch = (e) =>{
        if (tipo === "Utente") {
            setTipo("Mensa");
        }
        else setTipo("Utente");
        Oncambia(e);
    }
    

    return (
        <div>
            <Grid>
                <Paper elevation={10} style={paperStyle}>
                    <Grid align='center'>
                        <Avatar style={avatarStyle}></Avatar>
                        <h3>Bentornato, effettua l'accesso</h3>
                    </Grid>
                    <TextField label='Username' placeholder='Enter username' fullWidth required onChange={changeUsername} />
                    <TextField label='Password' placeholder='Enter password' type='password' fullWidth required onChange={changePassword} />

                    <Button type='submit' variant="contained" style={btnstyle} fullWidth onClick={login}>Login</Button>

                    <Typography > Non hai un account?
                        <NavLink className="navbar-item" activeclassname="is-active" to="/registrazione" exact="true">Registrati</NavLink>
                    </Typography>
                    <Switch  color="default" onChange={swictch}/> {tipo}
                </Paper>

            </Grid>
        </div>

    )
}

export default Login