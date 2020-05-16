import { API_BASE_URL, IMAGE_BASE_URL } from "./constants";
import { PathComponents, DirectoryListing, DirectoryItem } from "./types";

export async function fetchAlbumData(
    pathAlbum: PathComponents["album"]
): Promise<DirectoryListing> {
    const res = await fetch(API_BASE_URL + pathAlbum);
    return res.json();
}

export function getPhotoUrl(pathComp: PathComponents, item: DirectoryItem) {
    return IMAGE_BASE_URL + pathComp.album + item.name;
}

export function imageIsActive(pathComp: PathComponents, item: DirectoryItem) {
    return pathComp.file === item.name;
}

/**
 * Returns filename for currently displayed image.
 * Returns null if on album page.
 */
export function getImageName(path: string) {
    if (path.endsWith("/")) {
        return null;
    }

    const filenameIndex = path.lastIndexOf("/") + 1;

    const basename = path.substring(filenameIndex);
    if (basename.lastIndexOf(".") === -1) {
        return null;
    }

    return basename;
}

/**
 * Gets current path's album and image if available
 */
export function getPathComponents(path: string): PathComponents {
    let root = "";
    if (path.startsWith("/album")) {
        root = "/album";
        path = path.replace("/album", "");
    }

    // If trailing "/" then we on album
    if (path.endsWith("/")) {
        return {
            root,
            album: path,
            file: null,
        };
    }

    const imageName = getImageName(path);

    // Remove imageName only if current path is on an image
    if (imageName) {
        path = path.replace(imageName, "");
    }

    // Add slash if somehow missing a trailing slash
    if (!path.endsWith("/")) {
        path += "/";
    }

    return {
        root,
        album: path,
        file: imageName,
    };
}
