import React from 'react';
import ReactDOM from 'react-dom';
import { Provider } from 'react-redux';

import configureStore from './store';
import Root from './Root/Container';
import registerServiceWorker from './registerServiceWorker';

ReactDOM.render(
  <Provider store={configureStore()}>
    <Root />
  </Provider>,
  document.getElementById('root')
);
registerServiceWorker();
