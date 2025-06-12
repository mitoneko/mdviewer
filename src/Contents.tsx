import { invoke } from "@tauri-apps/api/core";
import parse from "html-react-parser";
import { useQuery } from "@tanstack/react-query";

import "./markdown.css";

// マークダウンファイル本体の表示
export default function Contents() {
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

