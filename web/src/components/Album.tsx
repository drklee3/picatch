import React, { useEffect, useState, useReducer } from "react";
import { RouteComponentProps, useHistory } from "react-router";
import AlbumItem from "./AlbumItem";
import ProgressBar from "./nprogress/ProgressBar";
import { HistoryState } from "../types";
import usePathComponents from "../hooks/usePathComponents";
import useAlbumApi from "../hooks/useAlbumApi";
import useKeyboardNavigation from "../hooks/useKeyboardNavigation";
import activeFileReducer from "../reducers/activeFileReducer";
import { ActiveFileActions } from "../reducers/activeFileActions";

type AlbumProps = RouteComponentProps & {
    root?: boolean;
};

function Album(props: AlbumProps) {
    const history = useHistory<HistoryState>();
    const path = usePathComponents();

    const [isDirectLink, setIsDirectLink] = useState(path.file !== null);

    // Also returns response but not really needed now
    const { files, isLoading, error } = useAlbumApi(path);
    const [activeFileState, dispatch] = useReducer(activeFileReducer, {
        album: path.album,
        albumSize: -1,
        name: path.file || "",
        index: -1,
    });

    const keyPressed = useKeyboardNavigation();

    /// Effects
    // Update page title on path change
    useEffect(() => {
        // Update the document title using the browser API
        document.title = path.file || path.album || "hello";
    }, [path]);

    useEffect(() => {
        if (files.length !== 0) {
            dispatch({
                type: ActiveFileActions.SET_ALBUM_SIZE,
                albumSize: files.length,
            });
        }
    }, [files]);

    // Update activeFile index on direct load
    useEffect(() => {
        if (!isDirectLink) {
            return;
        }

        // Only if files loaded
        if (files.length === 0) {
            return;
        }

        // Index can come from history state also on refresh, so this won't
        // necessarily run each time
        if (path.file !== null && activeFileState.index === -1) {
            const i = files.findIndex((e) => e.name === activeFileState.name);

            dispatch({
                type: ActiveFileActions.SET_INDEX,
                index: i,
            });
        }

        // Set to false after running once, even if above dispatch didn't run
        setIsDirectLink(false);
    }, [isDirectLink, path.file, activeFileState, files]);

    // Update current active file for browser back/forward buttons
    useEffect(() => {
        if (history.action !== "POP") {
            return;
        }

        const index = history.location.state?.index;

        dispatch({
            type: ActiveFileActions.SET_INDEX,
            index: index || -1,
        });
    }, [history.location, history.action]);

    // Arrow nav
    useEffect(() => {
        if (keyPressed === "ArrowRight") {
            dispatch({
                type: ActiveFileActions.INCREMENT_INDEX,
            });
        }

        if (keyPressed === "ArrowLeft") {
            dispatch({
                type: ActiveFileActions.DECREMENT_INDEX,
            });
        }

        if (keyPressed === "Escape") {
            dispatch({
                type: ActiveFileActions.SET_INDEX,
                index: -1,
            });
        }
    }, [keyPressed]);

    // Update browser url when album/active file state changes
    useEffect(() => {
        // Skip updating url if directly loading an image
        if (isDirectLink) {
            return;
        }

        if (files.length === 0) {
            return;
        }

        // Build url
        let fileName = "";

        if (
            activeFileState.index !== -1 &&
            activeFileState.index < files.length
        ) {
            fileName = files[activeFileState.index].name;
        }

        let newPath = activeFileState.album + fileName;

        // Prepend /album if not on root
        if (
            activeFileState.album.length > 1 &&
            !newPath.startsWith("/album/")
        ) {
            newPath = "/album" + newPath;
        }

        if (history.location.pathname !== newPath) {
            console.log(
                `history.push(${newPath},  { index: ${activeFileState.index} })`
            );
            history.push(newPath, { index: activeFileState.index });
        }
    }, [isDirectLink, activeFileState, files, history]);

    return (
        <div>
            <ProgressBar isAnimating={isLoading} />
            {error && <p>Failed to fetch images</p>}
            <ul id="image-list">
                {files.map((f, i) => (
                    <AlbumItem
                        pathComponents={path}
                        active={activeFileState.index === i}
                        activeFileState={activeFileState}
                        dispatch={dispatch}
                        index={i}
                        item={f}
                        key={f.name}
                    />
                ))}
                <li className="img-wrapper" />
            </ul>
        </div>
    );
}

export default Album;
