import { get } from 'redux-saga-fetch';

const LOGIN = 'LOGIN';
const LOGOUT = 'LOGOUT';

export function fetchUser() {
  return get('/api/me', {
    success: response => response.status === 200 ?
      response.json().then(login) :
      logout(),
    fail: logout,
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include'
  });
}

export function login(user) {
  return {
    type: LOGIN,
    payload: { user },
    error: null
  };
}

export function logout() {
  return {
    type: LOGOUT
  };
}

const initialState = {
  loggedIn: false,
  user: null
};

export default function reducer(state = initialState, action) {
  switch (action.type) {
    case LOGIN:
      return {
        ...state,
        loading: false,
        loggedIn: true,
        user: action.payload.user
      };
    case LOGOUT:
      return {
        ...state,
        loggedIn: false,
        user: null
      };
    default:
      return state;
  }
}
