import React from 'react';
import { Redirect, Route, Switch } from 'react-router';
import { BrowserRouter, Link } from 'react-router-dom';

import Login from '../Login/LoginContainer';
import Workout from '../Workout/Workout';

const App = () => (
  <div>
    <nav>
      <Link to='/'>Workout</Link>
    </nav>
    <Switch>
      <Route path='/workout' component={Workout} />
    </Switch>
  </div>
);

const AuthenticatedRoute = ({ authenticated, component: Component, ...rest }) => (
  <Route {...rest} render={props => (
    authenticated ?
      <Component {...props} /> :
      <Redirect to={{ pathname: '/login', state: { from: props.location } }} />
  )} />
);

const Root = ({ loggedIn }) => (
  <BrowserRouter>
    <div>
      <p>LoggedIn:{ loggedIn.toString() }</p>
      <Route path='/login' component={Login} />
      <AuthenticatedRoute authenticated={loggedIn} exact path='/' component={App} />
    </div>
  </BrowserRouter>
);

export default Root;
