import NewProject from './new-project';
import { render } from '@testing-library/react';


describe('statusbar compenent test',()=> {

    test('renders correctly Status Bar',()=>{
        const {container} =  render( <NewProject/>);
        const test_div = container.querySelectorAll('.new-project')
        expect(test_div).toHaveLength(1);
        expect(container).toBeInTheDocument()
    })

})