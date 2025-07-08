import { useEffect } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
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
        const unlisten = listen("invalid_content", () => {
            queryClient.invalidateQueries({ queryKey: ["contents"] });
        });
        return () => {
            // クリーンアップ関数でイベントリスナーを解除
            unlisten.then(unliten => unliten());
        };
    }, []);

    // コンテンツが変更された時、表示をトップにスクロールする
    useEffect(() => {
        window.scrollTo(0, 0);
    }, [contents]);

    if (isPending) {
        return ( <p>Loading...</p> );
    }

    return (
        <div className="markdown">
            {parse(contents??"")}
        </div>
    );
}

