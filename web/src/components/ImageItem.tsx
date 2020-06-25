import React, { useContext } from "react";
import LazyLoad from "react-lazyload";
import Image from "./Image";
import { getPhotoUrl } from "../util";
import { ActiveFileActions } from "../reducers/activeFileActions";
import { ImageSize } from "../constants";
import { LoadingContext } from "../contexts/LoadingContext";
import { PathComponents, DirectoryFile } from "../types";
import { ActiveFileState } from "../reducers/activeFileReducer";
import { ActiveFileActionTypes } from "../reducers/activeFileActions";

interface ImageItemProps {
    active: boolean;
    pathComponents: PathComponents;
    item: DirectoryFile;
    dispatch: React.Dispatch<ActiveFileActionTypes>;
    activeFileState: ActiveFileState;
    index: number;
}

function ImageItem({
    active,
    pathComponents,
    item,
    index,
    dispatch,
}: ImageItemProps) {
    const src = getPhotoUrl(pathComponents, item, ImageSize.Medium);
    const largeSrc = getPhotoUrl(pathComponents, item, ImageSize.Large);

    const { setIsLoading } = useContext(LoadingContext);

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
        <div
            className={`img-wrapper ${active ? "active" : ""}`}
            onClick={updateActiveFile}
        >
            {active && (
                <div className="img-fullscreen-wrapper">
                    <img
                        src={src}
                        alt={item.name}
                        className="img-fullscreen img-fullscreen-thumbnail"
                    />
                    <Image
                        src={largeSrc}
                        alt={item.name}
                        setIsLoading={setIsLoading}
                        className="img-fullscreen invisible"
                    />
                </div>
            )}
            <LazyLoad
                height="100%"
                offset={100}
                placeholder={<div className="img-placeholder" />}
            >
                <div className="img-thumbnail-background" />
                <Image
                    src={src}
                    alt={item.name}
                    className="img-thumbnail invisible"
                    width={item.dimensions?.width}
                    height={item.dimensions?.height}
                />
            </LazyLoad>
        </div>
    );
}

export default ImageItem;
