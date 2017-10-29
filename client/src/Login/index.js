import React from 'react';
import { withFormik } from 'formik';
import { Redirect } from 'react-router';

import { Container, Form, Input, Message, Button } from './Styles';

function loginApi(body) {
  const headers = new Headers();
  headers.append('Content-Type', 'application/json');

  return fetch('/api/login', { method: 'POST', headers, body: JSON.stringify(body) })
    .then(response => response.json());
}

function transformApiErrors(errors) {
  return {};
}

const InnerLoginForm = ({ errors, handleBlur, handleChange, handleSubmit, isSubmitting, touched, values }) => (
  <Form onSubmit={handleSubmit}>
    <Input
      name="username"
      onBlur={handleBlur}
      onChange={handleChange}
      required={true}
      type="text"
      value={values.username}
      />
    {touched.username && errors.username && <Message>{errors.username}</Message>}
    <Input
      name="password"
      onBlur={handleBlur}
      onChange={handleChange}
      required={true}
      type="password"
      value={values.password}
      />
    {touched.password && errors.password && <Message>{errors.password}</Message>}
    <Button type="submit"disabled={isSubmitting}>Login</Button>
  </Form>
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
      const user = await loginApi(values);
      setSubmitting(false);
      props.login({ user });
    } catch(err) {
      setSubmitting(false);
      setErrors(transformApiErrors(err));
    }
  }
})(InnerLoginForm);

const Login = ({ login, loggedIn }) => (
  loggedIn ?
    <Redirect to='/' /> :
    <Container>
      <LoginForm login={login} />
    </Container>
);

export default Login;
