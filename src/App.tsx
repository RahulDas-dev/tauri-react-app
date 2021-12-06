import React, { FC, ReactElement } from 'react'

import './App.css'

import Home from './compenent/home/home'
import StatusBar from './compenent/statusbar/statusbar'

const App: FC = (): ReactElement => {
  
  return (
    <div className="app-container white" >
      <div className="app-body">
        <Home/>
      </div>
      <StatusBar workspace="workspace"/>
    </div>
  )
}

export default App
