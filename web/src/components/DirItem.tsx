import React from "react";
import { PathComponents, DirectoryItem } from "../types";

type DirItemProps = {
    pathComponents: PathComponents;
    item: DirectoryItem;
    index: number;
    activeFile: [string, number];
    setActiveFile: React.Dispatch<React.SetStateAction<[string, number]>>;
};

function DirItem(props: DirItemProps) {
    return (
        <li className={`img-wrapper`}>
            <p>Album: {props.item.name}</p>
        </li>
    );
}

export default DirItem;
