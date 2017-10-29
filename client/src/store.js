import { createStore, applyMiddleware, compose } from 'redux';
import { composeWithDevTools } from 'redux-devtools-extension';
import createSagaMiddleware from 'redux-saga';

import reducer from './root';
import { saga as fetchSaga } from 'redux-saga-fetch';

export default function configureStore() {
  const initialState = undefined;

  const composeEnhancers =
    process.env.NODE_ENV === 'development' ? composeWithDevTools : compose;

  const sagaMiddleware = createSagaMiddleware();

  const store = createStore(
    reducer,
    initialState,
    composeEnhancers(applyMiddleware(sagaMiddleware))
  );

  sagaMiddleware.run(fetchSaga);

  return store;
}
