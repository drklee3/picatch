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
  name: string
  exif: {[key: string]: string} | null;
  dimensions?: ImageDimensions;
}

export interface DirectoryListing {
  current: string;
  files: DirectoryItem[];
}

export interface PathComponent {
  album: string;
  file?: string;
}
