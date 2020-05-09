<script>
    import { getPhotoUrl } from "./util";

    export let item;
    export let path;

    const src = getPhotoUrl(path.album, item.name);
    export let active = false;
</script>

<div class:active>
    <img
        {src}
        alt=""
        width="{item.dimensions.width / 10}"
        height="{item.dimensions.height / 10}"
    />
</div>

<style>

</style>
