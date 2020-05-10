import React from "react";
import ImageItem from "./ImageItem";
import DirItem from "./DirItem";
import { PathComponents, DirectoryItem, DirectoryItemType } from "../types";

type AlbumItemProps = {
    pathComponents: PathComponents;
    item: DirectoryItem;
    index: number;
    activeFile: [string, number];
    setActiveFile: React.Dispatch<React.SetStateAction<[string, number]>>;
};

function AlbumItem(props: AlbumItemProps) {
    if (props.item.type === DirectoryItemType.File) {
        return <ImageItem {...props} />;
    }

    return <DirItem {...props} />;
}

export default AlbumItem;
