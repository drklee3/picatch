/**
 * Url path, passed to API to fetch corresponding file (and dir listing)
 */
export function getPath(): string {
  return window.location.pathname;
}

/**
 * Gets current image filename or undefined if path points to dir
 */
export function currentImageName(): string | undefined {
  const path = getPath();
  const filenameIndex = path.lastIndexOf("/") + 1;

  const basename = path.substring(filenameIndex);
  if (basename.lastIndexOf(".") === -1) {
    return undefined;
  }

  return basename;
}

/**
 * Gets the url path to current album
 */
export function getCurrentAlbumPath(): string {
  let path = getPath();
  const imageName = currentImageName();

  // remove imageName only if current path is on an image
  if (imageName) {
    path = path.replace(imageName, "");
  }

  if (!path.endsWith("/")) {
    return path + "/";
  }

  return path;
}
