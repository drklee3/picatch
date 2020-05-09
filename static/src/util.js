import { API_BASE_URL } from "./constants";

export async function fetchAlbumData(album) {
    const res = await fetch(API_BASE_URL + album);
    return res.json();
}

export function getPhotoUrl(album, fileName) {
    return API_BASE_URL + album + fileName;
}

/**
 * Returns filename for currently displayed image.
 * Returns null if on album page.
 */
export function getImageName(path = window.location.pathname) {
    if (!path) {
        return null;
    }

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
export function getPathComponents(path = window.location.pathname) {
    if (!path) {
        return {
            album: null,
            file: null,
        }
    }

    if (path.startsWith("/album")) {
        path = path.replace("/album", "");
    }

    // If already album path, return path
    if (path.endsWith("/")) {
        return {
            album: path,
            file: null
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
        album: path,
        file: imageName
    };
}

/**
 * If currently on photo url, returns the photo's album.
 * If currently on album, returns album url (no change).
 * 
 * Default it uses current path, but a different path can be passed in to be parsed.
 */
export function getAlbumPath(path = window.location.pathname) {
    return getPathComponents(path).album;
}
