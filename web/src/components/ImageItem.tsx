import React, { useEffect } from "react";
import LazyLoad from "react-lazyload";
import { PathComponents, DirectoryItem, ActiveFile } from "../types";
import { getPhotoUrl } from "../util";

type ImageItemProps = {
    pathComponents: PathComponents;
    item: DirectoryItem;
    index: number;
    activeFile: ActiveFile;
    setActiveFile: React.Dispatch<React.SetStateAction<ActiveFile>>;
};

function ImageItem({
    pathComponents,
    item,
    index,
    activeFile,
    setActiveFile,
}: ImageItemProps) {
    const src = getPhotoUrl(pathComponents, item);
    // Also check filename in case index isn't found yet
    const isActive =
        activeFile.index === index || activeFile.name === item.name;

    useEffect(() => {
        // Hey I'm the current file but the index is wrong so let me update it
        // Seems kind of inefficient? Listening to each of these for every image, every update
        // Only needs to be run when an image is loaded directly the **first** time but this
        // runs every single time the active image is changed
        if (activeFile.index === -1 && activeFile.name === item.name) {
            setActiveFile({ name: item.name, index });
            console.log("Updated missing index");
        }
    }, [activeFile, index, item.name, setActiveFile]);

    function updateActiveFile() {
        // If already active, set to none
        if (isActive) {
            setActiveFile({ name: "", index: -1 });
            return;
        }

        setActiveFile({ name: item.name, index });
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
