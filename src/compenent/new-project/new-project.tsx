import { FC, ReactElement } from "react";
import { useDispatch } from "react-redux";
import { navigate, RouteType } from "../../state/features/navigationSlice";
import CaptureInput from "../capture-input/capture-input";

import './newproject.css'

interface IProps {
    workspace: string
}
  

const NewProject: FC = (props): ReactElement => {

    const dispatch = useDispatch()
    
    return (
        <div className="new-project" >
            <button className="btn" onClick={()=> dispatch(navigate(RouteType.Home))}>Home</button>
            <div className="left-area">
                <CaptureInput/>
            </div>
            <div className="right-area"></div>
        </div>
    );
  };
  
  export default NewProject;


  