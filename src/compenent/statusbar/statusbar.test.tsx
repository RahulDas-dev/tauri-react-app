import StatusBar from './statusbar';
import { render } from 'react-dom';


describe('statusbar compenent test',()=> {
    let container:HTMLDivElement ;
    
    beforeEach(()=>{
        container = document.createElement('div');
        document.body.appendChild(container);
        render(<StatusBar workspace="file"/>, container);
    })

    afterEach(()=>{
        document.body.removeChild(container);
        container.remove();
    })

    test('renders correctly Status Bar',()=>{
        const ststubardiv = container.querySelectorAll('.status-bar')
        expect(ststubardiv).toHaveLength(1);
    })

    test('Status Bar should render child div',()=>{
        const ststubardiv = container.querySelectorAll('.status-bar>div')
        expect(ststubardiv).toHaveLength(2);
    })

})