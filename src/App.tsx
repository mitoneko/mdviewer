import { invoke } from "@tauri-apps/api/core";
import { Container, Box, IconButton, Wrap} from "@yamada-ui/react";
import { MdOutlineFileOpen } from "react-icons/md";

import Contents from "./Contents";

export default function App() {

  return (
      <Container width="100dvw" height="100dvh" margin="0" 
          padding="0" gap="0" overflow="hidden">
          <Container bg="#FFBA84" height="4em" gap="0" padding="0">
              <Toolbar />
          </Container>
          <Box height="calc(100% - 4em)"  padding="1em" overflowY="auto">
              <Contents />
          </Box>
      </Container>
  );
}

function Toolbar() {
    const handleOpenFile = async () => {
        console.log("Open file clicked");
        await invoke("choose_file");
    }

    return (
        <Wrap align="center" paddingX="0.5em" marginY="auto">
            <IconButton variant="outline" icon={<MdOutlineFileOpen />} size="md"
                onClick={handleOpenFile} />
        </Wrap>
    );
}
