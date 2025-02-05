<script setup lang="ts">
import CNavbar from "./components/CNavbar.vue";
import CNote from "./components/CNote.vue";
import { useUiStore } from "./stores/ui";

const ui = useUiStore();
</script>

<template>
  <div class="flex h-screen touch-pan-x touch-pan-y flex-col">
    <CNavbar />
    <div class="h-full overflow-auto p-1">
      <CNote :noteId="ui.anchor" :path="[]" :focusPath="ui.focusPath" />
    </div>
  </div>
</template>

<style>
@import "tailwindcss";

/*
Weights according to https://github.com/bBoxType/FiraGO?tab=readme-ov-file#firago-font-weights

100 - Thin
200 - UltraLight
300 - Light
400 - Regular
500 - Medium
600 - SemiBold
700 - Bold
800 - ExtraBold
900 - Heavy
*/

@font-face {
  font-family: FiraGO;
  font-style: normal;
  font-weight: 100;
  src: url("/fonts/FiraGO-Thin.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: normal;
  font-weight: 200;
  src: url("/fonts/FiraGO-UltraLight.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: normal;
  font-weight: 300;
  src: url("/fonts/FiraGO-Light.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: normal;
  font-weight: 400;
  src: url("/fonts/FiraGO-Regular.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: normal;
  font-weight: 500;
  src: url("/fonts/FiraGO-Medium.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: normal;
  font-weight: 600;
  src: url("/fonts/FiraGO-SemiBold.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: normal;
  font-weight: 700;
  src: url("/fonts/FiraGO-Bold.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: normal;
  font-weight: 800;
  src: url("/fonts/FiraGO-ExtraBold.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: normal;
  font-weight: 900;
  src: url("/fonts/FiraGO-Heavy.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: italic;
  font-weight: 100;
  src: url("/fonts/FiraGO-ThinItalic.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: italic;
  font-weight: 200;
  src: url("/fonts/FiraGO-UltraLightItalic.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: italic;
  font-weight: 300;
  src: url("/fonts/FiraGO-LightItalic.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: italic;
  font-weight: 400;
  src: url("/fonts/FiraGO-Italic.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: italic;
  font-weight: 500;
  src: url("/fonts/FiraGO-MediumItalic.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: italic;
  font-weight: 600;
  src: url("/fonts/FiraGO-SemiBoldItalic.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: italic;
  font-weight: 700;
  src: url("/fonts/FiraGO-BoldItalic.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: italic;
  font-weight: 800;
  src: url("/fonts/FiraGO-ExtraBoldItalic.woff2") format("woff2");
}
@font-face {
  font-family: FiraGO;
  font-style: italic;
  font-weight: 900;
  src: url("/fonts/FiraGO-HeavyItalic.woff2") format("woff2");
}

@theme {
  --font-sans: FiraGO;
}
</style>
