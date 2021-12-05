import React, { FC, ReactElement } from 'react'
import './App.css'

import AppBody from './compenent/appbody/appbody'
import StatusBar from './compenent/statusbar/statusbar'

const App: FC = (): ReactElement => {
  return (
    <div className="app-container" >
      <AppBody/>
      <StatusBar workspace="workspace"/>
    </div>
  )
}

export default App
