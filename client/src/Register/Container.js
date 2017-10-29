import { connect } from 'react-redux';

import Container from '.';
import { register } from '../session';

function mapStateToProps({ session: { loggedIn } }) {
  return {
    loggedIn
  };
}

export default connect(mapStateToProps, { register })(Container);
