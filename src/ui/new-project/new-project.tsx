import { FC, ReactElement } from "react";
import { useDispatch } from "react-redux";
import { navigate, RouteType } from "../../state/features/navigationSlice";

import { GridImage } from "../../compenent/gridimage/gridimage";

import './newproject.css'


  

const NewProject: FC = (props): ReactElement => {

    const dispatch = useDispatch()
    const img_path="D:/python/Structure-from-Motion/dataset/box/IMG_20200328_173114.jpg"
    
    return (
        <div className="new-project" >
            <button className="btn" onClick={()=> dispatch(navigate(RouteType.Home))}>Home</button>
            <div className="left-area">
                {/* <CaptureInput/> */}
            </div>
            <div className="right-area">
                <GridImage source={img_path} width={200}/>
                <GridImage source={img_path} width={200}/>
                <GridImage source={img_path} width={200}/>
                <GridImage source={img_path} width={200}/>
                <GridImage source={img_path} width={200}/>
                {/* <GridImage source={img_path} width={200}/>
                <GridImage source={img_path} width={200}/>
                <GridImage source={img_path} width={200}/>
                <GridImage source={img_path} width={200}/> */}
            </div>
        </div>
    );
  };
  
  export default NewProject;


  