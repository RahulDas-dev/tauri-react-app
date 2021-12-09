import { render } from '@testing-library/react';
import configureStore from 'redux-mock-store';
import { Provider } from 'react-redux';


import App from './App';
import { RouteType } from './state/features/navigationSlice'; 


const mockStore = configureStore([]);

describe('App compenent test with Navigation state HOME',()=> {
    let store = mockStore({
      navigation: {
        value: RouteType.Home
      },
      theme: {
        value: 'white'
      },
    });

    test('.app-container class should render once',()=>{
        const {container} =  render( <Provider store={store}><App/></Provider>);
        let test_div = container.querySelectorAll('.app-container')
        expect(test_div).toHaveLength(1);
        expect(container).toBeInTheDocument()
    })

    test('.app-container should have height & width 100%',()=>{
      const {container} =  render( <Provider store={store}><App/></Provider>);
      const test_div = container.querySelector('.app-container')
      const computedStyle = window.getComputedStyle(test_div!, null);
      const parentStyle = window.getComputedStyle(container, null);
      expect(computedStyle.height).toBe(parentStyle.height)
      expect(computedStyle.width).toBe(parentStyle.width)
    })

    test('.app-container>.app-body class should render once',()=>{
      const {container} =  render( <Provider store={store}><App/></Provider>);
      const test_div = container.querySelectorAll('.app-container>.app-body')
      expect(test_div).toHaveLength(1);
    })


    test('.app-container>.app-body>.app-home should render onec',()=>{
      const {container} =  render( <Provider store={store}><App/></Provider>);
      const test_div = container.querySelectorAll('.app-container>.app-body>.app-home')
      expect(test_div).toHaveLength(1);
    })

    test('.app-container>.app-body>.new-project should not be render',()=>{
      const {container} =  render( <Provider store={store}><App/></Provider>);
      const test_div = container.querySelectorAll('.app-container>.app-body>.new-project')
      expect(test_div.length).toBe(0)
    })

    test('.app-home class should render onec',()=>{
      const {container} =  render( <Provider store={store}><App/></Provider>);
      const test_div = container.querySelectorAll('.app-home')
      expect(test_div).toHaveLength(1);
    })

    test('.app-container>.status-bar class should render once',()=>{
      const {container} =  render( <Provider store={store}><App/></Provider>);
      const test_div = container.querySelectorAll('.app-container>.status-bar')
      expect(test_div).toHaveLength(1);
    })

    test('.status-bar class should render onec',()=>{
      const {container} =  render( <Provider store={store}><App/></Provider>);
      const test_div = container.querySelectorAll('.status-bar')
      expect(test_div).toHaveLength(1);
    })

})

describe('App compenent test with Navigation state NEW_PROJECT',()=> {
  let store = mockStore({
    navigation: {
      value: RouteType.NewProject
    },
    theme: {
      value: 'white'
    }
  });

  test('.app-container>.app-body>.new-project should render onec',()=>{
    const {container} =  render( <Provider store={store}><App/></Provider>);
    const test_div = container.querySelectorAll('.app-container>.app-body>.new-project')
    expect(test_div).toHaveLength(1);
  })

  test('.app-container>.app-body>.app-home should not be render',()=>{
    const {container} =  render( <Provider store={store}><App/></Provider>);
    const test_div = container.querySelectorAll('.app-container>.app-body>.app-home')
    expect(test_div.length).toBe(0)
  })

})