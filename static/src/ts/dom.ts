import { DirectoryListing, DirectoryItem, DirectoryItemType } from "../types";

function parseImageDimensions(item: DirectoryItem): [number, number] {
    if (item.exif === null) {
        return undefined;
    }
}

function createImageElement(item: DirectoryItem, imgBasePath: string, activeFile: string): HTMLElement | undefined {
    // If dimensions missing, something probably went wrong reading the file
    if (item.dimensions === undefined) {
        return;
    }

    const wrapperElement = document.createElement("li");

    const imageElement = new Image(item.dimensions.width, item.dimensions.height);
    imageElement.src = imgBasePath + item.name;
    if (imageElement.src === undefined) {
        return;
    }

    imageElement.classList.add("img")

    // If currently active, add active class
    if (activeFile !== undefined && activeFile === item.name) {
        wrapperElement.classList.add("active");
    }

    // Add filename to data attr
    wrapperElement.setAttribute("data-filename", item.name);
    wrapperElement.id = item.name;

    // Put image into wrapper
    wrapperElement.appendChild(imageElement);

    return wrapperElement;
}

export function displayAlbum(album: DirectoryListing, imgBasePath: string, activeFile: string): void {
    const albumElement = document.getElementById("album");

    album.files.forEach(e => {
        if (e.type === DirectoryItemType.Dir) {
            return;
        }

        const element = createImageElement(e, imgBasePath, activeFile);
        if (element === undefined) {
            return;
        }

        albumElement.appendChild(element);
    })

    // Insert empty list item so the last one grows
    const emptyLi = document.createElement("li");
    albumElement.appendChild(emptyLi);
}

export function setParentAlbumUrl(url: string) {
    const e = document.getElementById("upAlbum");
    e.setAttribute("href", url);
}
