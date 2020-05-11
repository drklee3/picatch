import React from "react";
import { ActiveFileActions } from "../reducers/activeFileActions";
import { AlbumItemProps } from "./AlbumItem";

function DirItem({ pathComponents, item, dispatch }: AlbumItemProps) {
    // TODO: Fix this, redirects to / right after click
    function goToAlbum() {
        let newAlbumPath = pathComponents.album + item.name;

        if (newAlbumPath.length > 0 && !newAlbumPath.startsWith("/album/")) {
            newAlbumPath = "/album" + newAlbumPath;
        }

        dispatch({
            type: ActiveFileActions.SET_ALBUM,
            album: newAlbumPath,
        });
    }

    return (
        <li className={`img-wrapper`}>
            <p>{pathComponents.album + item.name}</p>
            <button onClick={() => goToAlbum()}>Album: {item.name}</button>
        </li>
    );
}

export default DirItem;
