import { FC, ReactElement, MouseEvent } from "react";

import './capture-input.css'

interface IProps {
    workspace: string
}
  

const CaptureInput: FC = (props): ReactElement => {

    const openFileDailog: (event:MouseEvent<HTMLButtonElement>)=> void =(event) =>{
        console.log("Button clicked ",(event.target as HTMLButtonElement).id )
    }
    
    return (
        <form className="project-input" >
            <div className="input-file">
                <input type="text" placeholder="Input Directory" readOnly/> 
                <button className="btn" id="input" type="button" onClick={openFileDailog}>Browse
                    <span className="material-icons p-5">insert_photo</span>
                </button>
            </div>
            <small  >The input directory should include a list of images</small>
            <br/>
            <div className="input-file">
                <input type="text" placeholder="Output Directory" readOnly/> 
                <button className="btn" id="output"  type="button" onClick={openFileDailog}>Browse
                    <span className="material-icons p-5">folder_open</span>  
                </button>
            </div>
            <small className="help-text" >The output directory should be empty</small>
        </form>
    );
  };
  
  export default CaptureInput;
  