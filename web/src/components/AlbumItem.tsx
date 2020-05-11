import React from "react";
import ImageItem from "./ImageItem";
import DirItem from "./DirItem";
import { PathComponents, DirectoryItem, DirectoryItemType } from "../types";
import { ActiveFileState } from "../reducers/activeFileReducer";
import { ActiveFileActionTypes } from "../reducers/activeFileActions";

export type AlbumItemProps = {
    active: boolean;
    pathComponents: PathComponents;
    item: DirectoryItem;
    dispatch: React.Dispatch<ActiveFileActionTypes>;
    activeFileState: ActiveFileState;
    index: number;
};

function AlbumItem(props: AlbumItemProps) {
    if (props.item.type === DirectoryItemType.File) {
        return <ImageItem {...props} />;
    }

    return <DirItem {...props} />;
}

export default AlbumItem;
