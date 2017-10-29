import React from 'react';
import { Redirect, Route, Switch } from 'react-router';
import { BrowserRouter, Link } from 'react-router-dom';
import { lifecycle } from 'recompose';

import Register from '../Register/Container';
import Login from '../Login/Container';
import Workout from '../Workout';

const App = () => (
  <div>
    <nav>
      <Link to="/">Workout</Link>
    </nav>
    <Switch>
      <Route path="/workout" component={Workout} />
    </Switch>
  </div>
);

const AuthenticatedRoute = ({
  authenticated,
  component: Component,
  ...rest
}) => (
  <Route
    {...rest}
    render={props =>
      authenticated ? (
        <Component {...props} />
      ) : (
        <Redirect
          to={{ pathname: '/login', state: { from: props.location } }}
        />
      )}
  />
);

const Root = ({ loggedIn }) => (
  <BrowserRouter>
    <div>
      <Route path="/register" component={Register} />
      <Route path="/login" component={Login} />
      <AuthenticatedRoute
        authenticated={loggedIn}
        exact
        path="/"
        component={App}
      />
    </div>
  </BrowserRouter>
);

const enhance = lifecycle({
  componentDidMount() {
    this.props.fetchUser();
  }
})

export default enhance(Root);
