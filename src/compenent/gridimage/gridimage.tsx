import { convertFileSrc } from "@tauri-apps/api/tauri";
import { FC, ReactElement } from "react";
import './gridimage.css'

export interface ImageProps{
  source: string,
  width: number
}

export const GridImage: FC<ImageProps> = (props): ReactElement => {

  return ( 
    <div className="img-container">
      <img  src={convertFileSrc(props.source)} 
            width={props.width}
            alt={`${props.source.split('/').pop()}`} />
      <div className="img-overlay">
          <div className="tools">
            <button className="btn-icon"><span className="material-icons ">compare</span></button>
            <button className="btn-icon"><span className="material-icons ">crop</span></button>
            <button className="btn-icon"><span className="material-icons ">delete_outline</span></button>  
          </div>
          <div className="image-name">
            {props.source.split('/').pop()}  
          </div> 
      </div>
           
    </div>
  );

};

