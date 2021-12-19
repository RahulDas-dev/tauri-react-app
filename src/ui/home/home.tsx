import { FC, ReactElement, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import History from "../../compenent/history/history";
import CaptureInput from "../../compenent/capture-input/capture-input";
import { navigate, RouteType } from "../../state/features/navigationSlice";

import './home.css'
import { RootState } from "../../state/store";


const Home: FC = (props): ReactElement => {

    const [isVisbile, setVisibility] = useState<boolean>(false);

    const shoWModal = () => setVisibility(true)
    const closeModal = () => setVisibility(false)

    const workspace = useSelector((state: RootState)=> state.workspace.value)

    const dispatch = useDispatch()
    
    return (
        <div className="app-home"  >
            <div className="home-left-section">
                <div className="btn-group">
                    <button className="btn width-12 " onClick = {shoWModal}>
                        <span className="material-icons">add</span> New Project
                    </button>
                    { workspace.project_id > 0 &&
                    <button className="btn width-12 " onClick = {()=> dispatch(navigate(RouteType.Workspace))}>
                        <span className="material-icons-outlined ">code</span> Current Workspace
                    </button>
                    }
                    <button className="btn width-12">
                        <span className="material-icons-outlined">format_list_bulleted</span> Read Document
                    </button>
                </div>
            </div>
            <div className="home-right-section">
                <History/>
            </div>
            {isVisbile && <CaptureInput onClose ={closeModal}/>}
        </div>
    );
  };
  
  export default Home;
  