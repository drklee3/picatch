export const IMAGE_EXTENSIONS = ["jpg", "jpeg"];
export const API_BASE_URL =
    process.env.NODE_ENV === "production"
        ? "/photos"
        : "http://localhost:8080/photos";
