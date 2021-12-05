import { FC, ReactElement } from "react";

import './statusbar.css'

interface IProps {
    workspace: string
}
  

const StatusBar: FC<IProps> = (props): ReactElement => {
    
    return (
        <div className={`status-bar ${props.workspace ? 'has-workspace' : 'no-workspace'}`} >
            <div className="left">left</div>
            <div className="right">right</div>
        </div>
    );
  };
  
  export default StatusBar;
  