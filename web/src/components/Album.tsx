import React, { useEffect, useState, useRef, useReducer } from "react";
import { RouteComponentProps, useHistory } from "react-router";
import JustifiedLayout from "./JustifiedLayout";
import AlbumItem from "./AlbumItem";
import ImageItem from "./ImageItem";
import ProgressBar from "./nprogress/ProgressBar";
import { HistoryState } from "../types";
import usePathComponents from "../hooks/usePathComponents";
import useAlbumApi from "../hooks/useAlbumApi";
import useKeyboardNavigation from "../hooks/useKeyboardNavigation";
import useResize from "../hooks/useResize";
import activeFileReducer from "../reducers/activeFileReducer";
import { ActiveFileActions } from "../reducers/activeFileActions";
import { getPathComponents } from "../util";
import { LoadingContext } from "../contexts/LoadingContext";

type AlbumProps = RouteComponentProps & {
    root?: boolean;
};

function Album(props: AlbumProps) {
    const history = useHistory<HistoryState>();
    const path = usePathComponents();

    const [isDirectLink, setIsDirectLink] = useState(path.file !== null);
    const [isLoading, setIsLoading] = useState(false);

    // Also returns response but not really needed now
    const { albums, files, error } = useAlbumApi(path, setIsLoading);
    const [activeFileState, dispatch] = useReducer(activeFileReducer, {
        album: path.album,
        albumSize: -1,
        name: path.file || "",
        index: -1,
    });

    const layoutRef = useRef<HTMLDivElement>(null);
    const { width } = useResize(layoutRef);

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
        const { album } = getPathComponents(history.location.pathname);

        dispatch({
            type: ActiveFileActions.SET_ALBUM_AND_INDEX,
            album: album,
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
        <LoadingContext.Provider value={{ isLoading, setIsLoading }}>
            <ProgressBar isAnimating={isLoading} />
            {error && <p>Failed to fetch images</p>}
            {albums.length > 0 && <h4>Albums</h4>}
            <div id="album-list">
                {albums.map((a, i) => (
                    <AlbumItem
                        album={a}
                        key={i}
                        pathComponents={path}
                        dispatch={dispatch}
                    />
                ))}
            </div>
            {files.length > 0 && <h4>Photos</h4>}
            <div id="image-list" ref={layoutRef}>
                <JustifiedLayout containerWidth={width} containerPadding={0}>
                    {files.map((f, i) => (
                        <div
                            style={{
                                width: f.dimensions?.width || 600,
                                height: f.dimensions?.height || 600,
                            }}
                            key={f.name}
                        >
                            <ImageItem
                                pathComponents={path}
                                active={activeFileState.index === i}
                                activeFileState={activeFileState}
                                dispatch={dispatch}
                                index={i}
                                item={f}
                            />
                        </div>
                    ))}
                </JustifiedLayout>
            </div>
        </LoadingContext.Provider>
    );
}

export default Album;
