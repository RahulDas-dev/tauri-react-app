import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export enum RouteType {
    Home = 'HOME',
    NewProject = 'NEW_PROJECT',
    Settings = 'SETTINGS',
    Workspace = 'WORKSPACE'
}

interface INavigatorState {
    value: RouteType
}

const initialState: INavigatorState = { value: RouteType.Home }

export const navigationSlice = createSlice({
    name: "navigation",
    initialState: initialState,
    reducers: {
        navigate: (state, action: PayloadAction<RouteType>) => {
            state.value = action.payload;
        }
    }
})

export const { navigate } = navigationSlice.actions

export default navigationSlice.reducer
