import {
    API_BASE_URL,
    IMAGE_BASE_URL,
    RESIZED_IMAGE_BASE_URL,
    ImageSize,
} from "./constants";
import {
    PathComponents,
    DirectoryListing,
    DirectoryAlbum,
    DirectoryFile,
} from "./types";

export async function fetchAlbumData(
    pathAlbum: PathComponents["album"]
): Promise<DirectoryListing> {
    const res = await fetch(API_BASE_URL + pathAlbum);
    return res.json();
}

function isAlbum(file: DirectoryAlbum | DirectoryFile): file is DirectoryAlbum {
    return (file as DirectoryAlbum).info !== undefined;
}

export function getPhotoUrl(
    pathComp: PathComponents,
    item: DirectoryAlbum | DirectoryFile,
    size: ImageSize
) {
    let itemName = item.name;
    let albumCoverPath = "";

    if (isAlbum(item) && item.info?.cover) {
        // Use cover item name if this is an album
        itemName = item.info.cover;
        albumCoverPath = item.name;
    }

    let dotIndex = itemName.lastIndexOf(".");
    let fileName = itemName.slice(0, dotIndex);
    let fileExtension = itemName.slice(dotIndex + 1);

    let baseUrl =
        size === ImageSize.Original ? IMAGE_BASE_URL : RESIZED_IMAGE_BASE_URL;

    return `${baseUrl}${pathComp.album}${albumCoverPath}${fileName}-${size}.${fileExtension}`;
}

export function imageIsActive(pathComp: PathComponents, item: DirectoryFile) {
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
    if (path.startsWith("/album")) {
        path = path.replace("/album", "");
    }

    // If trailing "/" then we on album
    if (path.endsWith("/")) {
        return {
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
        album: path,
        file: imageName,
    };
}
