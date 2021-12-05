import { FC, ReactElement } from "react";
import History from "../history/history";

import './appbody.css'



const AppBody: FC = (props): ReactElement => {
    
    return (
        <div className="app-body" >
            <div className="new-project">project</div>
            {/* <History/> */}
        </div>
    );
  };
  
  export default AppBody;
  