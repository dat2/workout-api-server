import { connect } from 'react-redux';

import Login from '.';
import { login } from '../session';

function mapStateToProps({ session: { loggedIn } }) {
  return {
    loggedIn
  };
}

export default connect(mapStateToProps, { login })(Login);
