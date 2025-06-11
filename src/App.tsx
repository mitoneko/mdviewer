import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Container, Box } from "@yamada-ui/react";
import parse from "html-react-parser";
import { useQuery } from "@tanstack/react-query";

import "./markdown.css";

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
    const { data: contents, isPending } = useQuery({
        queryKey: ["contents"],
        queryFn: async (): Promise<string> => {
            return await invoke<string>("contents");
        },
    });

    if (isPending) {
        return ( <p>Loading...</p> );
    }

    return (
        <div className="markdown">
            {parse(contents??"")}
        </div>
    );
}

