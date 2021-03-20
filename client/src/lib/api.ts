const BASE_URL = `${location.protocol}//${location.hostname}:3030`;

type HTTPMethod =
  | 'CONNECT'
  | 'DELETE'
  | 'GET'
  | 'HEAD'
  | 'OPTIONS'
  | 'PATCH'
  | 'POST'
  | 'PUT'
  | 'TRACE';

export const url = {
  BASE_URL,
  question: BASE_URL + '/question',
  questionTag: BASE_URL + '/question/tag',
  tag: BASE_URL + '/tag',
};

const request = async (
  url: string,
  params: { method?: HTTPMethod; body?: Object } = {}
) =>
  fetch(url, {
    method: params.method,
    body: JSON.stringify(params.body),
    headers: params.body && {
      'Content-Type': 'application/json',
    },
  });

const requestJson = (
  url: string,
  params: { method?: HTTPMethod; body?: Object } = {}
) => request(url, params).then((x) => x.json());

export const api = {
  question: {
    add(body: NewQuestion): Promise<IdObj> {
      return requestJson(url.question, { method: 'POST', body });
    },
    edit(edit: QuestionEdit) {
      return request(url.question, { method: 'PATCH', body: edit });
    },
    remove(id: IdObj) {
      return request(url.question, { method: 'DELETE', body: id });
    },
    get({ tags, limit, offset }: QuestionQuery): Promise<Question[]> {
      const params = new URLSearchParams({
        limit: limit.toString(),
        offset: offset.toString(),
      });
      for (const t of tags) {
        params.append('tags', t.toString());
      }
      return requestJson(url.question + '?' + params.toString());
    },
    tag: {
      add(tag: QuestionTag) {
        return request(url.questionTag, {
          method: 'POST',
          body: tag,
        });
      },
      remove(tag: QuestionTag) {
        return request(url.questionTag, {
          method: 'DELETE',
          body: tag,
        });
      },
    },
  },
  tag: {
    add(name: NewTag): Promise<IdObj> {
      return requestJson(url.tag, { method: 'POST', body: name });
    },
    get(): Promise<Tag[]> {
      return requestJson(url.tag);
    },
    remove(id: IdObj) {
      return request(url.tag, { method: 'DELETE', body: id });
    },
    edit(tag: TagEdit) {
      return request(url.tag, { method: 'PATCH', body: tag });
    },
  },
};

export type Question = {
  id: number;
  body: string;
  created_at: string;
  modified_at: string;
  tags: number[];
};

export type QuestionQuery = {
  tags: number[];
  limit: number;
  offset: number;
};

export type NewQuestion = {
  body: string;
  tags?: number[];
};

export type QuestionEdit = {
  id: number;
  body: string;
};

export type Tag = {
  id: number;
  name: string;
  color: number | null;
};

export type IdObj = {
  id: number;
};

export type NewTag = {
  name: string;
  color?: number;
};

export type TagEdit = {
  id: number;
  name?: string;
  color?: number;
};

export type QuestionTag = {
  question_id: number;
  tag_id: number;
};
