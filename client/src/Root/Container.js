import { connect } from 'react-redux';

import Root from './';

function mapStateToProps({ session: { loggedIn } }) {
  return {
    loggedIn
  };
}

export default connect(mapStateToProps)(Root);
