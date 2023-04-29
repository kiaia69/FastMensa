import './App.css';
import React, { useState } from 'react';
import Login from './components/Login';
import { Routes, Route, Navigate, BrowserRouter, useNavigate } from "react-router-dom";
import Registrazione from './components/Registrazione';
import Welcome from './components/Welcome';
import axios from "axios";

function AppRouter() {
  const navigate = useNavigate();

  const [credenziali, setCredenziali] = useState({
    username: '',
    password: '',
    login: false,
  });

  

  const changeUsername = (e) => {

    setCredenziali({
        ...credenziali,
        username: e.target.value
    });
}

const changePassword = (e) => {

    setCredenziali({
        ...credenziali,
        password: e.target.value
    });
}
const registrazione = (username, email, password, e) => {
  e.preventDefault();
  const registrationData = {
    username: username,
    email: email,
    password: password,
    operazione: "registrazione"
  }


  axios.post("http://127.0.0.1:3000/App", registrationData, {
    headers: {
        'Content-Type': 'application/json'
    }
}).then((response) => {
    console.log(response);
    if (response.data === true ) {
         
        alert("Ti sei registrato con successo");
        navigate('/');

    }
    else{
      alert("Errore del server, registrazione annullata");
    }

}).catch(error => {
    console.log(error.response);
    alert("Errore del server, registrazione annullata");
});
};

const login = (e) => {

    e.preventDefault();
    const userData = {
        username: credenziali.username,
        password: credenziali.password,
        operazione: "login"
    };
    axios.post("http://127.0.0.1:3000/App", userData, {
        headers: {
            'Content-Type': 'application/json'
        }
    }).then((response) => {
        console.log(response);
        if (response.data === true) {

            setCredenziali({
                ...credenziali,
                login: true
            });
            navigate('/welcome');
        }
        else{
          alert("Username e/o password errate");
        }

    }).catch(error => {
        console.log(error.response);
    });
};


  return (
    <Routes>
      <Route path='/' element={credenziali.login ? <Navigate to='/welcome' /> : <Login 
        onChangeUsername={changeUsername} 
        onChangePassword={changePassword} 
        onLogin={login}/>} 
      />
      <Route path='/welcome' element={credenziali.login ? <Welcome /> : <Navigate to='/' />} />
      <Route path='/registrazione' element={<Registrazione  
       onRegistration={registrazione}/>} 
      />
    </Routes>
  )


}


const App = () => <BrowserRouter><AppRouter /></BrowserRouter>

export default App;