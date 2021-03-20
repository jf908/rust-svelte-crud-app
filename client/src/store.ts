import { derived, writable } from 'svelte/store';
import { api, Question, QuestionQuery, Tag, url } from './lib/api';

export const moreQuestions = writable(false);

function createQuestionStore() {
  const store = writable<Question[] | null>(null);

  let queryLimit = 100;
  let previousQuery: QuestionQuery | null = null;

  return {
    ...store,
    async addQuestion(body: string, tags?: number[]) {
      const { id } = await api.question.add({ body, tags });
      store.update(
        (qs) =>
          qs && [
            {
              id,
              body,
              created_at: new Date().toISOString(),
              modified_at: new Date().toISOString(),
              tags: tags || [],
            },
            ...qs,
          ]
      );
      if (previousQuery) {
        previousQuery.offset++;
      }
    },
    async deleteQuestion(id: number) {
      await api.question.remove({ id });
      store.update((qs) => qs && qs.filter((x) => x.id !== id));

      if (previousQuery) {
        previousQuery.offset--;
      }
    },
    async editQuestion(id: number, body: string) {
      await api.question.edit({ id, body });
    },
    async refresh(tags: number[] = []) {
      previousQuery = { tags, limit: queryLimit, offset: 0 };
      let qs = await api.question.get(previousQuery);
      store.set(qs);
      moreQuestions.set(qs.length === queryLimit);
    },
    async getMore() {
      if (previousQuery) {
        previousQuery.offset += queryLimit;
        let qs = await api.question.get(previousQuery);
        store.update(
          (prevQuestions) => prevQuestions && [...prevQuestions, ...qs]
        );
        moreQuestions.set(qs.length === queryLimit);
      }
    },
    async addTag(question_id: number, tag_id: number) {
      await api.question.tag.add({ question_id, tag_id });
    },
    async removeTag(question_id: number, tag_id: number) {
      await api.question.tag.remove({ question_id, tag_id });
    },
  };
}

export const questions = createQuestionStore();

function createTagsStore() {
  const store = writable<Tag[] | null>(null);

  return {
    ...store,
    async addTag(name: string) {
      const { id } = await api.tag.add({ name });
      store.update(
        (ts) =>
          ts &&
          [
            ...ts,
            {
              id,
              name,
              color: null,
              created_at: new Date().toISOString(),
              modified_at: new Date().toISOString(),
            },
          ].sort((a, b) => a.name.localeCompare(b.name))
      );
    },
    async editTag(
      id: number,
      { name, color }: { name?: string; color?: number }
    ) {
      await api.tag.edit({ id, name, color });
    },
    async deleteTag(id: number) {
      await api.tag.remove({ id });
      store.update((ts) => ts && ts.filter((x) => x.id !== id));
    },
    async refresh() {
      let ts = await api.tag.get();
      store.set(ts);
    },
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

export const modalStack = writable([]);
