import React from 'react';
import { withFormik } from 'formik';
import { Redirect } from 'react-router';
import { validate } from 'email-validator';

import {
  Container,
  Form,
  Label,
  Input,
  InputGroup,
  Message,
  Button
} from '../Forms/Styles';

function registerApi(body) {
  const headers = new Headers();
  headers.append('Content-Type', 'application/json');

  return fetch('/api/register', {
    method: 'POST',
    headers,
    body: JSON.stringify(body),
    credentials: 'include'
  }).then(response => response.json());
}

function transformApiErrors(errors) {
  return {};
}

const InnerRegisterForm = ({
  errors,
  handleBlur,
  handleChange,
  handleSubmit,
  isSubmitting,
  touched,
  values
}) => (
  <Form onSubmit={handleSubmit}>
    <InputGroup>
      <Label htmlFor="email">Email</Label>
      <Input
        name="email"
        onBlur={handleBlur}
        onChange={handleChange}
        placeholder="user@example.com"
        required={true}
        type="email"
        value={values.email}
      />
    </InputGroup>
    {touched.email && errors.email && <Message>{errors.email}</Message>}
    <InputGroup>
      <Label htmlFor="username">Username</Label>
      <Input
        name="username"
        onBlur={handleBlur}
        onChange={handleChange}
        placeholder="username"
        required={true}
        type="text"
        value={values.username}
      />
    </InputGroup>
    {touched.username &&
      errors.username && <Message>{errors.username}</Message>}
    <InputGroup>
      <Label htmlFor="password">Password</Label>
      <Input
        name="password"
        onBlur={handleBlur}
        onChange={handleChange}
        required={true}
        type="password"
        value={values.password}
      />
    </InputGroup>
    {touched.password &&
      errors.password && <Message>{errors.password}</Message>}
    <InputGroup>
      <Label htmlFor="password2">Password</Label>
      <Input
        name="password2"
        onBlur={handleBlur}
        onChange={handleChange}
        required={true}
        type="password"
        value={values.password2}
      />
    </InputGroup>
    {touched.password2 &&
      errors.password2 && <Message>{errors.password2}</Message>}
    <Button type="submit" disabled={isSubmitting}>
      Register
    </Button>
  </Form>
);

const RegisterForm = withFormik({
  mapPropsToValues: props => ({
    email: '',
    username: '',
    password: '',
    password2: ''
  }),
  validate(values, props) {
    const errors = {};
    if (!values.email) {
      errors.email = 'Email is required!';
    } else if (!validate(values.email)) {
      errors.email = 'Email is not valid!';
    }
    if (!values.username) {
      errors.username = 'Username is required!';
    }
    if (!values.password) {
      errors.password = 'Password is required!';
    }
    if (!values.password2) {
      errors.password2 = 'Password is required!';
    }
    if (values.password !== values.password2) {
      errors.password2 = 'Passwords do not match!';
    }
    return errors;
  },
  async handleSubmit(
    { password2, ...values },
    { props, setSubmitting, setErrors }
  ) {
    try {
      const user = await registerApi(values);
      setSubmitting(false);
      props.login({ user });
    } catch (err) {
      setSubmitting(false);
      setErrors(transformApiErrors(err));
    }
  }
})(InnerRegisterForm);

const Register = ({ login, loggedIn }) =>
  loggedIn ? (
    <Redirect to="/" />
  ) : (
    <Container>
      <RegisterForm login={login} />
    </Container>
  );

export default Register;
