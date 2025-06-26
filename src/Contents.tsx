import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import parse from "html-react-parser";
import { useQuery, useQueryClient } from "@tanstack/react-query";

import "./markdown.css";

// マークダウンファイル本体の表示
export default function Contents() {
    const queryClient = useQueryClient();

    const { data: contents, isPending } = useQuery({
        queryKey: ["contents"],
        queryFn: async (): Promise<string> => {
            return await invoke<string>("contents");
        },
    });

    // Rust側でコンテンツの変更通知があった時、コンテンツを再取得する。
    useEffect(() => {
        let unlisten: UnlistenFn;
        async function f() {
            unlisten = await listen("invalid_content", () => {
                queryClient.invalidateQueries({ queryKey: ["contents"] });
            });
        };
        f();
        return () => {
            if (unlisten) {
                unlisten();
            }
        };
    }, []);

    if (isPending) {
        return ( <p>Loading...</p> );
    }

    return (
        <div className="markdown">
            {parse(contents??"")}
        </div>
    );
}

