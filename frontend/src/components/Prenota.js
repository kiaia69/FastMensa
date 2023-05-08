import React from "react";
import '../style/Welcome.css'
import { Button, Typography, Paper } from '@mui/material'

const Prenota = (mense) => {

  
    
    return(
        <div className="destra">
    <Paper elevation={20} style={{
        height:'55vh',width:'91.2vh', margin:"50px auto", border: "1px solid black", backgroundColor: 'black'
              }}>
      <Typography style={{fontFamily: "Arial",fontSize: "30px",fontWeight: "bold"}}variant="h5">
        Mense disponibili per la prenotazioni:
        {mense.map((elemento, index) => (
        <div key={index}>
          {elemento}
          <Button>Accedi</Button>
        </div>
      ))}
      </Typography>
      </Paper>
</div>
    );

}

export default Prenota;
