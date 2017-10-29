import React from 'react';
import { withFormik } from 'formik';
import { Redirect } from 'react-router';
import Yup from 'yup';

import {
  Container,
  Form,
  Label,
  Input,
  InputGroup,
  Message,
  Button
} from '../Forms/Styles';

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

function equalTo(ref, message) {
  return this.test({
    name: 'equalTo',
    exclusive: false,
    message,
    params: {
      reference: ref.path
    },
    test(value) {
      return value === this.resolve(ref)
    }
  });
}

Yup.addMethod(Yup.string, 'equalTo', equalTo);

const RegisterForm = withFormik({
  validationSchema: Yup.object().shape({
    email: Yup.string().email().required('Email is required!'),
    username: Yup.string().required('Username is required!'),
    password: Yup.string().required('Password is required!'),
    password2: Yup.string().equalTo(Yup.ref('password'), 'Passwords do not match!')
  }),
  mapPropsToValues: props => ({
    email: '',
    username: '',
    password: '',
    password2: ''
  }),
  handleSubmit(
    { password2, ...values },
    { props, setSubmitting, setErrors }
  ) {
    props.register(values, error => {
      setSubmitting(false);
      setErrors(error);
    });
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
