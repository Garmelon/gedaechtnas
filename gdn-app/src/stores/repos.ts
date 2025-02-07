import { defineStore } from "pinia";
import { computed, ref } from "vue";

type Repo = {
  id: string;
  name: string;
};

export const useReposStore = defineStore("repos", () => {
  const repos = ref<Map<string, Repo>>(new Map());
  const selectedRepoId = ref<string>();

  const selectedRepo = computed<Repo | undefined>(() => {
    if (selectedRepoId.value === undefined) return undefined;
    return repos.value.get(selectedRepoId.value);
  });

  const reposByName = computed<Repo[]>(() => {
    const values = [...repos.value.values()];
    values.sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0));
    return values;
  });

  const repoIdsByName = computed<string[]>(() => reposByName.value.map((it) => it.id));

  function addRepo(repo: Repo) {
    repos.value.set(repo.id, repo);
    selectedRepoId.value = repo.id;
  }

  function removeRepo(id: string | undefined) {
    if (id === undefined) return;
    const i = repoIdsByName.value.indexOf(id);
    repos.value.delete(id);
    if (i >= 0) {
      const j = Math.min(i, reposByName.value.length - 1);
      selectRepo(repoIdsByName.value[j]);
    }
  }

  function selectRepo(id: string | undefined) {
    if (id === undefined) return;
    if (repos.value.get(id) === undefined) return;
    selectedRepoId.value = id;
  }

  return {
    repos,
    reposByName,
    selectedRepo,
    selectedRepoId,
    addRepo,
    removeRepo,
    selectRepo,
  };
});
