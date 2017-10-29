import styled from 'styled-components';
import { lighten, darken } from 'polished';

export const Container = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
`;

export const Form = styled.form`
  display: flex;
  flex-direction: column;
  justify-content: center;
  font-family: 'Roboto', sans-serif;
`;

export const Input = styled.input`
  display: inline-block;
  padding: 5px;
  border: 1px solid ${lighten(0.2, 'grey')};
  outline: none;

  &:focus {
    border-color: ${darken(0.2, 'grey')};
    box-shadow: rgba(0, 0, 0, 0.1) 0px 0px 16px;
  }
`;

export const InputGroup = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin: 5px;
  font-family: inherit;
`;

export const Label = styled.label`
  display: inline-block;
  margin-right: 5px;
`;

export const Message = styled.div`
  display: block;
  padding: 5px;
  margin: 5px;
  border-radius: 3px;
  border: 1px solid red;
  background-color: ${lighten(0.4, 'red')};
  color: red;
`;

export const Button = styled.button`
  display: block;
  padding: 5px;
  margin: 5px;
  border: none;
  background-color: ${darken(0.05, 'white')};
  transition: all 0.5s ease;
  &:hover {
    cursor: pointer;
    background-color: ${darken(0.1, 'white')};
  }
`;
