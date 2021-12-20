import React from 'react';
import ReactDOM from 'react-dom';
/* import "@fontsource/poppins"; */
import "@fontsource/roboto";
import "@fontsource/roboto/300.css";
import "@fontsource/roboto-mono";
import "@fontsource/roboto-mono/300.css";
import "@fontsource/material-icons";
import "@fontsource/material-icons-outlined";
import './index.css';
import App from './App';
import { Provider } from 'react-redux';
import { store } from './state/store';
//import reportWebVitals from './reportWebVitals';


ReactDOM.render(
  <React.StrictMode>
    <Provider store = {store}>
      <App />
    </Provider>  
  </React.StrictMode>,
  document.getElementById('root')
);


//reportWebVitals();
