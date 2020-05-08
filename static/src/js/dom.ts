import {DirectoryListing, DirectoryItem, DirectoryItemType} from "./types";

function parseImageDimensions(item: DirectoryItem): [number, number] {
    if (item.exif === null) {
        return undefined;
    }
}

function createImageElement(item: DirectoryItem, imgBasePath: string, active: boolean): HTMLImageElement | undefined {
    const imageElement = new Image(300, 400);
    imageElement.src = imgBasePath + item.name;
    if (imageElement.src === undefined) {
        return;
    }

    // If currently active, add active class
    if (active) {
        imageElement.classList.add("active");
    }

    // add filename to data attr
    imageElement.setAttribute("data-filename", item.name);

    return imageElement;
}

export function displayAlbum(album: DirectoryListing, imgBasePath: string, activeFile: string): void {
    const albumElement = document.getElementById("album");

    album.files.forEach(e => {
        if (e.type === DirectoryItemType.Dir) {
            return;
        }

        const element = createImageElement(e, imgBasePath, activeFile === e.name);
        if (element === undefined) {
            return;
        }

        albumElement.appendChild(element);
    })
}

export function setParentAlbumUrl(url: string) {
    const e = document.getElementById("upAlbum");
    e.setAttribute("href", url);
}
