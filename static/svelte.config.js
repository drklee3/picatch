// See https://github.com/kaisermann/svelte-preprocess#with-svelte-vs-code
const sveltePreprocess = require("svelte-preprocess");

module.exports = {
    preprocess: sveltePreprocess(),
    // ...other svelte options (optional)
};
