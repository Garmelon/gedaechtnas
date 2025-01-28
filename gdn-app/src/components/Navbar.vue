<script setup lang="ts">
import { RiSettings3Fill } from "@remixicon/vue";
import NavbarDropdown from "./NavbarDropdown.vue";
</script>

<template>
  <div class="flex justify-between bg-black p-2 text-white">
    <NavbarDropdown />
    <div
      class="flex items-center rounded-md bg-neutral-800 px-1 hover:bg-neutral-700 active:bg-neutral-500"
    >
      <RiSettings3Fill class="h-4" />
    </div>
  </div>
</template>
