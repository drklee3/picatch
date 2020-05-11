import React, { useEffect, useReducer } from "react";
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
    // Also returns response but not really needed now
    const { files, isLoading, error } = useAlbumApi(path);

    const [activeFileState, dispatch] = useReducer(activeFileReducer, {
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

    // Update activeFile index on direct load
    useEffect(() => {
        if (
            activeFileState.name !== "" &&
            activeFileState.index === -1 &&
            files.length !== 0
        ) {
            const i = files.findIndex((e) => e.name === activeFileState.name);
            console.log("updating index");
            dispatch({
                type: ActiveFileActions.SET_INDEX,
                index: i,
            });
        }
    }, [path.file, activeFileState, files]);

    // Update current active file for browser back/forward buttons
    useEffect(() => {
        // Update active file, possibly between picture/album or between picture/picture
        if (history.action !== "POP") {
            return;
        }

        // pic -> album: Path doesn't have file, but there is an activeFile
        if (path.file === null && activeFileState.name !== "") {
            console.log("pic -> album");
            dispatch({
                type: ActiveFileActions.SET_FILE,
                name: "",
                index: -1,
            });
        }

        // album -> pic: Path has file, but there is no activeFile
        if (path.file !== null && activeFileState.name === "") {
            console.log("album -> pic");
            // Read from history state first
            let index = history.location.state?.index;

            // Somehow missing index from state?  Look in files list
            if (index === undefined) {
                index = files.findIndex((e) => e.name === activeFileState.name);
            }

            dispatch({
                type: ActiveFileActions.SET_FILE,
                name: path.file,
                index,
            });
        }

        // pic -> pic: Path has file, but activeFile is different
        if (
            path.file !== null &&
            activeFileState.name !== "" &&
            path.file !== activeFileState.name
        ) {
            console.log("pic -> pic", path.file, activeFileState.name);
            let index = history.location.state?.index;

            if (index === undefined) {
                index = files.findIndex((e) => e.name === activeFileState.name);
            }

            dispatch({
                type: ActiveFileActions.SET_FILE,
                name: path.file || "",
                index,
            });
        }
    }, [path.file, activeFileState, history, files]);

    // arrow nav
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
                type: ActiveFileActions.SET_FILE,
                name: "",
                index: -1,
            });
        }
    }, [keyPressed]);

    return (
        <div>
            <ProgressBar isAnimating={isLoading} />
            <pre>{JSON.stringify(path, null, 2)}</pre>
            <pre>{JSON.stringify(props, null, 2)}</pre>
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
