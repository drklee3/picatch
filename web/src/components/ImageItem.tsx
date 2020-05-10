import React, { useEffect } from "react";
import LazyLoad from "react-lazyload";
import { AlbumItemProps } from "./AlbumItem";
import { getPhotoUrl } from "../util";
import { ActiveFileActions } from "../reducers/activeFileReducer";

function ImageItem({
    active,
    pathComponents,
    item,
    index,
    dispatch,
    activeFileState,
}: AlbumItemProps) {
    const src = getPhotoUrl(pathComponents, item);

    /*
    useEffect(() => {
        // Hey I'm the current file but the index is wrong so let me update it
        // Seems kind of inefficient? Listening to each of these for every image, every update
        // Only needs to be run when an image is loaded directly the **first** time but this
        // runs every single time the active image is changed
        console.log("Checking index");
        if (
            activeFileState.index === -1 &&
            activeFileState.name === item.name
        ) {
            dispatch({ type: ActiveFileActions.SET_INDEX, index });
            console.log("Updated missing index");
        }
    }, [activeFileState, index, dispatch, item.name]);
    */

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
