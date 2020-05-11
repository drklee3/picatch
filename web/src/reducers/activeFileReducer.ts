import { ActiveFileActions, ActiveFileActionTypes } from "./activeFileActions";

export type ActiveFileState = {
    name: string;
    index: number;
};

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
