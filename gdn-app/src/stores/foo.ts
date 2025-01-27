import { defineStore } from "pinia";
import { ref } from "vue";

export const useFooStore = defineStore("foo", () => {
  const name = ref("foo");

  function setName(to: string) {
    name.value = to;
  }

  return { name, setName };
});
