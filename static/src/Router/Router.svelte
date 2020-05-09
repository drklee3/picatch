<script>
    import { onMount } from "svelte";
    import { getPathComponents } from "../util";

    // Get current path data
    const path = getPathComponents();

    // listen for history changes
    // window.addEventListener("popstate", handlePopState);
</script>

<slot {path} />
