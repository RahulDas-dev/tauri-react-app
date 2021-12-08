import React from 'react';
import configureStore from 'redux-mock-store';
import { Provider } from 'react-redux';
import { RouteType } from './state/features/navigationSlice'; 

import App from './App';
import { render } from 'react-dom';


const mockStore = configureStore([]);

describe('App compenent test with Navigation state HOME',()=> {
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
        render( <Provider store={store}><App/></Provider>, container);
    })

    afterEach(()=>{
        document.body.removeChild(container);
        container.remove();
    })

    test('.app-container class should render once',()=>{
        const ststubardiv = container.querySelectorAll('.app-container')
        expect(ststubardiv).toHaveLength(1);
    })

    test('.app-container>.app-body class should render once',()=>{
      const ststubardiv = container.querySelectorAll('.app-container>.app-body')
      expect(ststubardiv).toHaveLength(1);
    })

    test('.app-container>.app-body>.app-home should render onec',()=>{
      const ststubardiv = container.querySelectorAll('.app-container>.app-body>.app-home')
      expect(ststubardiv).toHaveLength(1);
    })

    test('.app-container>.app-body>.new-project should not be render',()=>{
      const ststubardiv = container.querySelectorAll('.app-container>.app-body>.new-project')
      expect(ststubardiv.length).toBe(0)
    })

    test('.app-home class should render onec',()=>{
      const ststubardiv = container.querySelectorAll('.app-home')
      expect(ststubardiv).toHaveLength(1);
    })

    test('.app-container>.status-bar class should render once',()=>{
      const ststubardiv = container.querySelectorAll('.app-container>.status-bar')
      expect(ststubardiv).toHaveLength(1);
    })

    test('.status-bar class should render onec',()=>{
      const ststubardiv = container.querySelectorAll('.status-bar')
      expect(ststubardiv).toHaveLength(1);
    })

})

describe('App compenent test with Navigation state NEW_PROJECT',()=> {
  let container:HTMLDivElement ;
  let store;

  beforeEach(()=>{
      container = document.createElement('div');
      document.body.appendChild(container);
      store = mockStore({
        navigation: {
          value: RouteType.NewProject
        }
      });
      render( <Provider store={store}><App/></Provider>, container);
  })

  afterEach(()=>{
      document.body.removeChild(container);
      container.remove();
  })

  test('.app-container>.app-body>.new-project should render onec',()=>{
    const ststubardiv = container.querySelectorAll('.app-container>.app-body>.new-project')
    expect(ststubardiv).toHaveLength(1);
  })

  test('.app-container>.app-body>.app-home should not be render',()=>{
    const ststubardiv = container.querySelectorAll('.app-container>.app-body>.app-home')
    expect(ststubardiv.length).toBe(0)
  })

})