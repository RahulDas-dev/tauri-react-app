import { FC, ReactElement } from "react";
import { useSelector } from "react-redux";
import { RootState } from "../../state/store";

import './statusbar.css'

interface IProps {
    workspace: string
}
  

const StatusBar: FC<IProps> = (props): ReactElement => {

    const workspace = useSelector((state: RootState)=> state.workspace.value)

    
    return (
        <div className={`status-bar ${props.workspace ? 'has-workspace' : 'no-workspace'}`} >
            <div className="left">
                {
                   workspace.project_id > 0 &&
                   <label>
                        <span className="material-icons-outlined">label</span>
                        <span>Project ID: {workspace.project_id}</span>
                    </label>    
                }
            </div>
            <div className="right">right</div>
        </div>
    );
  };
  
  export default StatusBar;
  