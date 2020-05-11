import { useState, useEffect } from "react";
import useKeypress from "./useKeypress";

function useKeyboardNavigation() {
    const rightKeyPressed = useKeypress("ArrowRight");
    const leftKeyPressed = useKeypress("ArrowLeft");
    const escapeKeyPressed = useKeypress("Escape");

    const [activeKeys, setActiveKeys] = useState(new Set<string>());
    const [keypressed, setKeypressed] = useState("");

    useEffect(() => {
        const keys = [rightKeyPressed, leftKeyPressed, escapeKeyPressed];
        const newestKey = keys.find(
            (k) => k.keyPressed && !activeKeys.has(k.targetKey)
        );

        if (newestKey) {
            setKeypressed(newestKey.targetKey);
        } else {
            setKeypressed("");
        }

        // Update key set
        keys.forEach((key) => {
            if (!key.keyPressed) {
                // Remove if not active anymore
                activeKeys.delete(key.targetKey);
            } else if (!activeKeys.has(key.targetKey)) {
                // Add if newly active
                activeKeys.add(key.targetKey);
            }
        });
        setActiveKeys(activeKeys);
    }, [activeKeys, rightKeyPressed, leftKeyPressed, escapeKeyPressed]);

    return keypressed;
}

export default useKeyboardNavigation;
