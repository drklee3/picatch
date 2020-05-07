const API_BASE_URL = "//localhost:8080/photos/";

enum DirectoryItemType {
  Dir,
  File,
}

interface DirectoryItem {
  type: DirectoryItemType;
  name: string
  exif: {[key: string]: string};
}

interface DirectoryListing {
  current: string;
  files: DirectoryItem[];
}

async function getFiles(path: string): Promise<DirectoryListing> {
  const res = await fetch(API_BASE_URL + path);
  return res.json();
}

/**
 * Url path, passed to API to fetch corresponding file (and dir listing)
 */
function getPath(): string {
  return window.location.pathname;
}

/**
 * Gets current image filename or undefined if path points to dir
 */
function currentImageName(): string | undefined {
  const path = getPath();
  const filenameIndex = path.lastIndexOf("/") + 1;

  const basename = path.substring(filenameIndex);
  if (basename.lastIndexOf(".") === -1) {
    return undefined;
  }

  return basename;
}

function getCurrentAlbum(): string {
  let path = getPath();
  const imageName = currentImageName();

  // remove imageName only if current path is on an image
  if (imageName) {
    path = path.replace(imageName, "");
  }

  return path;
}

function main() {
  console.log("hello")
  console.log(getCurrentAlbum())
}

main();
