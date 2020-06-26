import { PathComponents } from "../types";

export interface Breadcrumb {
    name: string;
    path: string;
}

export function getBreadcrumbsAlbumPaths(path: PathComponents): Breadcrumb[] {
    // Already at root album, just return self
    if (path.album === "/") {
        return [{ name: "home", path: path.album }];
    }

    let album = path.album;

    // Re add /album if doesn't exist
    if (!album.startsWith("/album")) {
        album = "/album" + album;
    }

    const paths = [];

    const albums = album.split("/").filter((a) => a !== "");

    for (let i = 0; i < albums.length; i++) {
        // Re add pre/post slashes since they're removed in the filter
        const path = "/" + albums.join("/") + "/";

        const albumName = albums.pop();

        if (albumName === undefined) {
            break;
        }

        if (path === "/album") {
            continue;
        }

        paths.push({ name: albumName, path });
    }

    paths.push({ name: "home", path: "/" });

    return paths.reverse();
}
