import React, { useState, useEffect } from "react";
import { RouteComponentProps, useHistory } from "react-router-dom";
import { fetchAlbumData } from "../util";
import { DirectoryItem } from "../types";
import AlbumItem from "./AlbumItem";
import ProgressBar from "./nprogress/ProgressBar";
import usePathComponents from "../hooks/usePathComponents";

type AlbumProps = RouteComponentProps & {
    root?: boolean;
};

function Album(props: AlbumProps) {
    // React router hooks
    const history = useHistory();

    const path = usePathComponents();

    // States
    const [files, setFiles] = useState<DirectoryItem[]>([]);
    // activeFile === ["fileName", file index]
    const [activeFile, setActiveFile] = useState<[string, number]>([
        path.file || "",
        -1, // file list doesn't exist yet buddy
    ]);
    const [isLoading, setIsLoading] = useState(true);

    // Effects

    // Only run this effect if album/file changes
    useEffect(() => {
        // Add function here to use async/await
        const fetchData = async () => {
            setIsLoading(true);
            try {
                // Fetch album data
                const dirListing = await fetchAlbumData(path.album);
                setFiles(dirListing.files);
            } catch (e) {
                console.error("Failed to fetch album data:", e);
            }
            setIsLoading(false);
        };

        // Get album data from api if new album
        fetchData();
    }, [path.album]);

    // Update page title on path change
    useEffect(() => {
        // Update the document title using the browser API
        document.title = path.file || path.album || "hello";
    }, [path]);

    // Update url whenever activeFile changes
    useEffect(() => {
        let newPath;

        // Either on / or /album/
        if (props.root) {
            newPath = "/" + activeFile[0];
        } else {
            newPath = "/album" + path.album + activeFile[0];
        }

        // Only update path if new path
        if (history.location.pathname !== newPath) {
            history.push(newPath);
            console.log("new activeFile:", activeFile);
        }
    }, [activeFile, path.album, props.root, history]);

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
            <ul id="image-list">
                {files.map((f, i) => (
                    <AlbumItem
                        pathComponents={path}
                        activeFile={activeFile}
                        setActiveFile={setActiveFile}
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
