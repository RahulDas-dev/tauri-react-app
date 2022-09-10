import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export interface IWorkSpaceState {
    project_id: number,
    status: string,
    input_directory: string,
    output_directory: string
}

/* export interface IWorkSpaceState {
    value: IWorkSpace
} */

export const initialState: IWorkSpaceState = {
    project_id: -1,
    status: "",
    input_directory: "",
    output_directory: ""

}

export const workSpaceSlice = createSlice({
    name: "workspace",
    initialState: initialState,
    reducers: {
        setProjectId: (state: IWorkSpaceState, action: PayloadAction<number>) => {
            state.project_id = action.payload;
        },
        setStatus: (state: IWorkSpaceState, action: PayloadAction<string>) => {
            state.status = action.payload;
        },
        setInputDir: (state: IWorkSpaceState, action: PayloadAction<string>) => {
            state.input_directory = action.payload;
        },
        setOutputDir: (state: IWorkSpaceState, action: PayloadAction<string>) => {
            state.output_directory = action.payload;
        },
        setWorkSpace: (state: IWorkSpaceState, action: PayloadAction<IWorkSpaceState>) => {
            state.project_id = action.payload.project_id;
            state.status = action.payload.status;
            state.input_directory = action.payload.input_directory;
            state.output_directory = action.payload.output_directory;
        },

    }
})

export const { setProjectId, setStatus, setInputDir, setOutputDir, setWorkSpace } = workSpaceSlice.actions

export default workSpaceSlice.reducer
