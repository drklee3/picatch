export enum DirectoryItemType {
  Dir = "Dir",
  File = "File",
}

export interface DirectoryItem {
  type: DirectoryItemType;
  name: string
  exif: {[key: string]: string} | null;
}

export interface DirectoryListing {
  current: string;
  files: DirectoryItem[];
}

export interface PathComponent {
  album: string;
  file?: string;
}
