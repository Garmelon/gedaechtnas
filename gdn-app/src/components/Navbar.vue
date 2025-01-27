<script setup lang="ts">
import { RiArrowDropDownLine, RiSettings3Fill } from "@remixicon/vue";
</script>

<template>
  <div class="flex justify-between bg-black p-2 text-white">
    <div
      class="cursor-default rounded-md bg-neutral-800 pl-2 text-lg hover:bg-neutral-700 active:bg-neutral-500"
    >
      GedächtNAS
      <RiArrowDropDownLine class="inline" />
    </div>
    <div
      class="flex items-center rounded-md bg-neutral-800 px-1 hover:bg-neutral-700 active:bg-neutral-500"
    >
      <RiSettings3Fill class="h-4" />
    </div>
  </div>
</template>
