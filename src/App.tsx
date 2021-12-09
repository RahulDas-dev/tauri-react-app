import { FC, ReactElement } from 'react'
import { useSelector } from 'react-redux'

import './App.css'

import Home from './compenent/home/home'
import NewProject from './compenent/new-project/new-project'
import Workspace from './compenent/workspace/workspace'
import StatusBar from './compenent/statusbar/statusbar'
import { RootState } from './state/store'
import { RouteType } from './state/features/navigationSlice'
import { ThemeType } from './state/features/themeSlice'

const App: FC = (): ReactElement => {

  const navigation = useSelector((state: RootState)=> state.navigation.value)
  const theme = useSelector((state: RootState)=> state.theme.value)

  const bgImageUrl = () => {
    let bg_image =  (theme === ThemeType.WHITE) ? '/image/logo_white.svg' : '/image/logo_black.svg'
    return `url(${process.env.PUBLIC_URL+ bg_image})`
  }

  const navigatePages = () => {
    switch(navigation) {
      case RouteType.Home:   return <Home/>;
      case RouteType.NewProject:   return <NewProject/>;
      case RouteType.Workspace:   return <Workspace/>;
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
