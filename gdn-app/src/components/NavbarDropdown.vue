<script setup lang="ts">
import { useReposStore } from "@/stores/repos";
import { offset, useFloating } from "@floating-ui/vue";
import {
  RiAddLine,
  RiArrowDropDownLine,
  RiArrowDropUpLine,
} from "@remixicon/vue";
import { ref, useTemplateRef } from "vue";
import NavbarDropdownEntry from "./NavbarDropdownEntry.vue";

const repos = useReposStore();

const open = ref(false);

// https://floating-ui.com/docs/vue
const reference = useTemplateRef("reference");
const floating = useTemplateRef("floating");
const { floatingStyles } = useFloating(reference, floating, {
  placement: "bottom-start",
  middleware: [offset(4)],
});

function onAddNewRepo() {
  repos.addRepo({ id: "test", name: "test" });
  console.log(repos.selectedRepo);
  open.value = false;
}
</script>

<template>
  <!-- Navbar entry -->
  <div
    ref="reference"
    class="relative flex flex-shrink cursor-default overflow-hidden whitespace-nowrap rounded-md bg-neutral-800 pl-2 text-lg font-light hover:bg-neutral-700 active:bg-neutral-500"
    @click="open = !open"
  >
    <span v-if="repos.selectedRepo">{{ repos.selectedRepo.name }}</span>
    <span v-else class="overflow-hidden overflow-ellipsis italic">
      no repo selected
    </span>

    <div>
      <RiArrowDropUpLine v-if="open" class="inline" />
      <RiArrowDropDownLine v-else class="inline" />
    </div>
  </div>

  <!-- Close dropdown when clicking outside it -->
  <div
    v-if="open"
    class="fixed left-0 top-0 h-screen w-screen"
    @click="open = false"
  ></div>

  <!-- Dropdown -->
  <div
    v-if="open"
    ref="floating"
    class="absolute left-0 top-0 w-fit min-w-48 rounded-md bg-neutral-800 font-light"
    :style="floatingStyles"
  >
    <NavbarDropdownEntry
      v-for="repo of repos.reposByName"
      :class="{ 'font-medium': repo.id === repos.selectedRepoId }"
      >{{ repo.name }}</NavbarDropdownEntry
    >
    <hr v-if="repos.reposByName.length > 0" class="m-1 text-neutral-700" />
    <NavbarDropdownEntry class="italic" @click="onAddNewRepo">
      <RiAddLine size="16px" class="-ml-1 inline" /> add new repo
    </NavbarDropdownEntry>
  </div>
</template>
