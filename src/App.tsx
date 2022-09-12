import React,{ FC, ReactElement } from 'react'
import { useSelector } from 'react-redux'

import './App.css'

import Home from './ui/home/home'
import NewProject from './ui/new-project/new-project'
import Workspace from './ui/workspace/workspace'

import StatusBar from './compenent/statusbar/statusbar'
import { RootState } from './state/store'
import { RouteType } from './state/features/navigationSlice'
import { ThemeType } from './state/features/themeSlice'

import blacklogo from './images/logo_black.svg'
import whitelogo from './images/logo_white.svg'

/* const Home = React.lazy(() => import('./ui/home/home'));
const NewProject = React.lazy(() => import('./ui/new-project/new-project'));
const Workspace = React.lazy(() => import('./ui/workspace/workspace'));
 */

const App: FC = (): ReactElement => {

  const navigation = useSelector<RootState, RouteType>((state)=> state.navigation.value)
  const theme = useSelector((state: RootState)=> state.theme.value)

  const bgImageUrl = () => {
    return (theme === ThemeType.WHITE) ? `url(${whitelogo})` : `url(${blacklogo})`
  }

  const navigatePages = () => {
    console.log(navigation)
    switch(navigation) {
      case RouteType.Home: return <Home/>;
      case RouteType.NewProject: return <NewProject/>;
      case RouteType.Workspace: return <Workspace/>;
      default: return <Home/>
    }
  }
  
  return (
    <div className={`app-container ${(theme === ThemeType.WHITE) ? 'white' : 'black'}`} >
      <div className="app-body" style={{backgroundImage: bgImageUrl()}}>
        {
          navigatePages()
        }
      </div>
      <StatusBar workspace="workspace"/>
    </div>
  )
}

export default App
