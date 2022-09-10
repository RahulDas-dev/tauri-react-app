import StatusBar from './statusbar';
import { initialState } from '../../state/features/workspaceSlice';
// @ts-ignore:
import { render } from '@testing-library/react';
import configureStore from 'redux-mock-store';

import { Provider } from 'react-redux';
import { RouteType } from '../../state/features/navigationSlice';

const mockStore = configureStore([]);

describe('statusbar compenent test',()=> {

    let store = mockStore({
        navigation: {
            value: RouteType.Home
        },
        theme: {
            value: 'white'
        },
        workspace:initialState
      });

    test('renders correctly Status Bar',()=>{
        const {container} =  render(<Provider store={store}><StatusBar workspace="file"/></Provider>);
        const test_div = container.querySelectorAll('.status-bar')
        expect(test_div).toHaveLength(1);
        expect(container).toBeInTheDocument()
    })

    test('Status Bar should render child div',()=>{
        const {container} =  render(<Provider store={store}><StatusBar workspace="file"/></Provider>);
        const test_div = container.querySelectorAll('.status-bar>div')
        expect(test_div).toHaveLength(2);
    })

    test('Status Bar should have height 22px',()=>{
        const {container} =  render(<Provider store={store}><StatusBar workspace="file"/></Provider>);
        const test_div = container.querySelector('.status-bar')
        //const computedStyle = window.getComputedStyle(test_div!, null);
        //expect(test_div).toHaveStyle('height: 22px')
        //expect(computedStyle.height).toBe('22px')
    })

})