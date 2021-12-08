import { FC, ReactElement } from 'react'
import { useSelector } from 'react-redux'

import './App.css'

import Home from './compenent/home/home'
import NewProject from './compenent/new-project/new-project'
import Workspace from './compenent/workspace/workspace'
import StatusBar from './compenent/statusbar/statusbar'
import { RootState } from './state/store'
import { RouteType } from './state/features/navigationSlice'

const App: FC = (): ReactElement => {

  const navigation = useSelector((state: RootState)=> state.navigation.value)

  const navigatePages = () => {
    switch(navigation) {
      case RouteType.Home:   return <Home/>;
      case RouteType.NewProject:   return <NewProject/>;
      case RouteType.Workspace:   return <Workspace/>;
      default: return <Home/>
    }
  }

  
  return (
    <div className="app-container white" >
      <div className="app-body">
        {
          navigatePages()
        }
      </div>
      <StatusBar workspace="workspace"/>
    </div>
  )
}

export default App
