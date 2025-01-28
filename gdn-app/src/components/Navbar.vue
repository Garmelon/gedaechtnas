<script setup lang="ts">
import { RiSettings3Fill } from "@remixicon/vue";
import NavbarDropdown from "./NavbarDropdown.vue";
</script>

<template>
  <div class="flex justify-between gap-1 bg-black p-2 text-white">
    <NavbarDropdown />
    <div
      class="flex items-center rounded-md bg-neutral-800 px-2 hover:bg-neutral-700 active:bg-neutral-500"
    >
      <RiSettings3Fill size="16px" class="inline" />
    </div>
  </div>
</template>
