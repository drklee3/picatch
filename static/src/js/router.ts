import {API_BASE_URL} from "./constants";
import {DirectoryListing, DirectoryItem, DirectoryItemType} from "./types";
import {displayAlbum} from "./dom";

export default class Router {
    public type: DirectoryItemType;
    public albumData: DirectoryListing;

    constructor() {
        window.addEventListener("popstate", this.handlePopState.bind(this));

        // Setup click listner
        const albumElement = document.getElementById("album");
        // uhh idk if this is a good idea but other naive way would cause a circular dependency
        albumElement.addEventListener("click", this.handleImageClick.bind(this));
    }

    /// Event Listeners

    openImage(target: HTMLElement, setPath = true) {
        // Maximize image
        target.classList.add("active");

        // Append url with filename
        const newPath = this.getAlbumPath() + target.dataset.filename;

        if (setPath) {
            this.setPath(newPath);
        }
    }

    closeImage(target: HTMLElement, setPath = true) {
        target.classList.remove("active");

        // Set to album url
        const newPath = this.getAlbumPath();

        if (setPath) {
            this.setPath(newPath);
        }
    }

    handleImageClick(e: MouseEvent) {
        const target = e.target as HTMLElement;

        // Ignore non-images
        if (target.tagName !== "IMG") {
            return;
        }

        // Toggle active class
        if (target.classList.contains("active")) {
            this.closeImage(target);
        } else {
            this.openImage(target);
        }
    }

    resetActiveImages() {
        const albumElement = document.getElementById("album");
        const activeImgElements = albumElement.querySelectorAll(".active");
        activeImgElements.forEach(e => {
            e.classList.remove("active");
        });
    }

    handlePopState(e: PopStateEvent) {
        const path = e.state.path;

        if (!this.isNewAlbum(path)) {
            console.log("existing album : ", path);
            // Same album, check if on image url
            const imgName = this.getImageName(path);

            // Reset active images
            this.resetActiveImages();

            if (!imgName) {
                // On an album so just do nothing else
                return;
            }

            const albumElement = document.getElementById("album");
            const imgElement = albumElement.querySelector(`[data-filename='${imgName}']`);

            // Found image, so let's set it active again.  Don't call setPath since we dont want a
            // second history entry
            this.openImage(imgElement as HTMLElement, false);
        }
    }

    /// Routing

    async init() {
        return this.updateCurrentAlbumData();
    }

    async makeRequest<T>(request: RequestInfo): Promise<T> {
        const response = await fetch(request);
        const body = await response.json();

        return body;
    }

    /**
     * Gets current album data.
     * If on specific image path, get album containing image.
     */
    async updateCurrentAlbumData() {
        const reqUrl = API_BASE_URL + this.getAlbumPath();

        try {
            this.albumData = await this.makeRequest<DirectoryListing>(reqUrl);
            displayAlbum(this.albumData, this.getAlbumImagePath());
        } catch (e) {
            console.error(`Failed to get current album data: ${reqUrl}`, e);
        }
    }

    getPath(): string {
        return window.location.pathname;
    }

    setPath(path: string) {
        console.log("setpath: ", path);
        // Update Title in Window's Tab
        document.title = path;

        // Load content for this tab/page

        window.history.pushState({path}, "", path);
    }

    /**
     * Returns filename for currently displayed image.
     * Returns undefined if on album page.
     */
    getImageName(path = this.getPath()): string | undefined {
        if (path.endsWith("/")) {
            return undefined;
        }

        const filenameIndex = path.lastIndexOf("/") + 1;
      
        const basename = path.substring(filenameIndex);
        if (basename.lastIndexOf(".") === -1) {
            return undefined;
        }

        return basename;
    }

    /**
     * Base path for actually fetching the image files
     */
    getAlbumImagePath(): string {
        return "/photos" + this.getAlbumPath();
    }

    /**
     * If currently on photo url, returns the photo's album.
     * If currently on album, returns album url (no change).
     * 
     * Default it uses 
     */
    getAlbumPath(path = this.getPath()): string {
        // If already album path, return path
        if (path.endsWith("/")) {
            return path;
        }

        const imageName = this.getImageName(path);
      
        // Remove imageName only if current path is on an image
        if (imageName) {
          path = path.replace(imageName, "");
        }

        // Add slash if somehow missing a trailing slash
        if (!path.endsWith("/")) {
          return path + "/";
        }
      
        return path;
    }

    isNewAlbum(newPath: string) {
        return this.getAlbumPath() !== this.getAlbumPath(newPath);
    }

    /**
     * Get parent album url
     */
    getParentAlbum() {
        // Albums always end in trailing /
        const pathArr = this.getAlbumPath().split("/");

        pathArr.pop();
        pathArr.pop();

        return pathArr.join("/");
    }
}
