import React, { useState } from "react";

type Props = React.ImgHTMLAttributes<HTMLImageElement>;

function ImageItem(props: Props) {
    const [loaded, setLoaded] = useState(false);

    return (
        <img
            {...props}
            className={`${props.className || ""} ${loaded ? "visible" : ""}`}
            onLoad={() => setLoaded(true)}
        />
    );
}

export default ImageItem;
