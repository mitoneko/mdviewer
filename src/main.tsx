import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { UIProvider, extendTheme, UIStyle } from "@yamada-ui/react";

const globalStyle: UIStyle = {
    body: {
        bg: "#ECB88A",
    },
}

const customTheme = extendTheme({ styles: { globalStyle } })()

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
      <UIProvider theme={customTheme} >
        <App />
    </UIProvider>
  </React.StrictMode>,
);
