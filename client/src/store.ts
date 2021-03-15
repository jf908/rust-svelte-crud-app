import { derived, writable } from 'svelte/store';
import { api, Question, Tag, url } from './lib/api';

function createQuestionStore() {
  const store = writable<Question[] | null>(null);

  return {
    ...store,
    async addQuestion(body: string, tags?: number[]) {
      const { id } = await api.question.add({ body, tags });
      store.update(
        (qs) =>
          qs && [
            ...qs,
            {
              id,
              body,
              created_at: new Date().toISOString(),
              modified_at: new Date().toISOString(),
              tags: tags || [],
            },
          ]
      );
    },
    async deleteQuestion(id: number) {
      await api.question.remove({ id });
      store.update((qs) => qs && qs.filter((x) => x.id !== id));
    },
    async editQuestion(id: number, body: string) {
      await api.question.edit({ id, body });
      // store.update((qs) => qs && qs.filter((x) => x.id === id));
    },
    async refresh() {
      let qs = await api.question.get();
      store.set(qs);
    },
  };
}

export const questions = createQuestionStore();

function createTagsStore() {
  const store = writable<Tag[] | null>(null);

  return {
    ...store,
    async addTag(name: string) {
      const { id } = await api.tag.add({ body: name });
      store.update(
        (ts) =>
          ts && [
            ...ts,
            {
              id,
              name,
              created_at: new Date().toISOString(),
              modified_at: new Date().toISOString(),
            },
          ]
      );
    },
    async deleteTag(id: number) {
      await api.tag.remove({ id });
      store.update((ts) => ts && ts.filter((x) => x.id !== id));
    },
    async refresh() {
      let ts = await api.tag.get();
      store.set(ts);
    },
    // async editQuestion(id: number, body: string) {
    //   await api.question.edit({ id, body });
    //   // store.update((qs) => qs && qs.filter((x) => x.id === id));
    // },
  };
}

export const tags = createTagsStore();

export const tagsMap = derived(tags, (tags): { [k: string]: Tag } =>
  tags ? tags.reduce((acc, x) => ({ ...acc, [x.id]: x }), {}) : {}
);

export function reconnect() {
  return Promise.all([questions.refresh(), tags.refresh()]);
}

export type ConfirmModal = {
  body: string;
  confirm: () => void;
};
export const confirmModal = writable<ConfirmModal | null>(null);

export const tagEditor = writable(false);
