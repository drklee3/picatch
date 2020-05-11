export enum DirectoryItemType {
    Dir = "Dir",
    File = "File",
}

export interface ImageDimensions {
    width: number;
    height: number;
}

export interface DirectoryItem {
    type: DirectoryItemType;
    name: string;
    exif: { [key: string]: string } | null;
    dimensions?: ImageDimensions;
}

export interface DirectoryListing {
    current: string;
    files: DirectoryItem[];
}

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
