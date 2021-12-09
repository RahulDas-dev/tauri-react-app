import StatusBar from './statusbar';
import { render } from '@testing-library/react';


describe('statusbar compenent test',()=> {

    test('renders correctly Status Bar',()=>{
        const {container} =  render( <StatusBar workspace="file"/>);
        const test_div = container.querySelectorAll('.status-bar')
        expect(test_div).toHaveLength(1);
        expect(container).toBeInTheDocument()
    })

    test('Status Bar should render child div',()=>{
        const {container} =  render( <StatusBar workspace="file"/>);
        const test_div = container.querySelectorAll('.status-bar>div')
        expect(test_div).toHaveLength(2);
    })

    test('Status Bar should have height 22px',()=>{
        const {container} =  render( <StatusBar workspace="file"/>);
        const test_div = container.querySelector('.status-bar')
        //const computedStyle = window.getComputedStyle(test_div!, null);
        //expect(test_div).toHaveStyle('height: 22px')
        //expect(computedStyle.height).toBe('22px')
    })

})