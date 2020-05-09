import React, { useState, useEffect } from "react";
import { RouteComponentProps } from "react-router";
import LazyLoad from "react-lazyload";
import { PathComponents, DirectoryItem } from "../types";
import { imageIsActive, getPhotoUrl } from "../util";

type ImageItemProps = {
    pathComponents: PathComponents;
    item: DirectoryItem;
    activeFile: string;
    setActiveFile: React.Dispatch<React.SetStateAction<string>>;
};

function ImageItem({
    pathComponents,
    item,
    activeFile,
    setActiveFile,
}: ImageItemProps) {
    const src = getPhotoUrl(pathComponents, item);
    const isActive = activeFile === item.name;

    function updateActiveFile() {
        // If already active, set to none
        if (isActive) {
            setActiveFile("");
            return;
        }

        setActiveFile(item.name);
    }

    return (
        <li
            className={`img-wrapper ${isActive ? "active" : ""}`}
            onClick={() => updateActiveFile()}
        >
            {isActive && (
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
