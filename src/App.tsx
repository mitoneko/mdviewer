import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Container, Box } from "@yamada-ui/react";
import parse from "html-react-parser";

export default function App() {

  return (
      <Container width="100dvw" height="100dvh" margin="0" padding="0" gap="0">
          <Box height="4em" bg="#FFBA84"></Box>
          <Box height="calc(100% - 4em)" >
              <Contents />
          </Box>
      </Container>
  );
}

function Contents() {
    const contents = "<h1>工事中</h1> <p> ただいま、<strong>工事中</strong>です。 </p>";

    return (
        <>
            {parse(contents)}
        </>
    );
}

