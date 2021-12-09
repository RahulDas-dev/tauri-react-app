import { FC, ReactElement } from "react";
import { useDispatch } from "react-redux";

import History from "../history/history";
import { navigate, RouteType } from "../../state/features/navigationSlice";

import './home.css'


const Home: FC = (props): ReactElement => {

    
    const dispatch = useDispatch()
    
    return (
        <div className="app-home"  >
            <div className="init-action">
                <div className="btn-group">
                    <button className="btn rounded" onClick = {()=> dispatch(navigate(RouteType.NewProject))}>
                        <span className="material-icons">add</span> New Project
                    </button>
                    <button className="btn rounded" onClick = {()=> dispatch(navigate(RouteType.Workspace))}>
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
  