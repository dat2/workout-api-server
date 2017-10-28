import { connect } from 'react-redux';

import Login from './Login';
import { login } from '../session/session';

export default connect(undefined, { login })(Login);
