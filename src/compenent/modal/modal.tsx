import { FC, Fragment, ReactElement } from "react";
import type {ReactNode} from 'react'; // ok
import { createPortal } from "react-dom";
import { useSelector } from "react-redux";
import { ThemeType } from "../../state/features/themeSlice";
import { RootState } from "../../state/store";

import './modal.css'


const Backdrop: FC = (): ReactElement => {
  const theme = useSelector((state: RootState)=> state.theme.value)
  return (
      <div className={`backdrop ${theme === ThemeType.WHITE ? 'white' : 'black'}`} ></div>
  );
};

export interface IModalProps{
  title: string,
  children: ReactNode,
}

const ModalOverlay: FC<IModalProps> = (props): ReactElement => {
  const theme = useSelector((state: RootState)=> state.theme.value)
  
  return (
      <div className={`modal ${theme === ThemeType.WHITE ? 'white' : 'black'}`} >
        <div className="modal-header">
          <div className="modal-title">{props.title}</div>
          {/* <button className="btn-icon"><span className="material-icons-outlined">clear</span></button> */}
        </div>
        <div className="modal-contain">{props.children}</div>
      </div>
  );
};

const portalElement = document.getElementById('portal') as HTMLDivElement;

export const Modal: FC<IModalProps> = (props): ReactElement => {

  return (
      <Fragment>
        {createPortal(<Backdrop/>,portalElement )}
        {
          createPortal(
            <ModalOverlay title={props.title}>{props.children}</ModalOverlay>,portalElement
          )
        }
      </Fragment>
  )
};
