import { FC, ReactElement, MouseEvent, useReducer } from "react";
import { useDispatch } from "react-redux";
import { invoke } from '@tauri-apps/api/tauri'

import { Modal } from "../modal/modal";
import { navigate, RouteType } from "../../state/features/navigationSlice";
import { IWorkSpaceState, setWorkSpace } from "../../state/features/workspaceSlice";

import './capture-input.css'


interface IProps {
    onClose : () => void;
} 

interface IState{
    input_value :string,
    input_err_cls:string,
    input_err_msg: string,
    output_value :string,
    output_err_cls:string,
    output_err_msg: string,
}
interface IAction{
    type:'setInputValue'|'setInputError'|'ResetInputError'|'setOutputValue'|'setOutputError'|'ResetOutputError',
    payload: string
}

const initialState: IState = {
    input_value :"",
    input_err_cls:"no-error",
    input_err_msg: "",
    output_value :"",
    output_err_cls: "no-error",
    output_err_msg: ""
};


function reducer(state:IState, action:IAction){
    switch(action.type){
        case 'setInputValue':
            return {...state, input_value: action.payload, input_err_cls: "no-error"}
        case 'setInputError':
            return {...state, input_err_msg: action.payload , input_err_cls: "error"}
        case 'ResetInputError':
                return {...state,input_value:"", input_err_msg: "" , input_err_cls: "no-error"}    
        case 'setOutputValue':
            return {...state, output_value: action.payload , output_err_cls: "no-error"}
        case 'setOutputError':
            return {...state, output_err_msg: action.payload , output_err_cls: "error"}
        case 'ResetOutputError':
                return {...state,output_value:"", output_err_msg: "" , output_err_cls: "no-error"}              
    }
}


const CaptureInput: FC<IProps> = (props): ReactElement => {

    const [state, dispatch ] = useReducer(reducer,initialState);
    const dispatchRedux = useDispatch()

    const openDailogInput: (event:MouseEvent<HTMLButtonElement>)=> void =(event) =>{
        (event.target as HTMLButtonElement).disabled = true
        dispatch({type:'ResetInputError', payload:""})
        invoke('plugin:dialog|open_input_dialog')
        .then((result)=> {
            dispatch({type:'setInputValue', payload: result as string})
        })
        .catch((error)=> {
            console.log(error)
            dispatch({type:'setInputError', payload: error as string})
        })
        .finally(()=>{
            (event.target as HTMLButtonElement).disabled = false
        })
    }

    const openDailogOutput: (event:MouseEvent<HTMLButtonElement>)=> void =(event) =>{
        (event.target as HTMLButtonElement).disabled = true
        dispatch({type:'ResetOutputError', payload:""})
        invoke('plugin:dialog|open_output_dialog')
        .then((result)=> {
            dispatch({type:'setOutputValue', payload: result as string})
        })
        .catch((error)=> {
            dispatch({type:'setOutputError', payload: error as string})
        })
        .finally(()=>{
            (event.target as HTMLButtonElement).disabled = false
        })
    }

    const enabled = state.input_value !=="" && state.output_value !=="";

    const createNewProject:(event:MouseEvent<HTMLButtonElement>)=> void =(event) =>{
        invoke<number[]>('plugin:database|add_workspace',
            {   
                inputDir:state.input_value, 
                outputDir: state.output_value
            }
        ).then((response: number[] ) => {
            const new_workspace: IWorkSpaceState = {
                project_id: response[1],
                status: "Active",
                input_directory: state.input_value,
                output_directory: state.output_value
            }
            dispatchRedux(setWorkSpace(new_workspace))
            dispatchRedux(navigate(RouteType.NewProject))
            console.log(response)
        }).catch((error) => {
            console.error(error)
        })
    }

    const cancel:(event:MouseEvent<HTMLButtonElement>)=> void =(event) =>{
        props.onClose()
    }   
    
    return (
        <Modal title="New Project">
            <form className="project-input" >
                <div className="input-box">
                    <div className="input-file">
                        <input type="text" placeholder="Select Input Directory" readOnly value={state.input_value}/> 
                        <button className="btn" type="button" onClick={openDailogInput}>Browse...
                            {/* <span className="material-icons p-5">insert_photo</span> */}
                        </button>
                    </div>
                    <small className={`${state.input_err_cls}`}>
                        { state.input_err_cls === 'error' &&  state.input_err_msg}
                        { state.input_err_cls === 'no-error' &&  "The input directory should include a list of images"}
                    </small>
                </div>
                <div className="input-box">    
                    <div className="input-file">
                        <input type="text" placeholder="Select Output Directory" readOnly value={state.output_value}/> 
                        <button className="btn" type="button" onClick={openDailogOutput}>Browse...
                            {/* <span className="material-icons p-5">folder_open</span> */}
                        </button>
                    </div>
                    <small className={`${state.output_err_cls}`}>
                        { state.output_err_cls === 'error' &&  state.output_err_msg}
                        { state.output_err_cls === 'no-error' &&  "The output directory should be empty"}
                    </small>
                </div>
                <div className="btn-group">
                    <button className="btn" type="button" disabled ={!enabled} onClick={createNewProject}>create</button>
                    <button className="btn" type="button" onClick={cancel}>close</button>
                </div>
            </form>
        </Modal>
    );
  };
  
  export default CaptureInput;


  