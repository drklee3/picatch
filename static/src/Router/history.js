/**
 * Sets the new path, updates browser url, and saves to history
 * 
 * @param path new Path to set in browser url
 * @param data 
 */
export function setPath(path, data = {}) {
    // Update Title in Window's Tab
    document.title = path;

    window.history.pushState({ path, data }, "", path);
}
