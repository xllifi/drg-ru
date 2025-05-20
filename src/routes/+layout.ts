import { addMessages, getLocaleFromNavigator, init } from "svelte-i18n";
import ru from "$lib/lang/ru.json";
import '$lib/styles/main.scss';

// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

addMessages("ru", ru);

init({
  fallbackLocale: "ru",
  initialLocale: getLocaleFromNavigator(),
});
