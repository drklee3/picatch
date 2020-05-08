import {getCurrentAlbumPath} from "./routing";
import {API_BASE_URL} from "./constants";
import {DirectoryListing, DirectoryItem, DirectoryItemType} from "./types";

async function makeRequest(path: string): Promise<DirectoryListing> {
  const res = await fetch(API_BASE_URL + path);
  return res.json();
}

export function getCurrentAlbum(): Promise<DirectoryListing> {
  const path = getCurrentAlbumPath();
  return makeRequest(path);
}

export function getImageUrl(item: DirectoryItem): string | undefined {
  if (item.type === DirectoryItemType.Dir) {
    return;
  }

  return API_BASE_URL + getCurrentAlbumPath() + item.name
}

export function getParentAlbum(): string {
  const path = getCurrentAlbumPath();
  const pathArr = path.split("/");
  pathArr.pop();
  
  return pathArr.join("/");
}
