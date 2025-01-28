<script setup lang="ts">
import { useReposStore } from "@/stores/repos";
import { RiDeleteBinFill, RiSettings3Fill } from "@remixicon/vue";
import CNavbarDropdown from "./CNavbarDropdown.vue";

const repos = useReposStore();
</script>

<template>
  <div class="flex gap-1 bg-black p-2 text-white">
    <!-- The div is necessary because of the delete button, otherwise I could
    just use justify-between -->
    <div class="mr-auto overflow-hidden">
      <CNavbarDropdown />
    </div>

    <!-- Temporary delete button until I add proper repo settings -->
    <div
      v-show="repos.selectedRepo !== undefined"
      class="flex select-none items-center rounded-md bg-neutral-800 px-2 hover:bg-neutral-700 active:bg-neutral-500"
      @click="repos.removeRepo(repos.selectedRepoId)"
    >
      <RiDeleteBinFill size="16px" class="inline" />
    </div>

    <!-- Nothing hooked up yet -->
    <div
      class="flex select-none items-center rounded-md bg-neutral-800 px-2 hover:bg-neutral-700 active:bg-neutral-500"
    >
      <RiSettings3Fill size="16px" class="inline" />
    </div>
  </div>
</template>
