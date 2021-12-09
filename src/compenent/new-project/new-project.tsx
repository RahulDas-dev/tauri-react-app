import { FC, ReactElement } from "react";

import './newproject.css'

interface IProps {
    workspace: string
}
  

const NewProject: FC = (props): ReactElement => {
    
    return (
        <div className="new-project" >
            <div className="inputs">
                <input type="file" />
            </div>
            <div className="settings"></div>
        </div>
    );
  };
  
  export default NewProject;
  