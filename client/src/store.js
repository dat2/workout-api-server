import { createStore, applyMiddleware, compose } from 'redux';
import { composeWithDevTools } from 'redux-devtools-extension';

import reducer from './root';

export default function configureStore() {
  const initialState = undefined;

  const composeEnhancers =
    process.env.NODE_ENV === 'development' ? composeWithDevTools : compose;

  const store = createStore(
    reducer,
    initialState,
    composeEnhancers(applyMiddleware())
  );

  return store;
}
