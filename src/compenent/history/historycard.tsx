import { ReactElement, FC } from "react"
import {IWorkspace} from './workspace'


interface IHistoryCard{
  data: IWorkspace
}

export const HistoryCard: FC<IHistoryCard> = (props): ReactElement => {
  return (
    <div className="history-card">
      <div className="card-header">{props.data.created_on} {props.data.status}</div>
      <div className="card-body">
        
        {props.data.updated_on}
        
        {props.data.setting_id}
        {props.data.project_id}
        {props.data.input_directory}
        {props.data.output_directory}
      </div>
    </div>
  );
};