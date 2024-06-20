import { styled } from "@linaria/react";
import { useState } from "react";

const Button = styled.button`
  color: red;
`;

export default function Index() {
  return <Button>Hello, Next.js!</Button>;
}
