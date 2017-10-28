import React from 'react';
import { withFormik } from 'formik';

import { Container, Form, Input, Message, Button } from './LoginStyles';

function login(body) {
  const headers = new Headers();
  headers.append('Content-Type', 'application/json');

  return fetch('/api/login', { method: 'POST', headers, body: JSON.stringify(body) });
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
      const user = await login(values);
      props.login({ user });
    } catch(err) {
      setErrors(transformApiErrors(err));
    }
    setSubmitting(false);
  }
})(InnerLoginForm);

const Login = ({ login }) => (
  <Container>
    <LoginForm login={login}/>
  </Container>
);

export default Login;
