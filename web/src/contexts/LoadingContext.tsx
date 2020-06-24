import { createContext } from "react";

export const LoadingContext = createContext({
    isLoading: false,
    setIsLoading: (loading: boolean) => {},
});
