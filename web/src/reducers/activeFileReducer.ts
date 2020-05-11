import { ActiveFileActions, ActiveFileActionTypes } from "./activeFileActions";

export type ActiveFileState = {
    album: string;
    albumSize: number;
    name: string;
    index: number;
};

function activeFileReducer(
    state: ActiveFileState,
    action: ActiveFileActionTypes
): ActiveFileState {
    switch (action.type) {
        case ActiveFileActions.INCREMENT_INDEX:
            // Only increment if state isn't -1
            if (state.index === -1) {
                return state;
            }

            // Don't increment if last item
            if (state.index === state.albumSize - 1) {
                return state;
            }

            return {
                ...state,
                index: state.index + 1,
            };
        case ActiveFileActions.DECREMENT_INDEX:
            // Only decrement if state isn't -1 or 0 (<= 0)
            if (state.index <= 0) {
                return state;
            }

            return {
                ...state,
                index: state.index - 1,
            };
        case ActiveFileActions.SET_FILE:
            return { ...state, name: action.name, index: action.index };
        case ActiveFileActions.SET_NAME:
            return { ...state, name: action.name };
        case ActiveFileActions.SET_ALBUM:
            return { ...state, album: action.album };
        case ActiveFileActions.SET_ALBUM_SIZE:
            // Ignore if same size
            if (action.albumSize === state.albumSize) {
                return state;
            }

            return { ...state, albumSize: action.albumSize };
        case ActiveFileActions.SET_INDEX:
            return { ...state, index: action.index };
        default:
            throw new Error();
    }
}

export default activeFileReducer;
