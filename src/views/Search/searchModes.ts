export const SEARCH_MODEL = {
  DEFAULT_MODEL: 0,
  WEB_SEARCH_MODEL: 1,
  TRANSLATION_MODEL: 2,
  TODO_MODEL: 3,
} as const;

export type SearchModelType = (typeof SEARCH_MODEL)[keyof typeof SEARCH_MODEL];

export const SEARCH_MODE_TABS_HEIGHT = 44;

export interface SearchModeExpose {
  focus?: () => void;
  handleKeydown?: (event: KeyboardEvent) => void;
  handleClose?: () => void;
  handleBeforeShow?: () => void;
  getDefaultHeight?: () => number;
}

export type FormType = "" | "search";
export interface SwitchModePayload {
  mode: SearchModelType;
  keyword?: string;
  source?: WebSearchSource;
  from?: FormType;
}
