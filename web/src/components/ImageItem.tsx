import React from "react";
import LazyLoad from "react-lazyload";
import { AlbumItemProps } from "./AlbumItem";
import { getPhotoUrl } from "../util";
import { ActiveFileActions } from "../reducers/activeFileActions";

function ImageItem({
    active,
    pathComponents,
    item,
    index,
    dispatch,
}: AlbumItemProps) {
    const src = getPhotoUrl(pathComponents, item);

    function updateActiveFile() {
        // If already active, set to none
        if (active) {
            dispatch({
                type: ActiveFileActions.SET_FILE,
                name: "",
                index: -1,
            });
            return;
        }

        dispatch({
            type: ActiveFileActions.SET_FILE,
            name: item.name,
            index: index,
        });
    }

    return (
        <li
            className={`img-wrapper ${active ? "active" : ""}`}
            onClick={updateActiveFile}
        >
            {active && (
                <div className="img-fullscreen-wrapper">
                    <img src={src} alt={item.name} className="img-fullscreen" />
                </div>
            )}
            <LazyLoad
                height="100%"
                offset={100}
                placeholder={<p>placeholder</p>}
            >
                <img
                    src={src}
                    alt={item.name}
                    className="img-thumbnail"
                    // width={item.dimensions?.width}
                    // height={item.dimensions?.height}
                />
            </LazyLoad>
        </li>
    );
}

export default ImageItem;