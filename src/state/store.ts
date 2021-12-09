import { configureStore } from "@reduxjs/toolkit"

import navigationReducer from './features/navigationSlice'
import themeSlice from "./features/themeSlice"


export const store = configureStore({
    reducer: {
        navigation: navigationReducer,
        theme: themeSlice
    }
})

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch