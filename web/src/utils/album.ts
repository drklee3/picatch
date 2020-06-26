import { PathComponents } from "../types";

const MAX_BREADCRUMBS = 3;

export interface Breadcrumb {
    name: string;
    path: string | null;
}

export function getBreadcrumbsAlbumPaths(path: PathComponents): Breadcrumb[] {
    // Already at root album, just empty
    if (path.album === "/") {
        return [];
    }

    let album = path.album;

    // remove /album if exist
    if (album.startsWith("/album")) {
        album = album.replace("/album", "");
    }

    let paths: Breadcrumb[] = [];

    const albums = album.split("/").filter((a) => a !== "");

    // Copy size since the albums.pop() modifies length during loop
    const albumSize = albums.length;

    for (let i = 0; i < albumSize; i++) {
        // Re add pre/post slashes since they're removed in the filter
        const path = "/album/" + albums.join("/") + "/";

        const albumName = albums.pop();

        // Check albumName because of the pop(), but it should exist
        if (albumName === undefined) {
            break;
        }

        paths.push({ name: albumName, path });
    }

    if (paths.length > MAX_BREADCRUMBS) {
        paths = paths.slice(0, MAX_BREADCRUMBS);

        paths.push({ name: "...", path: null });
    }

    return paths.reverse();
}
