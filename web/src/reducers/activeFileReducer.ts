export type ActiveFileState = {
    name: string;
    index: number;
};

export enum ActiveFileActions {
    SET_FILE = "SET_FILE",
    SET_NAME = "SET_NAME",
    SET_INDEX = "SET_INDEX",
    INCREMENT_INDEX = "INCREMENT_INDEX",
    DECREMENT_INDEX = "DECREMENT_INDEX",
}

export interface SetActiveFileAction {
    type: ActiveFileActions.SET_FILE;
    name: string;
    index: number;
}

export interface SetNameAction {
    type: ActiveFileActions.SET_NAME;
    name: string;
}

export interface SetIndexAction {
    type: ActiveFileActions.SET_INDEX;
    index: number;
}

export interface IncrementIndexAction {
    type: ActiveFileActions.INCREMENT_INDEX;
}

export interface DecrementIndexAction {
    type: ActiveFileActions.DECREMENT_INDEX;
}

export type ActiveFileActionTypes =
    | SetActiveFileAction
    | SetNameAction
    | SetIndexAction
    | IncrementIndexAction
    | DecrementIndexAction;

function activeFileReducer(
    state: ActiveFileState,
    action: ActiveFileActionTypes
): ActiveFileState {
    switch (action.type) {
        case ActiveFileActions.INCREMENT_INDEX:
            return { ...state, index: state.index + 1 };
        case ActiveFileActions.DECREMENT_INDEX:
            return { ...state, index: Math.max(state.index - 1, 0) };
        case ActiveFileActions.SET_FILE:
            return { name: action.name, index: action.index };
        case ActiveFileActions.SET_NAME:
            return { ...state, name: action.name };
        case ActiveFileActions.SET_INDEX:
            return { ...state, index: action.index };
        default:
            throw new Error();
    }
}

export default activeFileReducer;
