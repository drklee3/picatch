import { useEffect, useState } from "react";
import { fetchAlbumData } from "../util";
import { PathComponents, DirectoryListing } from "../types";

function useAlbumApi(
    path: PathComponents,
    setIsLoading: (loading: boolean) => void
) {
    const [
        directoryListing,
        setDirectoryListing,
    ] = useState<DirectoryListing | null>(null);
    const [error, setError] = useState<Error | null>(null);

    // Only run this effect if album/file changes
    useEffect(() => {
        // Add function here to use async/await
        const fetchData = async () => {
            setIsLoading(true);
            setError(null);
            try {
                // Fetch album data
                const dirListing = await fetchAlbumData(path.album);
                setDirectoryListing(dirListing);
            } catch (e) {
                console.error("Failed to fetch album data:", e);
                setError(e);
            }
            setIsLoading(false);
        };

        // Get album data from api if new album
        fetchData();
    }, [path.album, setIsLoading]);

    return {
        directoryListing,
        files: directoryListing?.files || [],
        albums: directoryListing?.albums || [],
        error,
    };
}

export default useAlbumApi;
