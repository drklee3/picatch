import { useEffect, useState } from "react";
import { PathComponents, DirectoryFile } from "../types";

function useActiveFile(path: PathComponents, files: DirectoryFile[]) {
    const [activeFileIndex, setActiveFileIndex] = useState(-1);

    useEffect(() => {
        if (path.file && files.length > 0) {
            const i = files.findIndex((e) => e.name === path.file);
            setActiveFileIndex(i);
        }
    }, [path, files]);

    return { activeFileIndex };
}

export default useActiveFile;
