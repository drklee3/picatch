import React from "react";
import { useHistory } from "react-router";
import { PathComponents, DirectoryItem, ActiveFile } from "../types";

type DirItemProps = {
    pathComponents: PathComponents;
    item: DirectoryItem;
    index: number;
    activeFile: ActiveFile;
    setActiveFile: React.Dispatch<React.SetStateAction<ActiveFile>>;
};

function DirItem(props: DirItemProps) {
    const history = useHistory();

    // TODO: Fix this, redirects to / right after click
    function goToAlbum() {
        let newAlbumPath = props.pathComponents.album + props.item.name;

        if (newAlbumPath.length > 0 && !newAlbumPath.startsWith("/album/")) {
            newAlbumPath = "/album" + newAlbumPath;
        }

        console.log("newAlbumPath:", newAlbumPath);
        history.push(newAlbumPath);
    }

    return (
        <li className={`img-wrapper`}>
            <p>{props.pathComponents.album + props.item.name}</p>
            <button onClick={() => goToAlbum()}>
                Album: {props.item.name}
            </button>
        </li>
    );
}

export default DirItem;
