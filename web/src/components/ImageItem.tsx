import React from "react";
import LazyLoad from "react-lazyload";
import { AlbumItemProps } from "./AlbumItem";
import Image from "./Image";
import { getPhotoUrl } from "../util";
import { ActiveFileActions } from "../reducers/activeFileActions";
import { ImageSize } from "../constants";

function ImageItem({
    active,
    pathComponents,
    item,
    index,
    dispatch,
}: AlbumItemProps) {
    const src = getPhotoUrl(pathComponents, item, ImageSize.Medium);
    const largeSrc = getPhotoUrl(pathComponents, item, ImageSize.Large);

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
                    <img
                        src={largeSrc}
                        alt={item.name}
                        className="img-fullscreen"
                    />
                </div>
            )}
            <LazyLoad
                height="100%"
                offset={100}
                placeholder={<p>placeholder</p>}
            >
                <Image
                    src={src}
                    alt={item.name}
                    className="img-thumbnail invisible"
                    width={item.dimensions?.width}
                    height={item.dimensions?.height}
                />
            </LazyLoad>
        </li>
    );
}

export default ImageItem;
