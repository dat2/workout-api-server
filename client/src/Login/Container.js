import { connect } from 'react-redux';

import Login from '.';
import { login } from '../session';

function mapStateToProps({ session: { loggedIn, error } }) {
  return {
    loggedIn,
    error
  };
}

export default connect(mapStateToProps, { login })(Login);
