import React from 'react';
import { withFormik } from 'formik';
import { Redirect } from 'react-router';
import { compose, withState } from 'recompose';
import Yup from 'yup';

import {
  Container,
  Form,
  Input,
  InputGroup,
  Label,
  Message,
  Button
} from '../Forms/Styles';

const InnerLoginForm = ({
  errors,
  handleBlur,
  handleChange,
  handleSubmit,
  isSubmitting,
  message,
  touched,
  values
}) => (
  <Form onSubmit={handleSubmit}>
    <InputGroup>
      <Label htmlFor="username">Username</Label>
      <Input
        name="username"
        onBlur={handleBlur}
        onChange={handleChange}
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
    <Button type="submit" disabled={isSubmitting}>
      Login
    </Button>
    {message && <Message>{message}</Message>}
  </Form>
);

const LoginForm = compose(
  withState('message', 'setMessage', ''),
  withFormik({
    validationSchema: Yup.object().shape({
      username: Yup.string().required('Username is required!'),
      password: Yup.string().required('Password is required!')
    }),
    mapPropsToValues: props => ({ username: '', password: '' }),
    handleSubmit(values, { props, setSubmitting, setErrors }) {
      props.login(values, error => {
        setSubmitting(false);
        if(error) {
          props.setMessage(error.message);
        } else {
          props.setMessage('');
        }
      });
    }
  })
)(InnerLoginForm);

const Login = ({ error, login, loggedIn }) =>
  loggedIn ? (
    <Redirect to="/" />
  ) : (
    <Container>
      <LoginForm error={error} login={login} />
    </Container>
  );

export default Login;
