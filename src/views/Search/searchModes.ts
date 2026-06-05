import { SEARCH_MODEL } from "@/constant";

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
