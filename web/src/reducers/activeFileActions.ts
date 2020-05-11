export enum ActiveFileActions {
    SET_FILE = "SET_FILE",
    SET_NAME = "SET_NAME",
    SET_INDEX = "SET_INDEX",
    SET_ALBUM = "SET_ALBUM",
    SET_ALBUM_SIZE = "SET_ALBUM_SIZE",
    INCREMENT_INDEX = "INCREMENT_INDEX",
    DECREMENT_INDEX = "DECREMENT_INDEX",
}

export interface SetActiveFileAction {
    type: ActiveFileActions.SET_FILE;
    name: string;
    index: number;
}

export interface SetNameAction {
    type: ActiveFileActions.SET_NAME;
    name: string;
}

export interface SetIndexAction {
    type: ActiveFileActions.SET_INDEX;
    index: number;
}

export interface SetAlbumAction {
    type: ActiveFileActions.SET_ALBUM;
    album: string;
}

export interface SetAlbumSizeAction {
    type: ActiveFileActions.SET_ALBUM_SIZE;
    albumSize: number;
}

export interface IncrementIndexAction {
    type: ActiveFileActions.INCREMENT_INDEX;
}

export interface DecrementIndexAction {
    type: ActiveFileActions.DECREMENT_INDEX;
}

export type ActiveFileActionTypes =
    | SetActiveFileAction
    | SetNameAction
    | SetIndexAction
    | SetAlbumAction
    | SetAlbumSizeAction
    | IncrementIndexAction
    | DecrementIndexAction;
