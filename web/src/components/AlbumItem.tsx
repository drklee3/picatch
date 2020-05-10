import React from "react";
import ImageItem from "./ImageItem";
import DirItem from "./DirItem";
import {
    PathComponents,
    DirectoryItem,
    DirectoryItemType,
    ActiveFile,
} from "../types";

type AlbumItemProps = {
    pathComponents: PathComponents;
    item: DirectoryItem;
    index: number;
    activeFile: ActiveFile;
    setActiveFile: React.Dispatch<React.SetStateAction<ActiveFile>>;
};

function AlbumItem(props: AlbumItemProps) {
    if (props.item.type === DirectoryItemType.File) {
        return <ImageItem {...props} />;
    }

    return <DirItem {...props} />;
}

export default AlbumItem;
