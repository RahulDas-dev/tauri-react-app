import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export enum ThemeType {
    BLACK = 'black',
    WHITE = 'white',
}

interface IThemwState {
    value: ThemeType
}

const initialState: IThemwState = { value: ThemeType.BLACK }

export const themeSlice = createSlice({
    name: "theme",
    initialState: initialState,
    reducers: {
        setTheme: (state, action: PayloadAction<ThemeType>) => {
            state.value = action.payload;
        },
        setBlackTheme: (state) => {
            state.value = ThemeType.BLACK;
        },
        setWhiteTheme: (state) => {
            state.value = ThemeType.WHITE;
        },
        resetTheme: (state) => {
            state.value = ThemeType.WHITE;
        }

    }
})

export const { setTheme, setBlackTheme, setWhiteTheme, resetTheme } = themeSlice.actions

export default themeSlice.reducer
