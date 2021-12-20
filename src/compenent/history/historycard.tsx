import { ReactElement, FC } from "react"
import {IWorkspace} from './workspace'
import './historycard.css'

interface IHistoryCard{
  data: IWorkspace
  type: 'even'|'odd'
}

export const HistoryCard: FC<IHistoryCard> = (props): ReactElement => {
  return (
    <div className={`history-card ${props.type}`}>
      <div className="card-header">
        <text>Workspace Id - <span className="mono-font">{props.data.project_id}</span></text>
        <div className="btn-group">
          <button className="btn-icon">
            <span className="material-icons-outlined md-18">mode_edit</span>
          </button>
          <button className="btn-icon">
            <span className="material-icons-outlined md-18">delete_outline</span>
          </button>
        </div>
      </div>
      <div className="card-body">
        <div className="card-item">
          <text>STATUS  - <span className="mono-font">{props.data.status}</span></text>
          <text>UPDATED ON - <span className="mono-font">{props.data.updated_on}</span></text>
        </div>   
        <div className="card-item1">
          <text>SETTING ID - <span className="mono-font">{props.data.setting_id}</span></text>
        </div>
        <div className="card-item1">
          <text>INPUT DIRECTORY - <span className="mono-font">{props.data.input_directory}</span></text>
        </div>
        <div className="card-item1">
          <text>OUTPUT DIRECTORY - <span className="mono-font">{props.data.output_directory}</span></text>
        </div>
      </div>
    </div>
  );
};