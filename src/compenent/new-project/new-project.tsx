import { FC, ReactElement } from "react";

import './newproject.css'

interface IProps {
    workspace: string
}
  

const NewProject: FC = (props): ReactElement => {
    
    return (
        <div className="new-projecct" >new Project
        </div>
    );
  };
  
  export default NewProject;
  