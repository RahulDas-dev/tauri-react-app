import { FC, ReactElement, useState, MouseEvent, useReducer } from "react";
import {IWorkspace} from'./workspace';

import './history.css'
import { HistoryCard } from "./historycard";
import { invoke } from "@tauri-apps/api/tauri";


interface IState{
    expanded: boolean;
    historyList: IWorkspace[]|[],
}

interface IAction{
    type:'expand-more'|'expand-less',
    payload: IState
}

function historyReducer(state:IState, action:IAction){
    switch(action.type){
        case 'expand-more':
            return {expanded: true, historyList: action.payload.historyList}
        case 'expand-less':
            return  {expanded: false, historyList: []}           
    }
}

const History: FC = (props): ReactElement => {

    const [state, dispatchHistory ] = useReducer(historyReducer,{expanded: false, historyList:[]});
    const exapndHistory: (event:MouseEvent<HTMLButtonElement>)=> void =(event) =>{
        if (state.expanded === false){
            invoke<IWorkspace[]>('plugin:database|get_work_history')
            .then((result)=> { dispatchHistory({ 
                                type:'expand-more', 
                                payload:{ 
                                    expanded: true,
                                    historyList: result
                                }
                            })
            })
            .catch((e)=> console.error(e))
        } else{
            dispatchHistory({ 
                type:'expand-less', 
                payload:{ 
                    expanded: false,
                    historyList: []
                }
            })
        } 
    }
    
    return (
        <div className="history" >
            <div className="history-header">
                <span className="history-title">Work History</span>
                <button className="btn-icon" onClick={exapndHistory}>
                    {!state.expanded && <span className="material-icons-outlined">expand_more</span>}
                    {state.expanded && <span className="material-icons-outlined">expand_less</span>}
                </button>
            </div>
            <div className="history-list">
                {state.historyList.map((data:IWorkspace )=>
                    <HistoryCard data={data}/>)}
            </div>
        </div>
    );
  };
  
  export default History;
  