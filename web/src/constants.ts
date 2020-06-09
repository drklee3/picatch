export const IMAGE_EXTENSIONS = ["jpg", "jpeg"];
export const API_BASE_URL =
    process.env.NODE_ENV === "production"
        ? "/api/photos"
        : "http://localhost:8080/api/photos";

export const IMAGE_BASE_URL =
    process.env.NODE_ENV === "production"
        ? "/photo"
        : "http://localhost:8080/photo";

export const RESIZED_IMAGE_BASE_URL =
    process.env.NODE_ENV === "production"
        ? "/photo_sized"
        : "http://localhost:8080/photo_sized";
