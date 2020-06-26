import React from "react";
import { ActiveFileActions } from "../reducers/activeFileActions";
import { PathComponents } from "../types";
import { ActiveFileActionTypes } from "../reducers/activeFileActions";
import { getBreadcrumbsAlbumPaths } from "../utils/album";

export interface BreadcrumbsProps {
    pathComponents: PathComponents;
    dispatch: React.Dispatch<ActiveFileActionTypes>;
}

function AlbumBreadcrumbs({ pathComponents, dispatch }: BreadcrumbsProps) {
    const paths = getBreadcrumbsAlbumPaths(pathComponents);

    function goToAlbum(album: string | null) {
        if (album === null) {
            return;
        }

        dispatch({
            type: ActiveFileActions.SET_ALBUM,
            album: album,
        });
    }

    return (
        <ul id="breadcrumbs-wrapper">
            {paths.map((breadcrumb, i) => (
                <li
                    className={`breadcrumb ${
                        breadcrumb.name === "..." ? "breadcrumb-ellipsis" : ""
                    }`}
                    key={i}
                    onClick={() => goToAlbum(breadcrumb.path)}
                >
                    <p>{breadcrumb.name}</p>
                </li>
            ))}
        </ul>
    );
}

export default AlbumBreadcrumbs;
