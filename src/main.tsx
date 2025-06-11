import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { UIProvider, extendTheme, UIStyle } from "@yamada-ui/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const globalStyle: UIStyle = {
    body: {
        bg: "#ECB88A",
    },
}

const customTheme = extendTheme({ styles: { globalStyle } })()

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
      <QueryClientProvider client={queryClient}>
          <UIProvider theme={customTheme} >
            <App />
          </UIProvider>
      </QueryClientProvider>
  </React.StrictMode>,
);
