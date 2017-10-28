import React from 'react';
import { withFormik } from 'formik';

function login(body) {
  const headers = new Headers();
  headers.append('Content-Type', 'application/json');

  return fetch('/api/login', { method: 'POST', headers, body: JSON.stringify(body) });
}

function transformApiErrors(errors) {
  return {};
}

const InnerLoginForm = ({ errors, handleBlur, handleChange, handleSubmit, isSubmitting, touched, values }) => (
  <form onSubmit={handleSubmit}>
    <input
      name="username"
      onBlur={handleBlur}
      onChange={handleChange}
      required={true}
      type="text"
      value={values.username}
      />
    {touched.username && errors.username && <div>{errors.username}</div>}
    <input
      name="password"
      onBlur={handleBlur}
      onChange={handleChange}
      required={true}
      type="password"
      value={values.password}
      />
    {touched.password && errors.password && <div>{errors.password}</div>}
    <button type="submit"disabled={isSubmitting}>Login</button>
  </form>
);

const LoginForm = withFormik({
  mapPropsToValues: props => ({ username: '', password: '' }),
  validate(values, props) {
    const errors = {};
    if(!values.username) {
      errors.username = 'Required';
    }
    if(!values.password) {
      errors.password = 'Required';
    }
    return errors;
  },
  async handleSubmit(values, { props, setSubmitting, setErrors }) {
    try {
      const user = await login(values);
      props.login({ user });
    } catch(err) {
      setErrors(transformApiErrors(err));
    }
    setSubmitting(false);
  }
})(InnerLoginForm);

const Login = ({ login }) => (
  <div>
    <LoginForm login={login}/>
  </div>
);

export default Login;
