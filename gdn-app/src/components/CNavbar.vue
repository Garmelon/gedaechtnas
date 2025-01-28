<script setup lang="ts">
import { useReposStore } from "@/stores/repos";
import { RiDeleteBinFill, RiSettings3Fill } from "@remixicon/vue";
import CNavbarButton from "./CNavbarButton.vue";
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
    <CNavbarButton
      v-show="repos.selectedRepo !== undefined"
      @click="repos.removeRepo(repos.selectedRepoId)"
    >
      <RiDeleteBinFill size="16px" class="inline" />
    </CNavbarButton>

    <!-- Nothing hooked up yet -->
    <CNavbarButton>
      <RiSettings3Fill size="16px" class="inline" />
    </CNavbarButton>
  </div>
</template>
