import NewProject from './new-project';
// @ts-ignore:
import { render } from '@testing-library/react';
import configureStore from 'redux-mock-store';
import { Provider } from 'react-redux';
import { initialState } from '../../state/features/workspaceSlice';
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
        const {container} =  render( <Provider store={store}><NewProject/> </Provider>);
        const test_div = container.querySelectorAll('.new-project')
        expect(test_div).toHaveLength(1);
        expect(container).toBeInTheDocument()
    })

})