import { connect } from 'react-redux';

import Root from './';
import { fetchUser } from '../session';

function mapStateToProps({ session: { loading, loggedIn } }) {
  return {
    loading,
    loggedIn
  };
}

export default connect(mapStateToProps, { fetchUser })(Root);
