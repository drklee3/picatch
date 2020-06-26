import { getPathComponents } from "./util";

const paths = [
    {
        pathname: "/",
        pathComponents: {
            album: "/",
            file: null,
        },
    },
    {
        pathname: "/album/someAlbum/notactuallyanimage.jpg/",
        pathComponents: {
            album: "/someAlbum/notactuallyanimage.jpg/",
            file: null,
        },
    },
    {
        pathname: "/album/someAlbum/image.jpg",
        pathComponents: {
            album: "/someAlbum/",
            file: "image.jpg",
        },
    },
    {
        pathname: "/album/a/nested/album/pic.jpeg",
        pathComponents: {
            album: "/a/nested/album/",
            file: "pic.jpeg",
        },
    },
];

describe("getImageName", () => {
    it("should get image name from path", () => {
        paths.forEach((path) => {
            expect(getPathComponents(path.pathname)).toEqual(
                path.pathComponents
            );
        });
    });
});
