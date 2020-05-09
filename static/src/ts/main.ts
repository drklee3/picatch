import Router from "./router";

async function main() {
  console.log("hello")
  const router = new Router();

  try {
    await router.init();
  } catch (e) {
    console.error("Failed to get album:", e);
  }
}

main();
