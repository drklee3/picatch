import { useState, useEffect } from "react";

function useResize(ref: React.RefObject<HTMLElement>) {
    const [dimensions, setDimensions] = useState({ width: 0, height: 0 });

    useEffect(() => {
        const getDimensions = () => ({
            width: ref.current?.offsetWidth || 0,
            height: ref.current?.offsetHeight || 0,
        });

        const handleResize = () => {
            setDimensions(getDimensions());
        };

        if (ref.current) {
            handleResize();
        }

        window.addEventListener("resize", handleResize);

        return () => {
            window.removeEventListener("resize", handleResize);
        };
    }, [ref]);

    return dimensions;
}

export default useResize;
