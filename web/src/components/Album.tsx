import React, { useEffect, useReducer } from "react";
import { RouteComponentProps, useHistory } from "react-router";
import AlbumItem from "./AlbumItem";
import ProgressBar from "./nprogress/ProgressBar";
import usePathComponents from "../hooks/usePathComponents";
import useAlbumApi from "../hooks/useAlbumApi";
import activeFileReducer, {
    ActiveFileActions,
} from "../reducers/activeFileReducer";

type AlbumProps = RouteComponentProps & {
    root?: boolean;
};

function Album(props: AlbumProps) {
    // React router hooks
    const history = useHistory();

    const path = usePathComponents();
    // Also returns response but not really needed now
    const { files, isLoading, error } = useAlbumApi(path);

    const [activeFileState, dispatch] = useReducer(activeFileReducer, {
        name: path.file || "",
        index: -1,
    });

    /// States

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

    // Update url whenever activeFile or album changes
    useEffect(() => {
        let newPath;

        // Either on / or /album/
        if (props.root) {
            newPath = "/" + activeFileState.name;
        } else {
            newPath = "/album" + path.album + activeFileState.name;
        }

        console.log(history.location.pathname, "vs", newPath);
        console.log("activeFileState:", activeFileState);
        console.log("path.album:", path.album);

        // Only update path if new path
        if (history.location.pathname !== newPath) {
            history.push(newPath);
            console.log("new path:", newPath);
        }
    }, [activeFileState, path.album, props.root, history]);

    /*
    // Update current active file for browser back/forward buttons
    useEffect(() => {
        // Update active file, possibly between picture/album or between picture/picture
        setActiveFile([
            path.file || "",
            path.file ? files.findIndex((e) => e.name === path.file) : -1,
        ]);

        console.log("Recovered activefile from history:", path.file);
    }, [path.file, files]);
    */

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
                        active={
                            activeFileState.index === i ||
                            activeFileState.name === f.name
                        }
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
