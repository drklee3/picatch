export interface PathComponents {
    root: string;
    album: string;
    file: string | null;
}

export interface ActiveFile {
    name: string;
    index: number;
}

export type HistoryState = HistoryStateValues | undefined;

export interface HistoryStateValues {
    index: number;
}

// Ensure these match models in Rust backend (model::directory.rs)
export interface ImageDimensions {
    width: number;
    height: number;
}

export interface AlbumInfo {
    description?: string;
    cover?: string;
}

export interface DirectoryAlbum {
    name: string;
    info?: AlbumInfo;
}

export interface DirectoryFile {
    name: string;
    exif?: { [key: string]: string };
    dimensions?: ImageDimensions;
}

export interface DirectoryListing {
    current: string;
    files: DirectoryFile[];
    albums: DirectoryAlbum[];
}
