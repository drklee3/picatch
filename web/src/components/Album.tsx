import React, { useState, useEffect, useRef } from "react";
import { RouteComponentProps } from "react-router";
import { getPathComponents, fetchAlbumData } from "../util";
import { PathComponents, DirectoryItem } from "../types";
import ImageItem from "./ImageItem";
import ProgressBar from "./nprogress/ProgressBar";

function usePrevious<T>(value: T) {
    const ref = useRef<T>();
    useEffect(() => {
        ref.current = value;
    });
    return ref.current;
}

function Album(props: RouteComponentProps) {
    const initPathComp = getPathComponents(props.location.pathname);

    // States
    const [pathComponents, setPathComponents] = useState<PathComponents>(
        initPathComp
    );

    const [files, setFiles] = useState<DirectoryItem[]>([]);
    const [activeFile, setActiveFile] = useState(initPathComp.file || "");
    const [isLoading, setIsLoading] = useState(false);

    // Previous states
    const prevPathComponents = usePrevious<PathComponents>(pathComponents);

    // Effects

    useEffect(() => {
        const currentPath = props.location.pathname;

        if (!currentPath) {
            return;
        }

        // Parse current path and save to state
        const newPathComps = getPathComponents(currentPath);
        setPathComponents(newPathComps);
    }, [props.location.pathname]);

    useEffect(() => {
        // Add function here to use async/await
        const fetchData = async () => {
            setIsLoading(true);
            try {
                // Fetch album data
                const dirListing = await fetchAlbumData(pathComponents);
                setFiles(dirListing.files);
            } catch (e) {
                console.error("Failed to fetch album data:", e);
            }
            setIsLoading(false);
        };

        // Get album data from api if new album
        if (
            prevPathComponents === undefined ||
            prevPathComponents.album !== pathComponents.album
        ) {
            fetchData();
        }
    }, [pathComponents]); // Only run this effect if path changes

    useEffect(() => {
        // Update the document title using the browser API
        document.title = pathComponents.file || pathComponents.album || "hello";
    }, [pathComponents]); // Only update title if album or file changes

    // Update url whenever activeFile changes
    useEffect(() => {
        const newPath = "/album" + pathComponents.album + activeFile;
        console.log("activeFile changed:", activeFile);
        props.history.push(newPath);
    }, [activeFile, props.history, pathComponents.album]);

    // Set active file on mount
    useEffect(() => {
        if (pathComponents.file) {
            setActiveFile(pathComponents.file);
        }
    }, [pathComponents]);

    return (
        <div>
            <ProgressBar isAnimating={isLoading} />
            <pre>{JSON.stringify(pathComponents, null, 2)}</pre>
            <pre>{JSON.stringify(props, null, 2)}</pre>
            <ul id="image-list">
                {files.map((f) => (
                    <ImageItem
                        pathComponents={pathComponents}
                        activeFile={activeFile}
                        setActiveFile={setActiveFile}
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
