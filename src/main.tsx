import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import "@fontsource/roboto";
import "@fontsource/roboto/300.css";
import "@fontsource/roboto-mono";
import "@fontsource/roboto-mono/300.css";
import "@fontsource/material-icons";
import "@fontsource/material-icons-outlined";

import "./styles/index.css";


import { Provider } from 'react-redux';
import { store } from './state/store';


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Provider store = {store}>
      <App />
    </Provider>  
  </React.StrictMode>
);
