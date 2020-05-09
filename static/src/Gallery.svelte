<script>
    import { onMount } from "svelte";
    import ImageItem from "./ImageItem.svelte";
    import { fetchAlbumData } from "./util";

    export let path = { album: "/", file: null };
    let oldPath = null;

    // Compare old paths
    $: {
        console.log("New path:", path);
        oldPath = path;
    }

    console.log("hello");

    let files = [];

    onMount(async () => {
        console.log("Gallery mounted! Fetching images");
        const albumData = await fetchAlbumData(path.album);
        files = albumData.files;
        console.log("Fetched data:", files);
    });
</script>

<div>
    <p>hello</p>
    <p>Current album: {path.album}</p>
    <p>file: {path.file}</p>

    <div style="display: flex">
        {#each files as file}
            {#if file.type === 'File'}
                <ImageItem item="{file}" {path} />
            {/if}
        {:else}
            <p>loading...</p>
        {/each}
    </div>
</div>

<style>

</style>
