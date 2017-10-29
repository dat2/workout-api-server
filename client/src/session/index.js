import { get, post } from 'redux-saga-fetch';

const FETCH_USER_SUCCESS = 'FETCH_USER_SUCCESS';
const FETCH_USER_FAILED = 'FETCH_USER_FAILED';

export function register(body) {
  return post.json('/api/register', {
    success: response => {
      switch(response.status) {
        case 200:
          return response.json().then(fetchUserSuccess);
        case 500:
          return response.json().then(fetchUserFailed);
        default:
          return fetchUserFailed();
      }
    },
    fail: fetchUserFailed,
    body,
    credentials: 'include'
  });
}

export function login(body, cb) {
  return post.json('/api/login', {
    success: response => {
      switch(response.status) {
        case 200:
          return response.json().then(user => {
            cb(null);
            return fetchUserSuccess(user);
          });
        case 500:
          return response.json().then(error => {
            cb(error);
            return fetchUserFailed(error);
          });
        default:
          cb(true);
          return fetchUserFailed();
      }
    },
    fail(err) {
      cb(err);
      return fetchUserFailed(err);
    },
    body,
    credentials: 'include'
  });
}

export function fetchUser() {
  return get('/api/me', {
    success: response => response.status === 200 ?
      response.json().then(fetchUserSuccess) :
      fetchUserFailed(),
    fail: fetchUserFailed,
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include'
  });
}

export function fetchUserSuccess(user) {
  return {
    type: FETCH_USER_SUCCESS,
    payload: { user },
    error: null
  };
}

export function fetchUserFailed(error) {
  return {
    type: FETCH_USER_FAILED,
    error
  };
}

const initialState = {
  loggedIn: false,
  user: null,
  error: null
};

export default function reducer(state = initialState, action) {
  switch (action.type) {
    case FETCH_USER_SUCCESS:
      return {
        ...state,
        loggedIn: true,
        user: action.payload.user,
        error: null
      };
    case FETCH_USER_FAILED:
      return {
        ...state,
        loggedIn: false,
        user: null,
        error: action.error
      };
    default:
      return state;
  }
}
