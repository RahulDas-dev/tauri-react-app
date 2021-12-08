import configureStore from 'redux-mock-store';
import { Provider } from 'react-redux';
import { RouteType } from '../../state/features/navigationSlice'; 
import Home from './home';
import { render } from 'react-dom';

const mockStore = configureStore([]);

describe('statusbar compenent test',()=> {
    let container:HTMLDivElement ;
    let store;
    
    beforeEach(()=>{
        container = document.createElement('div');
        document.body.appendChild(container);
        store = mockStore({
            navigation: {
              value: RouteType.Home
            }
          });
        render(<Provider store={store}><Home/></Provider>, container);
    })

    afterEach(()=>{
        document.body.removeChild(container);
        container.remove();
    })

    test('renders correctly Home Compeonent',()=>{
        const ststubardiv = container.querySelectorAll('.app-home')
        expect(ststubardiv).toHaveLength(1);
    })

    test('Home Compeonent 3 buttons',()=>{
        const ststubardiv = container.querySelectorAll('.app-home button')
        expect(ststubardiv).toHaveLength(3);
    })

})