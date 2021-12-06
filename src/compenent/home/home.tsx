import { FC, ReactElement } from "react";

import History from "../history/history";

import './home.css'



const Home: FC = (props): ReactElement => {

    const bgImageUrl = () => `url(${process.env.PUBLIC_URL+'/image/logo_white.svg'})`
    //console.log(bgImageUrl())
    
    return (
        <div className="app-home" style={{backgroundImage: bgImageUrl()}} >
            <div className="init-action">
                <div className="btn-group">
                    <button className="btn rounded">
                        <span className="material-icons">add</span> New Project
                    </button>
                    <button className="btn rounded">
                        <span className="material-icons-outlined md-18">code</span> Workspace
                    </button>
                    <button className="btn rounded">
                        <span className="material-icons-outlined">format_list_bulleted</span> Read Document
                    </button>
                </div>
            </div>
            <History/>
        </div>
    );
  };
  
  export default Home;
  