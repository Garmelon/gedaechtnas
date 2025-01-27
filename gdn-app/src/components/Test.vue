<script setup lang="ts">
import { useFooStore } from "@/stores/foo";

const foo = useFooStore();
</script>

<template>
  Foo is {{ foo.name }}.
  <button @click="foo.setName('bar')">Change to bar.</button>
</template>
