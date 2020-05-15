import React, { useState } from "react";

type Props = React.ImgHTMLAttributes<HTMLImageElement>;

function ImageItem(props: Props) {
    const [loaded, setLoaded] = useState(false);

    return (
        <img
            {...props}
            alt={props.alt ? props.alt : "Image"}
            className={`${props.className || ""} ${loaded ? "visible" : ""}`}
            onLoad={() => setLoaded(true)}
        />
    );
}

export default ImageItem;
