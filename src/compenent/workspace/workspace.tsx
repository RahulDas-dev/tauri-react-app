import { FC, ReactElement } from "react";

import './workspace.css'

interface IProps {
    workspace: string
}
  

const Workspace: FC = (props): ReactElement => {
    
    return (
        <div className="Workspace" >Workspace
        </div>
    );
  };
  
  export default Workspace;
  