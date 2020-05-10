import React from "react";
import { useHistory } from "react-router";
import { AlbumItemProps } from "./AlbumItem";

function DirItem(props: AlbumItemProps) {
    const history = useHistory();

    // TODO: Fix this, redirects to / right after click
    function goToAlbum() {
        let newAlbumPath = props.pathComponents.album + props.item.name;

        if (newAlbumPath.length > 0 && !newAlbumPath.startsWith("/album/")) {
            newAlbumPath = "/album" + newAlbumPath;
        }

        console.log("navigating to new album:", newAlbumPath);
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
