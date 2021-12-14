import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export interface IWorkSpace {
    project_id: number,
    status: string,
    input_directory: string,
    output_directory: string
}

interface IWorkSpaceState {
    value: IWorkSpace
}

const initialState: IWorkSpaceState = {
    value: {
        project_id: -1,
        status: "",
        input_directory: "",
        output_directory: ""
    }
}

export const workSpaceSlice = createSlice({
    name: "workspace",
    initialState: initialState,
    reducers: {
        setProjectId: (state: IWorkSpaceState, action: PayloadAction<number>) => {
            state.value.project_id = action.payload;
        },
        setStatus: (state: IWorkSpaceState, action: PayloadAction<string>) => {
            state.value.status = action.payload;
        },
        setInputDir: (state: IWorkSpaceState, action: PayloadAction<string>) => {
            state.value.input_directory = action.payload;
        },
        setOutputDir: (state: IWorkSpaceState, action: PayloadAction<string>) => {
            state.value.output_directory = action.payload;
        },

    }
})

export const { setProjectId, setStatus, setInputDir, setOutputDir } = workSpaceSlice.actions

export default workSpaceSlice.reducer
