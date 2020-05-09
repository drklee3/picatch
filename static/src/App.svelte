<script>
    import { API_BASE_URL } from "./constants";
    import Router from "./Router/Router.svelte";
    import Gallery from "./Gallery.svelte";
</script>

<Router let:path>
    <Gallery {path} />
</Router>

<style>

</style>
