import { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";
import { getPathComponents } from "../util";

function usePathComponents() {
    const location = useLocation();
    const [path, setPath] = useState(getPathComponents(location.pathname));

    // Update path when location.pathname changes
    useEffect(() => {
        setPath(getPathComponents(location.pathname));
    }, [location]);

    return path;
}

export default usePathComponents;
