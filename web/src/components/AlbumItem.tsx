import React from "react";
import { ActiveFileActions } from "../reducers/activeFileActions";
import { getPhotoUrl } from "../util";
import LazyLoad from "react-lazyload";
import Image from "./Image";
import { ImageSize } from "../constants";
import { PathComponents, DirectoryAlbum } from "../types";
import { ActiveFileActionTypes } from "../reducers/activeFileActions";

interface AlbumItemProps {
    pathComponents: PathComponents;
    album: DirectoryAlbum;
    dispatch: React.Dispatch<ActiveFileActionTypes>;
}

function AlbumItem({ pathComponents, album, dispatch }: AlbumItemProps) {
    const src = getPhotoUrl(pathComponents, album, ImageSize.Medium);

    // TODO: Fix this, redirects to / right after click
    function goToAlbum() {
        let newAlbumPath = pathComponents.album + album.name;

        if (newAlbumPath.length > 0 && !newAlbumPath.startsWith("/album/")) {
            newAlbumPath = "/album" + newAlbumPath;
        }

        dispatch({
            type: ActiveFileActions.SET_ALBUM,
            album: newAlbumPath,
        });
    }

    return (
        <div className="album-wrapper" onClick={goToAlbum}>
            <div className="album-text">
                <h4 className="album-name">{album.name.slice(0, -1)}</h4>
                <p className="album-description">{album.info?.description}</p>
            </div>
            <LazyLoad
                height="100%"
                offset={100}
                overflow={true}
                placeholder={<div className="album-placeholder" />}
            >
                <div className="album-thumbnail-background" />
                <Image
                    src={src}
                    alt={album.name}
                    className="album-thumbnail invisible"
                />
            </LazyLoad>
        </div>
    );
}

export default AlbumItem;
