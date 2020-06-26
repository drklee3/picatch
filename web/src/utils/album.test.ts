import { getBreadcrumbsAlbumPaths } from "./album";

describe("getParentAlbumPath", () => {
    it("should return current album only on root album", () => {
        const parent = getBreadcrumbsAlbumPaths({
            file: "",
            album: "/",
        });

        expect(parent).toEqual([]);
    });

    it("should return parent album", () => {
        const parent = getBreadcrumbsAlbumPaths({
            file: "",
            album: "/some/dir/",
        });

        expect(parent).toEqual([
            { name: "some", path: "/album/some/" },
            { name: "dir", path: "/album/some/dir/" },
        ]);
    });

    it("should return nested parent album", () => {
        const parent = getBreadcrumbsAlbumPaths({
            file: "",
            album: "/some/dir/nested/",
        });

        expect(parent).toEqual([
            { name: "some", path: "/album/some/" },
            { name: "dir", path: "/album/some/dir/" },
            { name: "nested", path: "/album/some/dir/nested/" },
        ]);
    });

    it("should return max 3 breadcrumbs", () => {
        const parent = getBreadcrumbsAlbumPaths({
            file: "",
            album: "/some/dir/nested/really/deep/",
        });

        expect(parent).toEqual([
            { name: "...", path: null },
            { name: "nested", path: "/album/some/dir/nested/" },
            { name: "really", path: "/album/some/dir/nested/really/" },
            { name: "deep", path: "/album/some/dir/nested/really/deep/" },
        ]);
    });
});
