import { useState, useEffect } from "react";

function useKeypress(targetKey: string) {
    const [keyPressed, setKeyPressed] = useState(false);

    useEffect(() => {
        console.log("adding event listner");
        const downHandler = ({ key }: KeyboardEvent) => {
            if (key === targetKey) {
                setKeyPressed(true);
            }
        };

        const upHandler = ({ key }: KeyboardEvent) => {
            if (key === targetKey) {
                setKeyPressed(false);
            }
        };

        window.addEventListener("keydown", downHandler);
        window.addEventListener("keyup", upHandler);

        return () => {
            window.removeEventListener("keydown", downHandler);
            window.removeEventListener("keyup", upHandler);
        };
    }, [targetKey]);

    return { keyPressed, targetKey };
}

export default useKeypress;
